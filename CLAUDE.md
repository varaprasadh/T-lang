# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Telugu Programming Language (`telugu_lang`) is a Rust-based programming language compiler/interpreter that uses Telugu keywords. It compiles `.tel` files using the `telc` binary.

## Build Commands

### Build for current platform
```bash
./build-simple.sh
```

### Build for all platforms
```bash
./build-all-platforms.sh
```

### Development build
```bash
cargo build
```

### Release build
```bash
cargo build --release
```

## Run Commands

### Run a Telugu program
```bash
telc filename.tel
```

### Run with verbose output (shows lexing, parsing, execution steps)
```bash
telc filename.tel --verbose
```

## Testing

Currently no formal test suite. Test by running example programs:
```bash
telc examples/hello.tel
telc examples/calculator.tel
telc examples/functions.tel
```

## Architecture

The interpreter follows a standard compiler pipeline:

1. **Lexer** (src/lexer.rs) - Tokenizes Telugu source code into tokens
2. **Parser** (src/parser.rs) - Parses tokens into an Abstract Syntax Tree (AST)
3. **Interpreter** (src/interpreter.rs) - Executes the AST
4. **AST** (src/ast.rs) - Defines language constructs (expressions, statements)
5. **Token** (src/token.rs) - Token definitions mapping Telugu keywords

### Key Language Features

- Telugu keywords for all programming constructs
- Variables (`sankhya`), functions (`pani`), control flow (`ayite`/`lekapothe`)
- Built-in REST API support (`fetch`, `post`, `put`, `delete`)
- Arrays, string operations, boolean logic
- JSON parsing capabilities

### Adding New Features

When adding language features:
1. Add new tokens in `src/token.rs` if new keywords are needed
2. Update lexer in `src/lexer.rs` to recognize new tokens
3. Add AST nodes in `src/ast.rs` for new constructs
4. Implement parsing logic in `src/parser.rs`
5. Add execution logic in `src/interpreter.rs`
6. Create example in `examples/` to demonstrate usage