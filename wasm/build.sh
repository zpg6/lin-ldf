#!/bin/bash

# Build WASM with enhanced TypeScript definitions
echo "🔧 Building WASM package with enhanced TypeScript definitions..."

# Check if wasm-pack is installed, install if not
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 wasm-pack not found, installing via cargo..."
    cargo install wasm-pack
    
    # Verify installation
    if ! command -v wasm-pack &> /dev/null; then
        echo "❌ Failed to install wasm-pack"
        exit 1
    fi
    echo "✅ wasm-pack installed successfully"
fi

# Generate TypeScript bindings with ts-rs
echo "📝 Generating TypeScript definitions..."
cargo test generate_typescript_bindings --features ts-rs --quiet

# Build the WASM package first
echo "🦀 Building WASM package..."
wasm-pack build --target web --out-dir pkg --features ts-rs

# Now intelligently patch the lin_ldf.d.ts with proper types
echo "🔄 Patching TypeScript definitions..."

# Create a Node.js script to patch the types
cat > patch_types.js << 'EOF'
const fs = require('fs');
const path = require('path');

// Read all ts-rs generated types
const typesDir = './types';
const pkgFile = './pkg/lin_ldf.d.ts';

function readAllTypes() {
    const types = new Map();
    const files = fs.readdirSync(typesDir);
    
    for (const file of files) {
        if (file.endsWith('.ts') && file !== 'index.ts') {
            const content = fs.readFileSync(path.join(typesDir, file), 'utf8');
            const typeName = file.replace('.ts', '');
            
            // Extract the type definition (remove imports and comments)
            const lines = content.split('\n');
            let typeLines = [];
            let inType = false;
            
            for (const line of lines) {
                if (line.startsWith('export type ')) {
                    inType = true;
                }
                if (inType) {
                    typeLines.push(line);
                }
            }
            
            if (typeLines.length > 0) {
                types.set(typeName, typeLines.join('\n'));
            }
        }
    }
    
    return types;
}

function patchDefinitions() {
    let content = fs.readFileSync(pkgFile, 'utf8');
    const types = readAllTypes();
    
    // Replace function return types
    content = content.replace(
        /export function parse_ldf_file\(ldf_content: string\): any;/,
        'export function parse_ldf_file(ldf_content: string): LinLdf;'
    );
    
    content = content.replace(
        /export function ldf_from_json\(json_str: string\): any;/,
        'export function ldf_from_json(json_str: string): LinLdf;'
    );
    
    content = content.replace(
        /export function get_ldf_stats\(ldf_content: string\): any;/,
        'export function get_ldf_stats(ldf_content: string): LdfStats;'
    );
    
    // Add LdfStats type definition
    const statsType = `
/**
 * Statistics about an LDF file's contents
 */
export interface LdfStats {
  signal_count: number;
  frame_count: number;
  node_count: number;
  schedule_table_count: number;
  diagnostic_signal_count: number;
  diagnostic_frame_count: number;
  node_attributes_count: number;
  signal_encoding_types_count: number;
  signal_representations_count: number;
  lin_protocol_version: string;
  lin_language_version: string;
  lin_speed: number;
}`;
    
    // Insert all type definitions before the InitInput type
    const typeDefinitions = Array.from(types.values()).join('\n\n') + '\n\n' + statsType + '\n\n';
    
    // Find the position to insert types (before InitInput)
    const insertPosition = content.indexOf('export type InitInput');
    if (insertPosition !== -1) {
        content = content.slice(0, insertPosition) + typeDefinitions + content.slice(insertPosition);
    }
    
    // Add header comment about enhanced types
    const header = `/* tslint:disable */
/* eslint-disable */
/**
 * Enhanced TypeScript definitions for lin-ldf WASM package
 * 
 * This file combines wasm-bindgen generated function signatures with
 * ts-rs generated type definitions for perfect type safety.
 */

`;
    
    content = content.replace(/^\/\* tslint:disable \*\/\n\/\* eslint-disable \*\/\n/, header);
    
    fs.writeFileSync(pkgFile, content);
    console.log('✅ Successfully patched lin_ldf.d.ts with enhanced types');
}

patchDefinitions();
EOF

# Run the patching script
node patch_types.js

# Clean up
rm patch_types.js

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 WASM package: wasm/pkg/"
echo ""
echo "🚀 Usage example:"
echo "   import init, { parse_ldf_file, type LinLdf } from './pkg/lin_ldf.js';"
echo "   const ldf: LinLdf = parse_ldf_file(content); // Fully typed!" 
echo "🚀 To run the demo:"
echo "   cd wasm/"
echo "   python3 -m http.server 8000"
echo ""
echo "Then open http://localhost:8000 in your browser"
echo ""