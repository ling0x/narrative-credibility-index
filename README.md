# Narrative Credibility Index CLI

A Rust CLI tool that scans a Markdown document and scores it against the **NCI Engineered Reality Scoring System** — a 20-category PSYOP identification rubric.

## Features

- 📄 **Auto-scan** — point at any `.md` file and get an AI-reasoned NCI score
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

### AI Agent scan
```bash
# Set your OpenAI-compatible API key
export OPENAI_API_KEY="sk-..."

# Scan a document
./target/release/nci scan article.md

# Use a custom base URL (e.g. Ollama, Together AI)
./target/release/nci scan article.md --api-url http://localhost:11434/v1 --model llama3
```

### Manual TUI input
```bash
./target/release/nci manual article.md
# Or without a file (blank slate)
./target/release/nci manual
```

### View rubric
```bash
./target/release/nci rubric
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|----------|
| `OPENAI_API_KEY` | API key | required for `scan` |
| `OPENAI_API_URL` | Base URL | `https://api.openai.com/v1` |
| `OPENAI_MODEL` | Model name | `gpt-4o` |

## Example Output

```
══════════════════════════════════════════════════════════
     NARRATIVE CREDIBILITY INDEX — SCORE REPORT
══════════════════════════════════════════════════════════
   1. Timing                          [4] ████░
   2. Emotional Manipulation          [3] ███░░
     ... (20 categories)
══════════════════════════════════════════════════════════
  TOTAL SCORE: 62 / 100
  ⚠  Strong likelihood — manipulation likely
══════════════════════════════════════════════════════════
```

---
*Based on NCI Engineered Reality Scoring System — Applied Behavior Research © 2024*
