mod rubric;
mod score;
mod ai_agent;
mod tui;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "nci",
    about = "Narrative Credibility Index — PSYOP detection CLI",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// AI-agent scan of a Markdown document
    Scan {
        /// Path to the .md document
        file: PathBuf,
        /// OpenAI-compatible API base URL
        #[arg(long, env = "OPENAI_API_URL", default_value = "https://api.openai.com/v1")]
        api_url: String,
        /// Model name
        #[arg(long, env = "OPENAI_MODEL", default_value = "gpt-4o")]
        model: String,
        /// API key (falls back to OPENAI_API_KEY env var)
        #[arg(long, env = "OPENAI_API_KEY")]
        api_key: Option<String>,
    },
    /// Interactive TUI to manually score a document
    Manual {
        /// Optional path to the .md document (for context display)
        file: Option<PathBuf>,
    },
    /// Print the full NCI rubric
    Rubric,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { file, api_url, model, api_key } => {
            // If the URL is the default OpenAI one, we require a key. 
            // If it's a custom one (like local Ollama), we default to a dummy key "sk-no-key" 
            // to satisfy reqwest's bearer auth if it's called, though local servers usually ignore it.
            let is_default_openai = api_url.contains("api.openai.com");
            
            let key = api_key
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .unwrap_or_else(|| {
                    if is_default_openai {
                        eprintln!("Error: OPENAI_API_KEY not set. Use --api-key or set the env var.");
                        std::process::exit(1);
                    } else {
                        "sk-no-key".to_string()
                    }
                });

            let content = std::fs::read_to_string(&file)?;
            println!("\n🔍 Scanning: {}\n", file.display());

            let scores = ai_agent::score_document(&content, &api_url, &model, &key).await?;
            score::print_report(&scores);
        }
        Commands::Manual { file } => {
            let preview = file
                .as_ref()
                .and_then(|p| std::fs::read_to_string(p).ok())
                .unwrap_or_default();
            let scores = tui::run_manual_tui(&preview)?;
            score::print_report(&scores);
        }
        Commands::Rubric => {
            rubric::print_rubric();
        }
    }

    Ok(())
}
