use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
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
) -> Result<Vec<CategoryScore>> {
    let document = document.to_string();
    let api_url = api_url.to_string();
    let model = model.to_string();
    let api_key = api_key.to_string();

    tokio::task::spawn_blocking(move || {
        call_llm_blocking(&document, &api_url, &model, &api_key)
    })
    .await?
}

fn build_system_prompt() -> String {
    let mut prompt = String::from(
        "You are an expert analyst applying the NCI Engineered Reality Scoring System. \
         Read the provided article and score it on 20 manipulation categories. \
         For each category output a JSON object with: id (int), score (int 1-5), reasoning (string max 30 words). \
         Return ONLY a JSON array of 20 objects — no markdown fences, no extra text.\n\n\
         Scoring: 1 = Not Present, 5 = Overwhelmingly Present.\n\nCategories:\n",
    );
    for cat in CATEGORIES {
        prompt.push_str(&format!("{}. {} — {}\n", cat.id, cat.name, cat.question));
    }
    prompt
}

fn call_llm_blocking(
    document: &str,
    api_url: &str,
    model: &str,
    api_key: &str,
) -> Result<Vec<CategoryScore>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    let url = format!("{}/chat/completions", api_url.trim_end_matches('/'));

    let body = ChatRequest {
        model: model.to_string(),
        temperature: 0.2,
        messages: vec![
            Message { role: "system".into(), content: build_system_prompt() },
            Message { role: "user".into(), content: format!("Article to analyse:\n\n{}", document) },
        ],
    };

    println!("  → Calling {} at {}…", model, url);

    let resp = client
        .post(&url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .context("HTTP request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        anyhow::bail!("API error {}: {}", status, text);
    }

    let chat_resp: ChatResponse = resp.json().context("Failed to parse API response")?;
    let raw = chat_resp.choices.into_iter().next()
        .map(|c| c.message.content)
        .unwrap_or_default();

    parse_llm_scores(&raw)
}

#[derive(Deserialize)]
struct RawScore {
    id: u8,
    score: u8,
    reasoning: String,
}

fn parse_llm_scores(raw: &str) -> Result<Vec<CategoryScore>> {
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let raw_scores: Vec<RawScore> =
        serde_json::from_str(cleaned).context("Failed to parse LLM JSON output")?;

    Ok(raw_scores
        .into_iter()
        .map(|r| CategoryScore {
            id: r.id,
            name: CATEGORIES.iter().find(|c| c.id == r.id)
                .map(|c| c.name.to_string())
                .unwrap_or_else(|| format!("Category {}", r.id)),
            score: r.score.clamp(1, 5),
            reasoning: r.reasoning,
        })
        .collect())
}
