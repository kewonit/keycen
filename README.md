# Keycen

[![CI](https://github.com/kewonit/keycen/actions/workflows/ci.yml/badge.svg)](https://github.com/kewonit/keycen/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/kewonit/keycen?style=flat-square)](https://github.com/kewonit/keycen/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**OS-level profanity autocorrect sensor** — Keycen monitors your keyboard input system-wide and silently replaces cuss words with appropriate alternatives in real-time.

Type naturally. Keycen handles the rest.

## Quick Start

### Download (Recommended)

Grab the latest release for your platform from [**Releases**](https://github.com/kewonit/keycen/releases/latest):

| Platform            | Download                      |
| ------------------- | ----------------------------- |
| Windows x64         | `keycen-windows-x86_64.zip`   |
| Linux x64           | `keycen-linux-x86_64.tar.gz`  |
| Linux ARM64         | `keycen-linux-aarch64.tar.gz` |
| macOS Intel         | `keycen-macos-x86_64.tar.gz`  |
| macOS Apple Silicon | `keycen-macos-aarch64.tar.gz` |

Then extract and run:

```bash
# Windows
.\keycen.exe --verbose

# Linux / macOS
chmod +x keycen
./keycen --verbose
```

### Build from Source

```bash
git clone https://github.com/kewonit/keycen.git
cd keycen
cargo build --release
./target/release/keycen --verbose
```

## How It Works

1. Keycen runs as a background process and monitors global keyboard input
2. As you type, it buffers characters into words
3. When a word boundary is detected (space, punctuation, enter), the word is checked against a profanity filter
4. If profanity is detected, the word is automatically replaced with a configured alternative
5. The replacement happens so fast it feels like autocorrect

## Features

- **Cross-platform**: Windows, Linux, macOS
- **System-wide**: Works in any application — browsers, chat apps, editors, everywhere
- **Evasion-resistant**: Catches alternative spellings, repeated characters, confusables, spacing tricks
- **Case-preserving**: Matches replacement casing to original (e.g., `b*d`→`good`, `B*D`→`GOOD`, `B*d`→`Good`)
- **Configurable**: Custom replacement words, app exclusions, TOML config with hot-reload
- **Lightweight**: ~3MB binary, minimal CPU usage while typing
- **Privacy-first**: No network access, no telemetry, no keystroke storage. Fully open source.

## Installation

### Prerequisites (Build from Source)

- [Rust toolchain](https://rustup.rs/) (stable, 1.70+)
- **Linux**: `libx11-dev libxtst-dev libxdo-dev` (`sudo apt-get install -y libx11-dev libxtst-dev libxdo-dev`)

### Pre-built Binaries

Download from [Releases](https://github.com/kewonit/keycen/releases/latest). Each release includes SHA-256 checksums for verification.

### Build from Source

```bash
git clone https://github.com/kewonit/keycen.git
cd keycen
cargo build --release
```

Binary will be at `target/release/keycen` (or `keycen.exe` on Windows).

### Platform-Specific Setup

**Windows:** No special setup needed. Just run it.

**Linux:** Add your user to the `input` group for grab mode:

```bash
sudo usermod -aG input $USER
# Log out and back in for group change to take effect
```

**macOS:** Grant Accessibility permission:
System Preferences → Security & Privacy → Privacy → Accessibility → Add `keycen`

## Usage

```bash
# Run with system tray (default)
keycen

# Run headless (no tray icon, good for servers/scripts)
keycen --daemon

# Custom config file
keycen --config /path/to/config.toml

# Verbose logging (shows every detection/correction)
keycen --verbose

# Show help
keycen --help
```

## Configuration

Config file is auto-created on first run at:

| Platform | Path                                               |
| -------- | -------------------------------------------------- |
| Windows  | `%APPDATA%\keycen\config.toml`                     |
| Linux    | `~/.config/keycen/config.toml`                     |
| macOS    | `~/Library/Application Support/keycen/config.toml` |

A bundled default config is also at `config/default.toml` in the repo.

### Example Config

```toml
[general]
enabled = true
# "auto" = try grab mode, fall back to listen mode
# "grab" = intercept-before-delivery (best UX, needs permissions)
# "listen" = observe-after-delivery (works everywhere, brief flash)
mode = "auto"

[replacements]
# word = "replacement" (case is auto-matched)
# Built-in replacements are loaded automatically.
# Add your own custom overrides here, e.g.:
# badword = "niceword"
# Words not in the built-in or custom map get asterisk censoring (e.g., "f***")

[exclusions]
# Apps where Keycen will NOT filter (terminals, editors, etc.)
apps = [
    "cmd.exe",
    "powershell.exe",
    "bash",
    "zsh",
    "WindowsTerminal.exe",
]
```

**Hot-reload**: Edit the config while Keycen is running — changes apply automatically.

## Development

### Run Tests

```bash
# Run all 16 tests
cargo test

# Run specific test suite
cargo test --test filter_tests      # Profanity detection tests
cargo test --test buffer_tests      # Word boundary tests
cargo test --test correction_tests  # Correction planning tests
cargo test --test config_tests      # Config parsing tests
cargo test --test integration_tests # End-to-end pipeline tests
```

### Build

```bash
# Debug build (fast compile, slow runtime)
cargo build

# Release build (optimized, ~3MB with LTO)
cargo build --release

# Lint
cargo clippy
```

### Project Structure

```
src/
├── main.rs               # Entry point, CLI parsing
├── app.rs                # Application orchestrator
├── config/
│   ├── mod.rs            # Config structs, TOML loader/saver
│   └── watcher.rs        # Config file hot-reload watcher
├── buffer/
│   ├── mod.rs            # Word buffer (accumulates typed chars)
│   └── classifier.rs     # Key event → buffer action classifier
├── filter/
│   ├── mod.rs            # ProfanityFilter trait
│   └── rustrict_filter.rs # rustrict implementation + case matching
├── correction/
│   ├── mod.rs            # Shared correction utilities, IS_SIMULATING guard
│   ├── grab_corrector.rs # Grab mode correction (backspace + retype)
│   └── listen_corrector.rs # Listen mode correction (erase + retype + boundary)
├── input/
│   ├── mod.rs            # InputMode enum
│   ├── grab.rs           # Grab mode listener (rdev::grab)
│   └── listen.rs         # Listen mode listener (rdev::listen)
├── appfilter/
│   ├── mod.rs            # App exclusion filter
│   ├── windows.rs        # Windows active window detection
│   ├── linux.rs          # Linux/X11 active window detection
│   └── macos.rs          # macOS active window detection
└── tray/
    ├── mod.rs            # System tray icon (green/red)
    └── menu.rs           # Tray context menu (toggle, reload, quit)

tests/
├── config_tests.rs       # Config TOML parsing
├── buffer_tests.rs       # Character/word classification
├── filter_tests.rs       # Profanity detection + evasion
├── correction_tests.rs   # Correction keystroke planning
└── integration_tests.rs  # Full pipeline tests
```

## Technical Details

### Hybrid Interception Strategy

1. **Grab mode** (primary): Uses OS-level keyboard hooks to intercept keystrokes _before_ they reach applications. When profanity is detected at a word boundary, the word is erased via simulated backspaces and the replacement is typed. Best UX — the user never sees the profanity. Requires elevated permissions on some platforms.

2. **Listen mode** (fallback): Observes keystrokes _after_ delivery. When profanity is detected, sends simulated backspace keystrokes to erase the word, then types the replacement. Works without special permissions but has a brief visual "flash" of the original word.

### Self-Correction Loop Prevention

Keycen uses an `IS_SIMULATING` atomic flag — when the correction engine is typing replacement characters, all input events are passed through without processing. This prevents infinite loops.

### Profanity Detection

Powered by [rustrict](https://crates.io/crates/rustrict) — a Rust profanity filter with:

- O(n) Aho-Corasick–based matching
- Unicode confusable detection (е → e, ⓕ → f)
- Spacing/repetition normalization (f u c k, fuuuuck)
- Leet speak handling (sh1t, @ss)

## Privacy & Security

- **Zero network access**: Keycen never connects to the internet
- **Zero storage**: Keystrokes are buffered in memory only for the current word, then immediately discarded
- **Zero telemetry**: No data collection of any kind
- **Open source**: Every line of code is auditable
- **Minimal permissions**: Only requests what's needed for keyboard hooks

## License

MIT
