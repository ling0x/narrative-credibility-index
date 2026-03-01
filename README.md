# Narrative Credibility Index CLI

A Rust CLI tool that scans written documents and scores them against the **NCI Engineered Reality Scoring System** — a 20-category PSYOP identification rubric developed by Applied Behavior Research.

## Features

- 📄 **Local File Scanning** — point at any `.md` file and get an AI-reasoned NCI score
- 🌐 **REST API Server** — launch a web server that accepts URLs, fetches live articles, extracts clean Markdown, and returns a full PSYOP analysis
- 🤖 **AI Agent Integration** — uses any OpenAI-compatible API (GPT-4, Ollama, Together AI, etc.) to reason through all 20 categories
- 🖥️ **Interactive TUI** — manually score documents using a beautiful Ratatui-powered terminal interface
- 📊 **Detailed Reports** — colour-coded terminal output and structured JSON responses

## Scoring Rubric

| Score Range | Interpretation |
|-------------|----------------|
| 0 – 25      | Low likelihood of a PSYOP |
| 26 – 50     | Moderate likelihood — look deeper |
| 51 – 75     | Strong likelihood — manipulation likely |
| 76 – 100    | Overwhelming signs of a PSYOP |

Each of the 20 categories is scored **1 (Not Present) → 5 (Overwhelmingly Present)**.

### The 20 NCI Categories

1. Timing
2. Emotional Manipulation
3. Uniform Messaging
4. Missing Information
5. Simplistic Narratives
6. Tribal Division
7. Authority Overload
8. Call for Urgent Action
9. Overuse of Novelty
10. Financial/Political Gain
11. Suppression of Dissent
12. False Dilemmas
13. Bandwagon Effect
14. Emotional Repetition
15. Cherry-Picked Data
16. Logical Fallacies
17. Manufactured Outrage
18. Framing Techniques
19. Rapid Behavior Shifts
20. Historical Parallels

## Installation

```bash
git clone https://github.com/ling0x/narrative-credibility-index
cd narrative-credibility-index
cargo build --release
```

The compiled binary will be at `./target/release/nci`.

## Usage

### 1. Scan a Local Document (CLI)

Analyze a Markdown file using AI:

```bash
# With OpenAI
export OPENAI_API_KEY="sk-..."
./target/release/nci scan article.md

# With local Ollama (no API key required)
./target/release/nci scan article.md --api-url http://localhost:11434/v1 --model llama3

# With Together AI or other compatible endpoints
./target/release/nci scan article.md --api-url https://api.together.xyz/v1 --model meta-llama/Llama-3-70b-chat-hf --api-key YOUR_KEY
```

**Output:**
```
🔍 Scanning: article.md

══════════════════════════════════════════════════════════
     NARRATIVE CREDIBILITY INDEX — SCORE REPORT
══════════════════════════════════════════════════════════
   1. Timing                          [4] ████▒
      Suspicious timing during corporate scandal.
   2. Emotional Manipulation          [5] █████
      Heavy use of fear without evidence.
   ...
──────────────────────────────────────────────────────────
  TOTAL SCORE: 62 / 100
  ⚠ Strong likelihood — manipulation likely
══════════════════════════════════════════════════════════
```

### 2. REST API Server (Web Scraping & Analysis)

Launch a web server that accepts article URLs, automatically fetches them, converts to Markdown, and returns a full NCI analysis:

```bash
# Start the server
./target/release/nci serve --port 3000 --api-url http://localhost:11434/v1 --model llama3
```

**API Endpoint:** `POST /scan`

**Request:**
```bash
curl -X POST http://127.0.0.1:3000/scan \
     -H "Content-Type: application/json" \
     -d '{"url": "https://example.com/news/article"}'
```

**Response:**
```json
{
  "total_score": 62,
  "interpretation": "Strong likelihood — manipulation likely",
  "saved_file": "/path/to/scans/scanned_article_a1b2c3d4.md",
  "scores": [
    {
      "id": 1,
      "name": "Timing",
      "score": 4,
      "reasoning": "Article released during unrelated corporate scandal"
    },
    {
      "id": 2,
      "name": "Emotional Manipulation",
      "score": 5,
      "reasoning": "Heavy use of fear-inducing imagery without evidence"
    }
    // ... 18 more categories
  ]
}
```

**What happens under the hood:**
1. Server fetches the HTML from the provided URL
2. Converts HTML to clean Markdown using `html2md`
3. Saves the Markdown file to `./scans/scanned_article_<uuid>.md`
4. Sends the content to your configured AI endpoint for analysis
5. Returns structured JSON with all 20 category scores and reasoning

### 3. Manual Scoring (Interactive TUI)

Use the Ratatui-powered terminal interface to manually score a document:

```bash
# With a document loaded for reference
./target/release/nci manual article.md

# Without a document (blank slate)
./target/release/nci manual
```

**Controls:**
- `↑↓` or `j/k` — Navigate categories
- `1-5` — Set score directly
- `+/-` or `←→` — Increment/decrement score
- `Enter` or `q` — Finish and show report
- `Ctrl+C` — Exit

**Features:**
- Live score visualization with progress bars
- Real-time total score gauge with color coding
- Side-by-side document preview
- Category details (question + example) displayed for each item

### 4. View the Full Rubric

Print all 20 categories with questions and examples:

```bash
./target/release/nci rubric
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|----------|
| `OPENAI_API_KEY` | API key for OpenAI or compatible services | Required for default OpenAI endpoint |
| `OPENAI_API_URL` | Base URL for API endpoint | `https://api.openai.com/v1` |
| `OPENAI_MODEL` | Model name to use | `gpt-4o` |

**Note:** When using custom endpoints (like Ollama), the API key is optional and defaults to a placeholder.

## Examples

### Example 1: Analyze a Wikipedia Article via API

```bash
# Start server
./target/release/nci serve -p 8080 --api-url http://localhost:11434/v1 --model llama3

# In another terminal
curl -X POST http://127.0.0.1:8080/scan \
     -H "Content-Type: application/json" \
     -d '{"url": "https://en.wikipedia.org/wiki/Propaganda"}'
```

### Example 2: Batch Processing Multiple Articles

```bash
# Create a list of URLs
cat urls.txt
https://example.com/article1
https://example.com/article2
https://example.com/article3

# Process them
while read url; do
  curl -X POST http://127.0.0.1:3000/scan \
       -H "Content-Type: application/json" \
       -d "{\"url\": \"$url\"}" \
       -o "result_$(echo $url | md5sum | cut -d' ' -f1).json"
done < urls.txt
```

### Example 3: Using with Different AI Providers

```bash
# OpenAI GPT-4
export OPENAI_API_KEY="sk-..."
./target/release/nci scan article.md

# Ollama with Llama 3
./target/release/nci scan article.md --api-url http://localhost:11434/v1 --model llama3

# Together AI
./target/release/nci scan article.md \
  --api-url https://api.together.xyz/v1 \
  --model meta-llama/Llama-3-70b-chat-hf \
  --api-key YOUR_TOGETHER_KEY

# Groq
./target/release/nci scan article.md \
  --api-url https://api.groq.com/openai/v1 \
  --model llama-3.1-70b-versatile \
  --api-key YOUR_GROQ_KEY
```

## Output Files

When using the API server, extracted Markdown files are saved to:
```
./scans/scanned_article_<uuid>.md
```

These files can be:
- Reviewed manually for accuracy
- Re-scanned using the CLI: `nci scan scans/scanned_article_*.md`
- Used as training data or audit trails

## Architecture

```
nci/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── server.rs       # Axum REST API
│   ├── ai_agent.rs     # LLM integration
│   ├── score.rs        # Score calculation & display
│   ├── rubric.rs       # 20 NCI categories
│   └── tui.rs          # Ratatui interactive interface
├── example/
│   └── article.md      # Demo article with manipulation markers
└── scans/              # Auto-generated directory for API extractions
```

## Development

```bash
# Run in dev mode
cargo run -- scan article.md

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- serve

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

MIT

---

*Based on the NCI Engineered Reality Scoring System — Applied Behavior Research © 2024*
