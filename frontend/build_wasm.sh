#!/bin/bash
set -e

echo "🦀 Building City Dashboard for WebAssembly..."
echo ""

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build in release mode for WebAssembly
echo "🔨 Building WASM binary..."
cargo build --release --target wasm32-unknown-unknown

# Create dist directory
echo "📁 Creating dist directory..."
mkdir -p dist

# Copy files to dist
echo "📋 Copying files to dist..."
cp target/wasm32-unknown-unknown/release/frontend.wasm dist/
cp index.html dist/

# Download macroquad's JS glue code if not present
if [ ! -f "dist/gl.js" ]; then
    echo "⬇️  Downloading macroquad JS glue code..."
    curl -o dist/gl.js https://raw.githubusercontent.com/not-fl3/macroquad/master/js/gl.js
fi

# Calculate file sizes
WASM_SIZE=$(du -h dist/frontend.wasm | cut -f1)
echo ""
echo "✅ Build complete!"
echo "📊 WASM size: $WASM_SIZE"
echo "📂 Output directory: dist/"
echo ""
echo "To test locally:"
echo "  cd dist"
echo "  python3 -m http.server 8000"
echo "  # Then open http://localhost:8000 in your browser"
echo ""
