use colored::*;
use serde::{Deserialize, Serialize};
use crate::rubric::CATEGORIES;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScore {
    pub id: u8,
    pub name: String,
    pub score: u8,
    pub reasoning: String,
}

pub fn print_report(scores: &[CategoryScore]) {
    let total: u32 = scores.iter().map(|s| s.score as u32).sum();
    let (label, color_fn): (&str, fn(&str) -> ColoredString) = match total {
        0..=25  => ("Low likelihood of a PSYOP",              |s: &str| s.green()),
        26..=50 => ("Moderate likelihood — look deeper",       |s: &str| s.yellow()),
        51..=75 => ("Strong likelihood — manipulation likely", |s: &str| s.truecolor(255, 165, 0)),
        _       => ("Overwhelming signs of a PSYOP",           |s: &str| s.red().bold()),
    };

    let width = 58usize;
    println!("\n{}", "═".repeat(width).cyan());
    println!("{:^width$}", "NARRATIVE CREDIBILITY INDEX — SCORE REPORT".bold());
    println!("{}", "═".repeat(width).cyan());

    for s in scores {
        let bar: String = "█".repeat(s.score as usize) + &"░".repeat(5 - s.score as usize);
        println!(
            "  {:>2}. {:<26} [{}] {}",
            s.id,
            s.name,
            s.score.to_string().bold(),
            bar
        );
        if !s.reasoning.is_empty() {
            for line in wrap_text(&s.reasoning, 52) {
                println!("      {}", line.dimmed());
            }
        }
    }

    println!("{}", "─".repeat(width).cyan());
    println!("  TOTAL SCORE: {} / 100", format!("{total}").bold());
    println!("  {}", color_fn(label));
    println!("{}", "═".repeat(width).cyan());
    println!();
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > max_width && !current.is_empty() {
            lines.push(current.clone());
            current.clear();
        }
        if !current.is_empty() { current.push(' '); }
        current.push_str(word);
    }
    if !current.is_empty() { lines.push(current); }
    lines
}

#[allow(dead_code)]
pub fn empty_scores() -> Vec<CategoryScore> {
    CATEGORIES
        .iter()
        .map(|c| CategoryScore { id: c.id, name: c.name.to_string(), score: 1, reasoning: String::new() })
        .collect()
}
