# Changelog

All notable changes to the Nova programming language will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Object/Map data structures enhancement
- String interpolation
- Module system and imports
- Enhanced error messages
- Performance optimizations

## [0.2.0] - 2024-08-26

### Added
- Enhanced collections library with comprehensive data structures
- Cross-platform compilation support
- Improved string concatenation with automatic type conversion
- Extended standard library with crypto, datetime, and HTTP modules

### Fixed
- String concatenation with numbers now works automatically
- Compilation errors in collections module resolved
- REPL stability improvements

### Changed
- Updated version from 0.1.0 to 0.2.0
- Improved error handling in binary operations

## [0.1.0] - 2024-01-XX

### Added
- **Core Language Features**
  - Variables with `let` keyword
  - Functions with `fn` keyword
  - Conditional expressions (`if`/`else`)
  - Loop constructs (`while`, `for`-`in`)
  - Comments (line comments with `//`)

- **Data Types**
  - Numbers (64-bit floating point)
  - Strings (UTF-8 encoded)
  - Booleans (`true`, `false`)
  - Arrays (dynamic, mixed-type)
  - Null values
  - Functions as first-class values

- **Operators**
  - Arithmetic: `+`, `-`, `*`, `/`
  - Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
  - Logical: `and`, `or`, `!`
  - Assignment: `=`

- **Built-in Functions**
  - `print(value)` - Output to console
  - `len(collection)` - Get length of strings/arrays
  - `type(value)` - Get type name as string
  - `str(value)` - Convert value to string
  - `num(value)` - Convert value to number
  - `push(array, value)` - Add element to array
  - `pop(array)` - Remove last element from array

- **Language Infrastructure**
  - Lexical analyzer (tokenizer)
  - Recursive descent parser
  - Tree-walking interpreter
  - Dynamic typing with runtime checks
  - Garbage collection (reference counting)
  - Error reporting with descriptive messages

- **Development Tools**
  - Interactive REPL
  - File execution mode
  - Comprehensive test suite
  - Performance benchmarks
  - VS Code syntax highlighting extension
  - TextMate grammar definition

- **Standard Library**
  - `std/math.nova` - Mathematical functions and constants
  - `std/string.nova` - String manipulation utilities
  - `std/array.nova` - Array processing functions

- **Documentation**
  - Complete language specification
  - API documentation
  - Tutorial examples
  - Contributing guidelines
  - Performance benchmarks

- **Examples**
  - Hello World (`examples/hello.nova`)
  - Calculator functions (`examples/calculator.nova`)
  - Fibonacci sequence (`examples/fibonacci.nova`)
  - Conditional logic (`examples/conditions.nova`)
  - Variable usage (`examples/variables.nova`)
  - Array operations (`examples/arrays.nova`)
  - Loop constructs (`examples/loops.nova`)
  - Built-in functions (`examples/built_ins.nova`)
  - Advanced patterns (`examples/advanced.nova`)

### Technical Details
- **Architecture**: Rust-based interpreter
- **Parsing**: Recursive descent parser
- **Execution**: Tree-walking interpreter
- **Memory**: Reference counting garbage collection
- **Platform**: Cross-platform (Windows, macOS, Linux)

### Performance
- Fibonacci(25): ~500ms (baseline benchmark)
- 1000-element array operations: ~50ms
- 1000 function calls: ~10ms
- String processing: ~100ms per 1000 operations

### Known Limitations
- No module system (planned for v0.2)
- No object/map literals (planned for v0.2)
- No string interpolation (planned for v0.2)
- No error handling constructs (planned for v0.3)
- No regular expressions (planned for v0.3)
- No file I/O operations (planned for v0.3)

## Future Roadmap

### Version 0.2.0 (Planned)
- Object/Map data structures
- String interpolation
- Module system and imports
- More built-in functions
- Improved error messages
- Performance optimizations

### Version 0.3.0 (Planned)
- Error handling (try/catch)
- Regular expressions
- File I/O operations
- HTTP client library
- JSON parsing/serialization
- Async/await support

### Version 1.0.0 (Vision)
- Stable language specification
- Comprehensive standard library
- Package manager
- Compilation target (WASM/native)
- IDE integrations
- Production-ready tooling

## Contributing

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines on how to contribute to Nova.

## License

Nova is released under the MIT License. See [LICENSE](LICENSE) for details.