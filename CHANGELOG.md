# Changelog

All notable changes to Keycen will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-10

### Added

- OS-level profanity detection using [rustrict](https://crates.io/crates/rustrict) engine
- Global keyboard interception via `rdev` (grab mode with listen fallback)
- Case-preserving replacements (lowercase, UPPERCASE, Titlecase)
- Configurable custom word → replacement mappings via TOML config
- Built-in replacement map (13 common words) loaded at runtime
- False positive allowlist for 30+ common English words
- System tray icon with toggle, reload config, and quit menu
- App exclusion list (terminals, editors excluded by default)
- Config hot-reload via file watcher — changes apply instantly
- Evasion detection: spaced letters, leet-speak, repeated characters
- Cross-platform support: Windows, Linux (X11), macOS
- CLI interface with `--daemon`, `--config`, and `--verbose` flags
- 16 automated tests covering all modules
- GitHub Actions CI (test, clippy, fmt) and release workflows
- SHA-256 checksums for all release binaries

### Technical

- Non-blocking correction via spawned threads (prevents hook timeout)
- `CORRECTION_IN_PROGRESS` flag prevents typing interleave during correction
- 500ms cooldown after corrections prevents re-triggering on replacement words
- Safe-word set auto-generated from replacement values
- Minimum 4-character word length to reduce false positives
- Zero compiler warnings
