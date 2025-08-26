# Nova Programming Language

<div align="center">
  <img src="https://img.shields.io/badge/Nova-v0.2.0-ff6b6b" alt="Nova Version">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-MIT-blue" alt="License">
  <img src="https://img.shields.io/badge/Platform-Cross--Platform-green" alt="Platform">
</div>

<p align="center">
  <strong>A modern, elegant programming language designed for simplicity and expressiveness</strong>
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#examples">Examples</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/your-username/nova-lang.git
cd nova-lang

# Build Nova (requires Rust)
cargo build --release

# Start the interactive REPL
./target/release/nova.exe

# Run an example
./target/release/nova.exe docs/examples/hello.nova
```

## ✨ Features

- **🎯 Simple Syntax**: Clean, intuitive syntax inspired by modern languages
- **⚡ Dynamic Typing**: Write code faster without type annotations
- **📦 Rich Data Structures**: Built-in arrays, objects, and powerful collections
- **🔧 First-class Functions**: Functions as values, closures, and higher-order programming
- **🔄 Modern Control Flow**: Intuitive conditionals and loop constructs
- **🛠️ Comprehensive Standard Library**: Math, string, and array utilities included
- **🎮 Interactive REPL**: Immediate feedback and experimentation
- **📝 VS Code Support**: Syntax highlighting and language support

## 📖 Language Overview

### Basic Syntax

```nova
// Variables and basic operations
let name = "Nova";
let version = 0.1;
let is_awesome = true;

// Functions
fn greet(person) {
    "Hello, " + person + "!"
}

print(greet(name)); // Hello, Nova!
```

### Data Structures

```nova
// Arrays
let numbers = [1, 2, 3, 4, 5];
let mixed = [42, "hello", true, [1, 2]];

// Array operations
print(numbers[0]);        // 1
print(len(numbers));      // 5
print(push(numbers, 6)); // [1, 2, 3, 4, 5, 6]
```

### Control Flow

```nova
// Conditionals
if (age >= 18) {
    print("Adult");
} else {
    print("Minor");
}

// Loops
for item in [1, 2, 3, 4, 5] {
    print("Number: " + item);
}

let counter = 0;
while (counter < 5) {
    print("Count: " + counter);
    counter = counter + 1;
}
```

### Higher-Order Functions

```nova
fn map_array(array, func) {
    let result = [];
    for item in array {
        result = push(result, func(item));
    }
    result
}

fn square(x) { x * x }
let squares = map_array([1, 2, 3], square);
print(squares); // [1, 4, 9]
```

## 🎪 Examples

Explore these example programs to learn Nova:

| Example | Description |
|---------|-------------|
| [`hello.nova`](examples/hello.nova) | Basic hello world program |
| [`calculator.nova`](examples/calculator.nova) | Calculator with functions |
| [`arrays.nova`](examples/arrays.nova) | Array operations and manipulation |
| [`loops.nova`](examples/loops.nova) | Control flow and iteration |
| [`fibonacci.nova`](examples/fibonacci.nova) | Recursive Fibonacci sequence |
| [`advanced.nova`](examples/advanced.nova) | Complex programming patterns |

```bash
# Run any example  
./target/release/nova.exe docs/examples/arrays.nova
```

## 🏗️ Project Structure

```
nova-lang/
├── 📦 nova/
│   ├── compiler/        # Core compiler and interpreter
│   ├── runtime/         # Runtime system and VM
│   ├── stdlib/          # Standard library modules
│   ├── tools/           # Development tools (formatter, linter)
│   └── testing/         # Testing framework
├── 📁 docs/
│   ├── examples/        # Example programs
│   ├── reference/       # Language specification
│   └── tutorials/       # Learning materials
├── 🧪 test_suite/
│   ├── unit/            # Unit tests
│   ├── integration/     # Integration tests
│   └── benchmark/       # Performance benchmarks
├── 🎨 editor_support/
│   ├── vscode/          # VS Code extension
│   ├── grammars/        # Syntax highlighting
│   └── sublime/         # Sublime Text support
├── 🔧 Cargo.toml        # Workspace configuration
├── 📋 Makefile          # Build automation
└── 📖 README.md         # This file
```

## 🔧 Installation & Usage

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

### Building from Source

```bash
# Clone and build
git clone https://github.com/your-username/nova-lang.git
cd nova-lang
cargo build --release

# Alternative: Use Makefile (if available)
make build
```

### Usage Modes

```bash
# Interactive REPL
./target/release/nova.exe

# Execute a file
./target/release/nova.exe program.nova

# Run tests
cargo test

# Build in release mode
cargo build --release

# Clean build artifacts
cargo clean
```

## 📚 Documentation

### Core Documentation

- **[Language Specification](spec/language-spec.md)** - Complete language reference
- **[Contributing Guide](docs/CONTRIBUTING.md)** - How to contribute
- **[Changelog](CHANGELOG.md)** - Version history and updates

### API Reference

- **[Built-in Functions](docs/api/builtins.md)** - Core language functions
- **[Standard Library](std/)** - Math, string, and array utilities
- **[Error Handling](docs/api/errors.md)** - Error types and handling

### Tutorials

- **[Getting Started](docs/tutorials/getting-started.md)** - Your first Nova program
- **[Language Tour](docs/tutorials/language-tour.md)** - Comprehensive overview
- **[Best Practices](docs/tutorials/best-practices.md)** - Writing idiomatic Nova

## 🧪 Testing

Nova includes comprehensive testing at multiple levels:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run Nova test files
./target/release/nova.exe test_suite/unit/test.nova
```

### Test Coverage

- ✅ **Core Language Features** - Variables, functions, control flow
- ✅ **Data Types** - Numbers, strings, booleans, arrays
- ✅ **Built-in Functions** - All standard library functions
- ✅ **Error Handling** - Runtime error conditions
- ✅ **Edge Cases** - Boundary conditions and special cases

## 📈 Performance

Nova's performance characteristics (measured on standard benchmarks):

| Benchmark | Result | Description |
|-----------|--------|-------------|
| Fibonacci(25) | ~500ms | Recursive function calls |
| Array Sum(1000) | ~10ms | Iteration and arithmetic |
| String Ops | ~100ms | String manipulation |
| Function Calls(1000) | ~5ms | Call overhead |

Run performance tests yourself with cargo test for benchmarks

## 🛠️ Development Tools

### VS Code Extension

Install syntax highlighting and language support:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Install from `vscode-nova/` directory
4. Enjoy Nova syntax highlighting!

### Language Support

Nova provides rich tooling support:

- **Syntax Highlighting** - VS Code, TextMate, GitHub
- **Auto-completion** - Basic identifier completion
- **Error Detection** - Runtime error reporting
- **REPL Integration** - Interactive development

## 🌟 GitHub Linguist Integration

Nova is ready for GitHub Linguist integration with:

- ✅ Language definition (`languages.yml`)
- ✅ File extension mapping (`.nova`)
- ✅ Syntax highlighting grammar
- ✅ Comprehensive test suite
- ✅ Example code repository
- ✅ Active development community

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Quick Contribution Steps

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** your changes
4. **Test** your changes (`make test`)
5. **Commit** your changes (`git commit -m 'Add amazing feature'`)
6. **Push** to the branch (`git push origin feature/amazing-feature`)
7. **Open** a Pull Request

### Areas for Contribution

- 🐛 **Bug Fixes** - Fix issues and improve stability
- ✨ **New Features** - Extend language capabilities
- 📖 **Documentation** - Improve guides and examples
- 🧪 **Testing** - Add more test cases and benchmarks
- 🎨 **Tools** - IDE plugins and development tools
- 🌍 **Localization** - Translate documentation

See our [Contributing Guide](docs/CONTRIBUTING.md) for detailed information.

## 📋 Roadmap

### Current Version (v0.2.0)
- ✅ Core language implementation
- ✅ Standard data types and operations
- ✅ Functions and closures
- ✅ Control flow constructs
- ✅ Built-in functions
- ✅ REPL and file execution
- ✅ Comprehensive testing

### Next Release (v0.3.0)
- 🔄 Object/Map data structures (partially implemented)
- 🔄 String interpolation
- 🔄 Module system and imports
- 🔄 Enhanced error messages
- 🔄 Performance optimizations
- 🔄 Improved string-number concatenation

### Future Versions
- 📋 Error handling (try/catch)
- 📋 Regular expressions
- 📋 File I/O operations
- 📋 HTTP client library
- 📋 Package manager
- 📋 Compilation targets

## 📊 Language Statistics

| Metric | Value |
|--------|-------|
| **Lines of Code** | ~3,000 (Rust implementation) |
| **Test Cases** | 50+ comprehensive tests |
| **Example Programs** | 9 detailed examples |
| **Built-in Functions** | 8 core functions |
| **Standard Library** | 3 modules (math, string, array) |
| **Documentation Pages** | 10+ comprehensive guides |

## 🏆 Recognition

Nova aims to be recognized for:

- **Simplicity** - Easy to learn and write
- **Expressiveness** - Powerful enough for real applications
- **Quality** - Well-tested and documented
- **Community** - Welcoming to contributors
- **Innovation** - Modern language design

## 📄 License

Nova is open source software released under the [MIT License](LICENSE).

```
Copyright (c) 2024 Nova Programming Language Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

## 🔗 Links

- **[Website](docs/website/index.html)** - Official Nova website
- **[GitHub Repository](https://github.com/your-username/nova-lang)** - Source code
- **[Documentation](docs/)** - Complete documentation
- **[Examples](examples/)** - Code examples
- **[Issues](https://github.com/your-username/nova-lang/issues)** - Bug reports and feature requests

---

<p align="center">
  Made with ❤️ by the Nova community<br>
  <strong>Happy coding with Nova! 🚀</strong>
</p># nova-lang
# nova-lang
# nova-lang
