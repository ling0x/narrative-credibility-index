use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use crate::rubric::CATEGORIES;
use crate::score::CategoryScore;

pub fn run_manual_tui(document_preview: &str) -> Result<Vec<CategoryScore>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, document_preview);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

struct AppState {
    scores: Vec<u8>,
    selected: usize,
    doc_preview: String,
}

impl AppState {
    fn new(doc_preview: &str) -> Self {
        Self {
            scores: vec![1u8; CATEGORIES.len()],
            selected: 0,
            doc_preview: doc_preview.lines().take(40).collect::<Vec<_>>().join("\n"),
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    doc_preview: &str,
) -> Result<Vec<CategoryScore>> {
    let mut state = AppState::new(doc_preview);
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| render(f, &state, &mut list_state))?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), _)
                | (KeyCode::Esc, _)
                | (KeyCode::Enter, _)
                | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    return Ok(build_scores(&state));
                }
                (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
                    if state.selected + 1 < CATEGORIES.len() {
                        state.selected += 1;
                        list_state.select(Some(state.selected));
                    }
                }
                (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
                    if state.selected > 0 {
                        state.selected -= 1;
                        list_state.select(Some(state.selected));
                    }
                }
                (KeyCode::Char(c), _) if ('1'..='5').contains(&c) => {
                    state.scores[state.selected] = c as u8 - b'0';
                }
                (KeyCode::Char('+') | KeyCode::Right, _) => {
                    let v = &mut state.scores[state.selected];
                    if *v < 5 { *v += 1; }
                }
                (KeyCode::Char('-') | KeyCode::Left, _) => {
                    let v = &mut state.scores[state.selected];
                    if *v > 1 { *v -= 1; }
                }
                _ => {}
            }
        }
    }
}

fn render(f: &mut Frame, state: &AppState, list_state: &mut ListState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.area());

    render_left(f, state, list_state, chunks[0]);
    render_right(f, state, chunks[1]);
}

fn render_left(f: &mut Frame, state: &AppState, list_state: &mut ListState, area: Rect) {
    // Reserve bottom 3 rows for gauge
    let inner_area = Rect { height: area.height.saturating_sub(3), ..area };
    let gauge_area = Rect {
        y: area.y + area.height.saturating_sub(3),
        height: 3,
        ..area
    };

    let items: Vec<ListItem> = CATEGORIES
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let score = state.scores[i];
            let bar = "█".repeat(score as usize) + &"░".repeat(5 - score as usize);
            let selected = i == state.selected;
            let base_style = if selected {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let score_style = Style::default().fg(score_color(score)).bg(if selected { Color::Cyan } else { Color::Reset });
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {:>2}. {:<24}", cat.id, cat.name), base_style),
                Span::styled(format!(" [{}] {}", score, bar), score_style),
            ]))
        })
        .collect();

    let total: u32 = state.scores.iter().map(|&s| s as u32).sum();

    f.render_stateful_widget(
        List::new(items)
            .block(Block::default()
                .title(format!(" NCI Manual Scoring  [Total: {}/100] ", total))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
            )
            .highlight_style(
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            ),
        inner_area,
        list_state,
    );

    f.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL))
            .gauge_style(Style::default().fg(gauge_color(total)))
            .ratio((total as f64 / 100.0).min(1.0))
            .label(format!("{}/100 — {}", total, interpret(total))),
        gauge_area,
    );
}

fn render_right(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    // Category detail
    let cat = &CATEGORIES[state.selected];
    f.render_widget(
        Paragraph::new(format!(
            "Category {}  —  {}\n\nQuestion:\n{}\n\nExample:\n{}",
            cat.id, cat.name, cat.question, cat.example
        ))
        .block(Block::default()
            .title(" Category Detail ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left),
        chunks[0],
    );

    // Document preview
    let preview = if state.doc_preview.is_empty() {
        "(No document loaded — use: nci manual <file.md>)".to_string()
    } else {
        state.doc_preview.clone()
    };
    f.render_widget(
        Paragraph::new(preview)
            .block(Block::default()
                .title(" Document Preview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
            )
            .wrap(Wrap { trim: true }),
        chunks[1],
    );

    // Keybindings strip at bottom of right panel
    let hint_area = Rect {
        x: area.x, y: area.y + area.height.saturating_sub(1),
        width: area.width, height: 1,
    };
    f.render_widget(
        Paragraph::new("  ↑↓/jk Navigate  1-5 Set score  +/- Adjust  Enter/q Done")
            .style(Style::default().fg(Color::DarkGray)),
        hint_area,
    );
}

fn build_scores(state: &AppState) -> Vec<CategoryScore> {
    CATEGORIES.iter().enumerate().map(|(i, cat)| CategoryScore {
        id: cat.id,
        name: cat.name.to_string(),
        score: state.scores[i],
        reasoning: String::new(),
    }).collect()
}

fn score_color(score: u8) -> Color {
    match score { 1 => Color::Green, 2 => Color::LightGreen, 3 => Color::Yellow, 4 => Color::LightRed, _ => Color::Red }
}

fn gauge_color(total: u32) -> Color {
    match total { 0..=25 => Color::Green, 26..=50 => Color::Yellow, 51..=75 => Color::LightRed, _ => Color::Red }
}

fn interpret(total: u32) -> &'static str {
    match total {
        0..=25  => "Low likelihood",
        26..=50 => "Moderate — look deeper",
        51..=75 => "Strong — manipulation likely",
        _       => "Overwhelming PSYOP signs",
    }
}
