# btcli

An online command-line translation tool for Chinese and other languages with TUI interface.

~(^-^)~

## Description

btcli is a command-line translation tool that provides a TUI (Text User Interface) for easy access to Baidu Translate API. It supports multiple languages and provides a convenient way to translate text directly from the command line.

## Features

- Command-line interface with TUI
- Support for multiple languages
- Configuration management
- Error handling and logging
- Cross-platform compatibility

## Installation

To build and install btcli, you'll need Rust and Cargo installed on your system.

Clone the repository and build:

```bash
git clone https://github.com/RevengingSA/btcli.git
cd btcli
cargo build --release
```

The binary will be available in `target/release/btcli`.

## Usage

First, configure your Baidu Translate API credentials:

```bash
# Edit config.toml with your API credentials
# Or run the program once to generate an example config
./btcli
```

The application supports both TUI (Text User Interface) mode and pure command-line mode:

```bash
# Interactive TUI mode
./btcli

# Command line mode - basic translation
./btcli "text to translate"

# Command line mode - with options
./btcli -t zh "Hello world"              # Translate to Chinese
./btcli -s en -t zh "Hello world"      # Specify source and target languages
./btcli --help                         # Show help information
./btcli --version                      # Show version information
```

### Command Line Options

The command-line interface supports the following options:

- `-s, --source LANG`: Specify the source language (e.g., en, zh)
- `-t, --target LANG`: Specify the target language (e.g., en, zh)
- `-h, --help`: Display help information
- `-v, --version`: Display version information

Examples:

```bash
# Basic translation (uses configured default languages)
btcli "Hello world"

# Translate to specific language
btcli -t zh "Hello world"

# Specify both source and target languages
btcli -s en -t fr "Hello world"

# Get help
btcli --help
```

## Dependencies

- Rust 1.70+
- Cargo
- UPX (optional, for binary compression)
- tar (for packaging)
- zip (for packaging)

## Build Script Usage

The unified build script supports various options:

```bash
# Build with default formats (zst,zip) for all common platforms
./scripts/build.sh

# Clean build artifacts
./scripts/build.sh clean

# Build with specific compression formats
./scripts/build.sh build --format zst
./scripts/build.sh build --format zip,gz

# Check dependencies
./scripts/build.sh check

# Build for specific target platforms
./scripts/build.sh --target x86_64-unknown-linux-gnu
./scripts/build.sh --target x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu
./scripts/build.sh --target x86_64-unknown-linux-gnu --format zst

# Build with all files in a single directory (no debug/release separation)
./scripts/build.sh --allin
```

## UPX Compression

For release builds, you can compress the binary with UPX to reduce file size:

```bash
# Build the release binary
cargo build --release

# Compress the binary with UPX
cargo run --release --bin post_build
```

This will reduce the binary size significantly (typically by 60-70%).

## MSYS2 Environment Notes

When using the build script in MSYS2 environment:

- The script detects MSYS2 automatically
- Dependencies are checked but not automatically installed
- Install missing packages manually using `pacman -S package-name`
- Typical packages needed: `mingw-w64-x86_64-rust`, `mingw-w64-x86_64-upx`, `zip`, `tar`

## Multi-Platform Support

The build script supports cross-compilation for multiple platforms:

- Linux x86_64: `x86_64-unknown-linux-gnu`
- Linux ARM64: `aarch64-unknown-linux-gnu`
- Linux RISC-V64: `riscv64gc-unknown-linux-gnu`
- Windows ARM64: `aarch64-pc-windows-msvc`
- Android ARM64: `aarch64-linux-android`
- Android ARMv7: `armv7-linux-androideabi`
- Android x86_64: `x86_64-linux-android`
- Android x86: `i686-linux-android`

Install targets with: `rustup target add <target>`

## Termux Support

The build script also works in Termux environment on Android:

- The script automatically detects Termux environment
- Install dependencies with: `pkg install rust cargo tar zip upx rustup zstd`
- Cross-compile for various platforms as described above
- By default builds for Android targets: `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`, `i686-linux-android`

## Output Files

The build script generates files with the naming convention:
`btcli-{version}-{os}-{arch}.(tar.zst|zip|tar.gz|tar.xz)`

Each archive contains:
- debug/ directory with debug binaries and all debug artifacts (.d, .rlib, deps/) (normal mode)
- release/ directory with release binaries and all release artifacts (normal mode)
- Or all files in a single directory (when using --allin flag)

## License

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. 
If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

Copyright (C) 2026 S.A. (@snoware)

## Author

S.A. (@snoware)