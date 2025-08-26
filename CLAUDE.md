# Nova Programming Language - Development Guide

This document provides development and build instructions for the Nova programming language.

## Project Status

✅ **Ready for First Release (v0.2.0)**
- Core language functionality implemented and tested
- Cross-platform compilation working
- String concatenation with automatic type conversion implemented
- Examples working correctly
- Documentation up to date

## Build System

### Prerequisites
- Rust 1.70+ (https://rustup.rs/)
- Git

### Build Commands
```bash
# Build in release mode
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run examples
./target/release/nova.exe docs/examples/hello.nova

# Start REPL
./target/release/nova.exe
```

### Makefile Targets
```bash
make build      # Build in release mode
make test       # Run all tests
make examples   # Run all example programs
make repl       # Start REPL
make clean      # Clean build artifacts
make package    # Create distribution package
```

## Project Structure

```
nova-lang/
├── nova/
│   ├── compiler/        # Core compiler and interpreter
│   ├── runtime/         # Runtime system and VM
│   ├── stdlib/          # Standard library modules
│   ├── tools/           # Development tools
│   └── testing/         # Testing framework
├── docs/examples/       # Example programs
├── test_suite/          # Test files
├── editor_support/      # IDE support files
├── target/              # Build artifacts
├── Cargo.toml           # Workspace configuration
└── Makefile            # Build automation
```

## Key Components

### Compiler (`nova/compiler/`)
- `lexer.rs` - Tokenization
- `parser.rs` - Syntax analysis
- `interpreter.rs` - Execution engine
- `ast.rs` - Abstract syntax tree definitions
- `value.rs` - Runtime value types

### Standard Library (`nova/stdlib/`)
- Collections, math, string, crypto, datetime, HTTP modules
- Native function implementations in Rust

### Testing
- Rust unit tests in `src/tests.rs`
- Nova language tests in `test_suite/`
- Example programs serve as integration tests

## Recent Changes (v0.2.0)

1. **Fixed String Concatenation**: Added automatic type conversion for `+` operator
2. **Collections Library**: Fixed compilation errors and added proper trait derives
3. **Cross-platform**: Confirmed working on Windows
4. **Examples**: All examples now run correctly

## Development Tips

### Running Individual Tests
```bash
# Run specific Rust test
cargo test test_variables

# Run Nova language file
./target/release/nova.exe test_suite/unit/test.nova
```

### Adding New Features
1. Update AST definitions in `ast.rs`
2. Add lexer support in `lexer.rs` 
3. Add parser support in `parser.rs`
4. Implement in interpreter in `interpreter.rs`
5. Add tests
6. Update documentation

### Common Issues
- **REPL hangs**: This is a known issue, use Ctrl+C to exit
- **String + Number errors**: Fixed in v0.2.0 with automatic conversion
- **Path issues**: Use `./target/release/nova.exe` on Windows

## Testing Status

✅ Unit Tests: 21/22 passing (1 recursive function test failing)
✅ Basic Examples: Working
✅ String Operations: Working
✅ Math Operations: Working
✅ Arrays: Working
✅ Control Flow: Working
✅ Functions: Working (except some recursive cases)

## Release Readiness

The Nova programming language v0.2.0 is ready for first release:

- [x] Compiles successfully on Windows
- [x] Core functionality working
- [x] Examples demonstrate language features
- [x] Documentation updated
- [x] Version numbers updated
- [x] CHANGELOG updated
- [x] Critical bugs fixed

## Future Development

### Immediate Priorities (v0.3.0)
- Fix recursive function scoping issue
- Improve REPL stability
- Add more built-in functions
- Enhanced error messages

### Long-term Goals
- Module system
- Object/Map literals
- String interpolation
- File I/O
- Package manager

## Commands for Development

### Test All Examples
```bash
for file in docs/examples/*.nova; do
    echo "Testing $file"
    ./target/release/nova.exe "$file"
    echo "---"
done
```

### Release Build
```bash
cargo build --release
make package  # Creates distribution package
```

### Format and Lint
```bash
cargo fmt      # Format code
cargo clippy   # Lint code
```

This document should be updated as the project evolves.