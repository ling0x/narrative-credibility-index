# Narrative Credibility Index CLI

A Rust CLI tool that scans a Markdown document and scores it against the **NCI Engineered Reality Scoring System** — a 20-category PSYOP identification rubric.

## Features

- 📄 **Auto-scan** — point at any `.md` file and get an AI-reasoned NCI score
- 🌐 **Web API** — launch a REST API to submit news URLs, automatically extract their content, save them locally, and scan them
- 🤖 **AI Agent mode** — uses an OpenAI-compatible API to reason through all 20 categories
- 🖥️ **Interactive TUI** — manually input scores for each category using a Ratatui-powered terminal UI
- 📊 **Score report** — colour-coded result with interpretation band

## Scoring Rubric

| Score Range | Interpretation |
|-------------|----------------|
| 0 – 25      | Low likelihood of a PSYOP |
| 26 – 50     | Moderate likelihood — look deeper |
| 51 – 75     | Strong likelihood — manipulation likely |
| 76 – 100    | Overwhelming signs of a PSYOP |

Each of the 20 categories is scored **1 (Not Present) → 5 (Overwhelmingly Present)**.

## Installation

```bash
git clone https://github.com/ling0x/narrative-credibility-index
cd narrative-credibility-index
cargo build --release
```

## Usage

### 1. AI Agent scan (CLI)
```bash
# Scan a local document
./target/release/nci scan article.md

# Use a custom base URL (e.g. Ollama, Together AI)
./target/release/nci scan article.md --api-url http://localhost:11434/v1 --model llama3
```

### 2. REST API Server (Automated Web Scraping & Scoring)
You can run an Axum-powered web server to submit live URLs:

```bash
# Start the server
./target/release/nci serve --port 3000 --api-url http://localhost:11434/v1 --model llama3
```

In another terminal, send a POST request with the URL of the article you want to scan:
```bash
curl -X POST http://127.0.0.1:3000/scan \
     -H "Content-Type: application/json" \
     -d '{"url": "https://example.com/news/some-article"}'
```
*The server will fetch the webpage, convert the HTML to a clean Markdown file, save it into a `scans/` folder, analyse it with the AI, and return the complete NCI JSON breakdown.*

### 3. Manual TUI input
```bash
./target/release/nci manual article.md
```

### 4. View rubric
```bash
./target/release/nci rubric
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|----------|
| `OPENAI_API_KEY` | API key | required for standard OpenAI endpoints |
| `OPENAI_API_URL` | Base URL | `https://api.openai.com/v1` |
| `OPENAI_MODEL` | Model name | `gpt-4o` |
