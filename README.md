# MilyAI

Modular AI assistant CLI in Rust. Text REPL by default, with optional TTS, STT, camera, and voice wake mode.

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

# voice wake mode (tts+stt)
cargo build --features voice
```

## Run

```bash
# interactive REPL
milyai run

# voice mode (say "Milly" to wake)
milyai voice
```

## Config

Default config path: OS config dir, e.g. Windows `%APPDATA%/milyai/config.yaml`.

```yaml
agent_name: "Mily"
persona: "Ramah, ingin tahu, membantu"
curiosity: 0.6
llm_endpoint: "http://localhost:11434/api/generate"
# stt_model_path: "C:/models/vosk-model-small-id-0.22"
```

Install via Chocolatey/Homebrew: see `packaging/` (replace placeholder URLs before publishing). 