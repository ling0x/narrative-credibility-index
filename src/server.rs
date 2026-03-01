use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;

use crate::ai_agent;
use crate::score::CategoryScore;

#[derive(Clone)]
pub struct ServerState {
    pub api_url: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ScanResponse {
    pub total_score: u32,
    pub interpretation: String,
    pub saved_file: String,
    pub scores: Vec<CategoryScore>,
}

pub async fn start(port: u16, state: ServerState) -> anyhow::Result<()> {
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/scan", post(handle_scan))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("🚀 API Server listening on http://0.0.0.0:{}", port);
    println!("   Try testing with: curl -X POST http://127.0.0.1:{}/scan -H 'Content-Type: application/json' -d '{{\"url\": \"https://example.com\"}}'", port);
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_scan(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<ScanRequest>,
) -> Result<Json<ScanResponse>, (StatusCode, String)> {
    println!("📡 Received request to scan: {}", payload.url);

    // 1. Fetch HTML
    let response = reqwest::get(&payload.url)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to fetch URL: {}", e)))?;
        
    let html = response
        .text()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read HTML: {}", e)))?;

    // 2. Convert to Markdown using html2md
    let markdown = html2md::parse_html(&html);

    // 3. Save to a temporary markdown file in a `scans/` directory
    let file_id = Uuid::new_v4().to_string();
    let file_name = format!("scanned_article_{}.md", file_id);
    
    let scans_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("scans");
        
    if !scans_dir.exists() {
        fs::create_dir_all(&scans_dir)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create scans directory: {}", e)))?;
    }
    
    let file_path = scans_dir.join(&file_name);
    fs::write(&file_path, &markdown)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save markdown file: {}", e)))?;
        
    println!("💾 Saved extracted markdown to: {}", file_path.display());
    println!("🤖 Analyzing content with AI...");

    // 4. Run the AI scoring
    let scores = ai_agent::score_document(&markdown, &state.api_url, &state.model, &state.api_key)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("AI scoring failed: {}", e)))?;

    // 5. Build Final Report
    let total_score: u32 = scores.iter().map(|s| s.score as u32).sum();
    let interpretation = match total_score {
        0..=25  => "Low likelihood of a PSYOP",
        26..=50 => "Moderate likelihood — look deeper",
        51..=75 => "Strong likelihood — manipulation likely",
        _       => "Overwhelming signs of a PSYOP",
    }.to_string();

    println!("✅ Analysis complete. Total Score: {}", total_score);

    Ok(Json(ScanResponse {
        total_score,
        interpretation,
        saved_file: file_path.to_string_lossy().into_owned(),
        scores,
    }))
}
