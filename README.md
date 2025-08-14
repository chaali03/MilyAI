# MilyAI

Modular AI assistant CLI in Rust. Text REPL by default, with optional TTS, STT, camera, voice wake mode, and web learning.

## Build

```bash
# minimal (text-only)
cargo build
# with voice (tts+stt)
cargo build --features voice
# with web learning
cargo build --features web
```

## Run

```bash
# voice mode (say "Milly" to wake)
milyai voice
# browse a URL and learn (web)
milyai browse --url https://example.com
# learn daemon: periodically learn from configured URLs
milyai learn
```

## Config

```yaml
agent_name: "Mily"
persona: "Ramah, ingin tahu, membantu"
curiosity: 0.6
llm_endpoint: "http://localhost:11434/api/generate"
# stt_model_path: "C:/models/vosk-model-small-id-0.22"
# Web learning
web_user_agent: "MilyAI/0.1 (+https://example.com)"
learn_urls:
  - "https://www.rust-lang.org/"
  - "https://news.ycombinator.com/"
learn_interval_secs: 3600
```

Install via Chocolatey/Homebrew: see `packaging/` (replace placeholder URLs before publishing). 