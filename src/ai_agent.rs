use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use colored::Colorize;
use crate::rubric::CATEGORIES;
use crate::score::CategoryScore;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub async fn score_document(
    document: &str,
    api_url: &str,
    model: &str,
    api_key: &str,
    verbose: bool,
) -> Result<Vec<CategoryScore>> {
    let document = document.to_string();
    let api_url = api_url.to_string();
    let model = model.to_string();
    let api_key = api_key.to_string();

    tokio::task::spawn_blocking(move || {
        call_llm_blocking(&document, &api_url, &model, &api_key, verbose)
    })
    .await?
}

fn build_system_prompt(verbose: bool) -> String {
    if verbose {
        println!("\n{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("{}", "  STEP 1: Building System Prompt".cyan().bold());
        println!("{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("\n{}", "Loading NCI Engineered Reality Scoring categories...".blue());
    }

    let mut prompt = String::from(
        "You are an expert analyst applying the NCI Engineered Reality Scoring System. \
         Read the provided article and score it on 20 manipulation categories. \
         For each category output a JSON object with: id (int), score (int 1-5), reasoning (string max 30 words). \
         Return ONLY a JSON array of 20 objects — no markdown fences, no extra text.\n\n\
         Scoring: 1 = Not Present, 5 = Overwhelmingly Present.\n\nCategories:\n",
    );

    for cat in CATEGORIES {
        if verbose {
            println!("  {} {}: {}", "│".cyan(), format!("[{}]", cat.id).yellow(), cat.name.green());
        }
        prompt.push_str(&format!("{}. {} — {}\n", cat.id, cat.name, cat.question));
    }

    if verbose {
        println!("\n{}", "✓ System prompt built with 20 categories".green().bold());
    }

    prompt
}

fn call_llm_blocking(
    document: &str,
    api_url: &str,
    model: &str,
    api_key: &str,
    verbose: bool,
) -> Result<Vec<CategoryScore>> {
    if verbose {
        println!("\n{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("{}", "  STEP 2: Preparing Document for Analysis".cyan().bold());
        println!("{}", "═══════════════════════════════════════════════════════".cyan().bold());
        let preview_length = document.chars().take(200).count();
        let doc_preview: String = document.chars().take(200).collect();
        println!("\n{}", "Document preview (first 200 chars):".blue());
        println!("{}", format!("  │ {}{}...", doc_preview, if document.len() > preview_length { "" } else { "" }).white());
        println!("\n{}", format!("  Document length: {} characters", document.len()).yellow());
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    let url = format!("{}/chat/completions", api_url.trim_end_matches('/'));

    let body = ChatRequest {
        model: model.to_string(),
        temperature: 0.2,
        messages: vec![
            Message { role: "system".into(), content: build_system_prompt(verbose) },
            Message { role: "user".into(), content: format!("Article to analyse:\n\n{}", document) },
        ],
    };

    if verbose {
        println!("\n{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("{}", "  STEP 3: Sending Request to LLM".cyan().bold());
        println!("{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("\n{}", format!("  Model: {}", model).yellow());
        println!("{}", format!("  API URL: {}", url).yellow());
        println!("{}", format!("  Temperature: {}", body.temperature).yellow());
        println!("\n{}", "  Waiting for LLM response...".blue().bold());
    } else {
        println!("  → Calling {} at {}…", model, url);
    }

    let mut request = client.post(&url).json(&body);
    if !api_key.is_empty() {
        request = request.bearer_auth(api_key);
    }

    let resp = request.send().context("HTTP request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        anyhow::bail!("API error {}: {}", status, text);
    }

    let chat_resp: ChatResponse = resp.json().context("Failed to parse API response")?;
    let raw = chat_resp.choices.into_iter().next()
        .map(|c| c.message.content)
        .unwrap_or_default();

    if verbose {
        println!("\n{}", "✓ Received response from LLM".green().bold());
    }

    parse_llm_scores(&raw, verbose)
}

#[derive(Deserialize)]
struct RawScore {
    id: u8,
    score: u8,
    reasoning: String,
}

fn parse_llm_scores(raw: &str, verbose: bool) -> Result<Vec<CategoryScore>> {
    if verbose {
        println!("\n{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("{}", "  STEP 4: Parsing LLM Response".cyan().bold());
        println!("{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("\n{}", "Raw response:".blue());
        let preview: String = raw.chars().take(300).collect();
        println!("{}", format!("  │ {}{}...", preview, if raw.len() > 300 { "" } else { "" }).white().dimmed());
    }

    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    if verbose {
        println!("\n{}", "Parsing JSON response...".blue());
    }

    let raw_scores: Vec<RawScore> =
        serde_json::from_str(cleaned).context("Failed to parse LLM JSON output")?;

    if verbose {
        println!("\n{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("{}", "  STEP 5: Extracting Category Scores".cyan().bold());
        println!("{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("\n{}", format!("Found {} category scores:", raw_scores.len()).blue());
    }

    let scores: Vec<CategoryScore> = raw_scores
        .into_iter()
        .map(|r| {
            let category_name = CATEGORIES.iter().find(|c| c.id == r.id)
                .map(|c| c.name.to_string())
                .unwrap_or_else(|| format!("Category {}", r.id));
            
            let score = CategoryScore {
                id: r.id,
                name: category_name.clone(),
                score: r.score.clamp(1, 5),
                reasoning: r.reasoning.clone(),
            };

            if verbose {
                let score_color = match score.score {
                    1..=2 => "green",
                    3 => "yellow",
                    4..=5 => "red",
                    _ => "white",
                };
                let score_display = match score_color {
                    "green" => format!("{}/5", score.score).green(),
                    "yellow" => format!("{}/5", score.score).yellow(),
                    "red" => format!("{}/5", score.score).red(),
                    _ => format!("{}/5", score.score).white(),
                };
                println!("\n  {} {}", "│".cyan(), format!("[{}] {}", r.id, category_name).white().bold());
                println!("  {} Score: {}", "│".cyan(), score_display);
                println!("  {} Reasoning: {}", "│".cyan(), r.reasoning.white().dimmed());
            }

            score
        })
        .collect();

    if verbose {
        println!("\n{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!("{}", "✓ Analysis Complete".green().bold());
        println!("{}", "═══════════════════════════════════════════════════════".cyan().bold());
        println!();
    }

    Ok(scores)
}
