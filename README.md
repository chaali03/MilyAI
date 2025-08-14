# MilyAI

Local-first AI assistant (Rust). Optional voice, web learning, and local LLM.

## Voice conversation
- Wake word: `milyai voice` (say “Milly”)
- Continuous talk (auto stop on silence): `milyai voicechat`
- Requires Vosk model path in config and TTS feature.

```yaml
stt_model_path: "C:/models/vosk-model-small-id-0.22"
```

Build:
```bash
cargo build --features "voice"         # wake word
cargo build --features "stt-vosk tts"  # for voicechat
```

## Local LLM (no API key)
- Install Ollama: `https://ollama.com/download`
- Pull a model (examples):
  - `ollama pull llama3.1:8b`
  - `ollama pull gemma2:9b`
- Config `%APPDATA%/milyai/config.yaml`:
```yaml
ollama_url: "http://127.0.0.1:11434"
ollama_model: "llama3.1:8b"
```
- Build and run:
```bash
cargo build --features "llm-ollama"
cargo run --features "llm-ollama" -- run
```

Tip: You can also use a custom `llm_endpoint` if you have your own local server.

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