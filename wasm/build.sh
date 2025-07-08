#!/bin/bash

# Build script for lin-ldf WASM demo

set -e

echo "🦀 Building lin-ldf WASM module..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Please install it first:"
    echo "   cargo install wasm-pack"
    exit 1
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