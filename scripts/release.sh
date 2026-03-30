#!/bin/bash
set -e

echo "🚀 BetterCurl Release Builder"
echo "=============================="

VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d'"' -f2)
echo "📦 Building version: $VERSION"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Define targets
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

# Build for each target
for TARGET in "${TARGETS[@]}"; do
    echo "🔨 Building for $TARGET..."
    rustup target add "$TARGET" 2>/dev/null || true

    case "$TARGET" in
        x86_64-unknown-linux-gnu)
            OS="linux"
            BINARY="bettercurl"
            ;;
        x86_64-apple-darwin|aarch64-apple-darwin)
            OS="macos"
            BINARY="bettercurl"
            ;;
        x86_64-pc-windows-gnu)
            OS="windows"
            BINARY="bettercurl.exe"
            ;;
    esac

    # Build
    cross build --release --target "$TARGET" || cargo build --release --target "$TARGET"

    # Create archive
    ARCHIVE="bettercurl-$VERSION-$OS-$TARGET.tar.gz"
    mkdir -p "release/$TARGET"
    cp "target/$TARGET/release/$BINARY" "release/$TARGET/"

    if [[ "$OS" != "windows" ]]; then
        (cd "release/$TARGET" && tar -czf "../../$ARCHIVE" "$BINARY")
    else
        (cd "release/$TARGET" && tar -czf "../../$ARCHIVE" --force-local "$BINARY")
    fi

    echo "✅ Created $ARCHIVE"
done

echo ""
echo "📦 Release assets ready:"
ls -lh bettercurl-$VERSION-*.tar.gz
echo ""
echo "To create a GitHub release, run:"
echo "  gh release create v$VERSION bettercurl-$VERSION-*.tar.gz --generate-notes"
