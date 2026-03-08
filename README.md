# Tonalli

A terminal-based AI chat application with a TUI interface, supporting multiple LLM providers.

## Requirements

- [Rust](https://rustup.rs/) (edition 2024)

## Building

```bash
cargo build --release
```

## Configuration

Configuration is done via environment variables. You can set them directly or use a `.env` file in the project root.

### Required

| Variable | Description |
|---|---|
| `AGENT_PROVIDER` | LLM provider to use. Supported values: `gemini`, `ollama` |
| `AGENT_MODEL` | Model name to use (e.g. `gemini-2.0-flash`, `llama3.2`) |

### Gemini

| Variable | Description |
|---|---|
| `AGENT_API_KEY` | Gemini API key. Required when `AGENT_PROVIDER=gemini` |

### Ollama

| Variable | Description |
|---|---|
| `AGENT_OLLAMA_HOST` | Ollama host. Defaults to `localhost:11434` |

Ollama must be running locally or at the configured host before starting the app.

## Running

```bash
cargo run --release
```

### Example `.env` for Gemini

```env
AGENT_PROVIDER=gemini
AGENT_MODEL=gemini-2.0-flash
AGENT_API_KEY=your_api_key_here
```

### Example `.env` for Ollama

```env
AGENT_PROVIDER=ollama
AGENT_MODEL=llama3.2
# AGENT_OLLAMA_HOST=localhost:11434  # optional, defaults to localhost:11434
```

## Keybindings

| Key | Action |
|---|---|
| `Enter` | Send message |
| `Ctrl+C` / `Esc` | Quit |
| `Up` / `Down` | Scroll history |
| `Page Up` / `Page Down` | Scroll history faster |
| Scroll wheel | Scroll history |
