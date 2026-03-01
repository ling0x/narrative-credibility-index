mod rubric;
mod score;
mod ai_agent;
mod tui;
mod server;

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
    /// Start a REST API server to scrape URLs and scan them
    Serve {
        /// Port to listen on
        #[arg(long, short, default_value = "3000")]
        port: u16,
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
        Commands::Serve { port, api_url, model, api_key } => {
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

            server::start(port, server::ServerState { api_url, model, api_key: key }).await?;
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
