# MilyAI

Modular AI assistant CLI in Rust. Text REPL by default, with optional TTS, STT, and camera modules via feature flags.

## Build

```bash
# minimal (text-only)
cargo build

# with TTS
cargo build --features tts

# with STT (requires Vosk model path configured)
cargo build --features stt-vosk

# with camera
cargo build --features camera
```

## Run

```bash
# interactive REPL
milyai run

# say text (requires --features tts)
milyai say "halo dunia"

# transcribe 4s (requires --features stt-vosk)
milyai listen

# snapshot (requires --features camera)
milyai snapshot --output frame.png
```

## Config

Default config path: OS config dir, e.g. Windows `%APPDATA%/milyai/config.yaml`.

```yaml
agent_name: "Mily"
persona: "Ramah, ingin tahu, membantu"
curiosity: 0.6
llm_endpoint: "http://localhost:11434/api/generate" # example
# stt_model_path: "C:/models/vosk-model-small-id-0.22"
```

## Install

- Chocolatey (placeholder): `choco install milyai`
- Homebrew (placeholder): `brew install milyai`

Replace URLs in `packaging/` with real release artifacts when publishing. 