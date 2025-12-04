#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DAEMON_DIR="$PROJECT_ROOT/centy-daemon"
BIN_DIR="$PROJECT_ROOT/src-tauri/bin"

# Ensure bin directory exists
mkdir -p "$BIN_DIR"

echo "Building centy-daemon sidecar..."

cd "$DAEMON_DIR"

# Detect current platform
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Darwin)
        case "$ARCH" in
            x86_64)
                TARGET="x86_64-apple-darwin"
                ;;
            arm64)
                TARGET="aarch64-apple-darwin"
                ;;
            *)
                echo "Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            x86_64)
                TARGET="x86_64-unknown-linux-gnu"
                ;;
            aarch64)
                TARGET="aarch64-unknown-linux-gnu"
                ;;
            *)
                echo "Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*)
        TARGET="x86_64-pc-windows-msvc"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Building for target: $TARGET"

# Build the daemon
cargo build --release --target "$TARGET"

# Copy to bin directory with platform suffix
if [[ "$OS" == MINGW* || "$OS" == MSYS* || "$OS" == CYGWIN* ]]; then
    cp "$DAEMON_DIR/target/$TARGET/release/centy-daemon.exe" "$BIN_DIR/centy-daemon-$TARGET.exe"
else
    cp "$DAEMON_DIR/target/$TARGET/release/centy-daemon" "$BIN_DIR/centy-daemon-$TARGET"
fi

echo "Sidecar built successfully: centy-daemon-$TARGET"
