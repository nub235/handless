<p align="center">
  <img src="src-tauri/icons/logo.png" alt="Handless logo" width="128" height="128" />
</p>

<h1 align="center">Handless</h1>

<p align="center">
  Free, open-source macOS speech-to-text.<br/>
  Press a shortcut, speak, get text in any app. Run locally for privacy or use cloud APIs.
</p>

<p align="center">
  <a href="https://handless.elwin.cc"><img src="https://img.shields.io/badge/Website-handless.elwin.cc-ef6f2f" alt="Website" /></a>
  <a href="https://github.com/ElwinLiu/handless/actions/workflows/build-test.yml"><img src="https://github.com/ElwinLiu/handless/actions/workflows/build-test.yml/badge.svg" alt="Build" /></a>
  <a href="https://github.com/ElwinLiu/handless/actions/workflows/lint.yml"><img src="https://github.com/ElwinLiu/handless/actions/workflows/lint.yml/badge.svg" alt="Lint" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>
</p>

<p align="center">
  <a href="docs/README.zh.md">简体中文</a> ·
  <a href="docs/README.zh-TW.md">繁體中文</a> ·
  <a href="docs/README.ja.md">日本語</a>
</p>

## 
## This fork adds [Parakeet Unified](https://huggingface.co/nvidia/parakeet-unified-en-0.6b) with real-time streaming using [parakeet-rs](https://github.com/altunenes/parakeet-rs)

## Features

- **Local transcription** -- download models in Settings, runs fully on-device
- **Cloud STT** via OpenAI, Soniox, Deepgram, etc
- **Voice Activity Detection** (local models only)
- **LLM post-processing** to clean up or reformat transcriptions
- **macOS** (Intel & Apple Silicon)
- **17 languages**

## Install

**[Download for macOS](https://github.com/ElwinLiu/handless/releases/latest)** (Intel & Apple Silicon)

Also available at [handless.elwin.cc](https://handless.elwin.cc). Build from source: see [BUILD.md](BUILD.md).

## CLI

**Remote control** (talks to a running instance):

```bash
handless --toggle-transcription    # Toggle recording
handless --toggle-post-process     # Toggle recording + post-processing
handless --cancel                  # Cancel current operation
```

**Startup flags:**

```bash
handless --start-hidden            # No main window
handless --no-tray                 # No tray icon
handless --debug                   # Verbose logging
handless --help                    # All flags
```

Combine freely: `handless --start-hidden --no-tray`

> **macOS:** invoke the binary directly: `/Applications/Handless.app/Contents/MacOS/Handless --toggle-transcription`

## Troubleshooting

`Cmd+Shift+D` opens the debug panel.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). For translations: [CONTRIBUTING_TRANSLATIONS.md](CONTRIBUTING_TRANSLATIONS.md).

## License

[MIT](LICENSE)

## Acknowledgments

Forked from [Handy](https://github.com/cjpais/Handy) v0.7.8.

[Whisper](https://github.com/openai/whisper) | [whisper.cpp](https://github.com/ggerganov/whisper.cpp) | [NeMo Parakeet](https://github.com/NVIDIA/NeMo) | [Moonshine](https://github.com/usefulsensors/moonshine) | [SenseVoice](https://github.com/FunAudioLLM/SenseVoice) | [Silero VAD](https://github.com/snakers4/silero-vad) | [Tauri](https://tauri.app) | [Handy](https://github.com/cjpais/Handy)
