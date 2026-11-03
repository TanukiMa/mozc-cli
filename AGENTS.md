# AGENTS.md — Mozc CLI

## Project Overview

Mozc CLI is a cross-platform command-line tool for restarting Mozc (Google Japanese Input) components on **Windows** and **macOS**.

It was initiated after a discussion between a Mozc developer (yukawa) and the project author. The core request was:

> "We want a Mozc CLI. For now, a tool that can restart `mozc_server`, `mozc_renderer`, and the client on Windows and macOS would be good."

The tool aims to provide a simple, reliable way to restart the Mozc engine without requiring users to manually kill processes or re-login.

## Goals (from discussion)

1. **Primary feature**: Restart Mozc-related processes
   - Windows: `GoogleIMEJaConverter.exe`, `GoogleIMEJaRenderer.exe`, `GoogleJapaneseInput.exe`
   - macOS: `com.google.inputmethod.Japanese.Converter` and `com.google.inputmethod.Japanese.Renderer` (via `launchctl`)

2. **Platform support**: Windows and macOS (Linux is out of scope for the initial version)

3. **Internationalization (i18n)**: Support multiple languages via locale files (JSON)
   - Default languages: Japanese (`ja`) and English (`en`)
   - Automatic language detection based on system locale / `LANG`
   - Easy to add more languages later

4. **Release language**: **Rust** (single binary, no runtime dependency)
5. **Test language**: **Python** is acceptable for tests and prototypes

6. **Future extensibility** (not required for v0.1):
   - `status` command (show running processes / version)
   - `--lang` / `-l` option
   - Dictionary / config tool launchers
   - Homebrew / Scoop packages

## Technical Decisions

| Item                    | Decision                          | Reason                                      |
|-------------------------|-----------------------------------|---------------------------------------------|
| Release language        | Rust                              | Single binary, fast, safe, easy distribution |
| Test / prototype        | Python                            | Quick iteration                             |
| CLI parsing             | `clap` (derive)                   | Idiomatic Rust CLI                          |
| i18n                    | JSON locale files + simple loader | Simple, human-readable, easy to translate   |
| Process management      | OS commands (`taskkill` / `launchctl` / `pkill`) | Simple and reliable for current scope |
| Configuration           | None for v0.1                     | Keep it simple                              |

## Repository Structure

```
mozc-cli/
├── AGENTS.md              # This file — guidance for AI agents & contributors
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs            # Main application logic
├── locales/
│   ├── en.json            # English messages
│   └── ja.json            # Japanese messages
└── tests/                 # Python tests (optional)
    └── test_i18n.py
```

## Coding Guidelines for Agents

- Prefer **Rust** for any production/release code.
- Keep the binary as self-contained as possible.
- Locale files must remain simple JSON key-value pairs. Do not introduce complex i18n frameworks unless necessary.
- Always support both Windows and macOS code paths.
- Error messages and user-facing text must go through the i18n system.
- When adding a new user-visible string, update **both** `en.json` and `ja.json`.
- Avoid hard-coding Japanese or English strings in source code.

## Current Status (v0.1)

- [x] Basic project structure
- [x] JSON-based i18n
- [x] `restart` command (Windows + macOS)
- [ ] `status` command
- [ ] Proper error handling & exit codes
- [ ] Automated tests
- [ ] GitHub Actions CI
- [ ] Binary releases (GitHub Releases)

## How to Build & Run

```bash
cargo build --release
./target/release/mozc-cli restart
```

Language is auto-detected. You can force English with:

```bash
LANG=en_US.UTF-8 ./target/release/mozc-cli restart
```

## Notes for Future Contributors / AI Agents

- The original discussion happened in Japanese. The author prefers technical discussion in Japanese but wants documentation and code comments in English where possible.
- The tool is intentionally minimal. Do not add features that were not requested unless they significantly improve reliability or usability.
- When in doubt about Mozc process names or launchctl service names, verify against a real installation rather than assuming.
