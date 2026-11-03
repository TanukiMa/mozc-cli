# mozc-cli

CLI tool to restart **Mozc** (Google Japanese Input) components on **Windows** and **macOS**.

## Motivation

This tool was born from a conversation with a Mozc developer (yukawa):

> "Mozc CLI ほしいよね。とりあえずは Windows と macOS で mozc_server / mozc_renderer と client を再起動するツールができれば良いと思う。"

## Features

- Restart Mozc converter / renderer / client with a single command
- Cross-platform (Windows + macOS)
- Multilingual support (Japanese / English) via JSON locale files
- Single binary written in Rust

## Installation

### From source

```bash
git clone https://github.com/TanukiMa/mozc-cli.git
cd mozc-cli
cargo build --release
```

The binary will be at `target/release/mozc-cli` (or `mozc-cli.exe` on Windows).

### Cargo

```bash
cargo install --git https://github.com/TanukiMa/mozc-cli.git
```

## Usage

```bash
# Restart Mozc
mozc-cli restart

# Force English messages
mozc-cli --lang en restart

# Force Japanese messages
mozc-cli --lang ja restart
```

### Language detection

The tool automatically selects the language based on:

1. `--lang` / `-l` option
2. `LC_ALL` / `LC_MESSAGES` / `LANG` environment variables
3. Falls back to English

## Supported platforms

| Platform | Status     | Method                                      |
|----------|------------|---------------------------------------------|
| Windows  | Supported  | `taskkill` on known process names           |
| macOS    | Supported  | `launchctl stop/start` on Mozc services     |
| Linux    | Not supported | —                                        |

## Project structure

See [AGENTS.md](./AGENTS.md) for detailed guidance intended for AI coding agents and contributors.

## License

MIT
