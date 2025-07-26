# TypeScript Tests for lin-ldf WASM Package

This directory contains comprehensive TypeScript tests for the `lin-ldf` WASM package, validating both functionality and type safety.

## Test Structure

```
wasm/tests/
├── package.json       # Test dependencies and scripts
├── tsconfig.json      # TypeScript configuration
├── bun.lock          # Dependency lock file
└── tests/            # Test files
    ├── basic.test.ts         # Core functionality tests
    ├── error-handling.test.ts # Error handling scenarios
    └── type-safety.test.ts   # TypeScript type safety validation
```

## Test Categories

### 🧪 Basic Functionality (`basic.test.ts`)

- LDF file parsing with the README example
- Signal, frame, and node validation
- Statistics generation
- JSON serialization/deserialization
- Schedule tables and node attributes

### ❌ Error Handling (`error-handling.test.ts`)

- Invalid LDF content rejection
- Malformed header detection
- Incomplete file handling
- Empty/whitespace content

### 🎯 Type Safety (`type-safety.test.ts`)

- TypeScript interface validation
- Enum type handling (Scalar vs Array)
- Array type consistency
- Type-safe transformations (filter, map, reduce)

## Running Tests

### Prerequisites

- [Bun](https://bun.sh) installed
- WASM package built with enhanced TypeScript definitions

### Commands

```bash
# Install dependencies
bun install

# Run all tests
bun test

# Run specific test file
bun test tests/basic.test.ts

# Watch mode
bun test --watch
```

## What's Being Tested

✅ **Enhanced TypeScript Support**: All 20 type definitions work correctly  
✅ **Function Return Types**: No `any` types - all functions return properly typed objects  
✅ **Type Safety**: Full IntelliSense and compile-time error detection  
✅ **Runtime Validation**: Actual parsing functionality matches type definitions  
✅ **Error Handling**: Graceful handling of invalid input

## Test Data

Tests use the same LDF example from the main README to ensure consistency between documentation and implementation.

## CI Integration

These tests run automatically on every push/PR via GitHub Actions using Bun as the test runner.
