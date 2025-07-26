#!/bin/bash

# Build script for lin-ldf WASM demo

set -e

echo "🦀 Building lin-ldf WASM module..."

# Check if wasm-pack is installed, auto-install if cargo is available
if ! command -v wasm-pack &> /dev/null; then
    echo "⚠️  wasm-pack not found."
    if command -v cargo &> /dev/null; then
        echo "🔧 Installing wasm-pack via cargo..."
        cargo install wasm-pack
        if [ $? -ne 0 ]; then
            echo "❌ Failed to install wasm-pack"
            exit 1
        fi
        echo "✅ wasm-pack installed successfully"
    else
        echo "❌ cargo not found. Please install Rust and cargo first:"
        echo "   https://rustup.rs/"
        exit 1
    fi
fi

# Build the WASM module
echo "🔧 Compiling Rust to WebAssembly..."
wasm-pack build --target web --out-dir pkg

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🚀 To run the demo:"
    echo "   cd wasm/"
    echo "   python3 -m http.server 8000"
    echo ""
    echo "Then open http://localhost:8000 in your browser"
else
    echo "❌ Build failed"
    exit 1
fi 