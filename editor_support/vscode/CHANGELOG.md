# Nova Language Extension Changelog

## [0.3.0] - 2024-08-26

### 🎉 Major Features Added
- **Class System Support**: Full object-oriented programming support
  - Class definitions with `class` keyword
  - Constructor methods with `constructor` keyword
  - Method definitions with visibility modifiers (`private`, `public`)
  - Inheritance support with `extends` keyword
  - Instance creation with `new` keyword
  - Static methods with `static` keyword
  - Access to `this`, `super` in methods

### 🔧 Language Server Enhancements
- **New Keywords Auto-completion**:
  - `class`, `extends`, `super`, `this`
  - `constructor`, `private`, `public`, `static`
  - `new` for object instantiation
- **Enhanced Syntax Highlighting**: All OOP keywords properly highlighted
- **Improved IntelliSense**: Better code suggestions for class-based code

### 📝 Examples and Documentation
- Added comprehensive class examples in Nova codebase
- Updated test suite with class functionality tests
- Enhanced hover documentation for new keywords

### 🐛 Bug Fixes
- Improved error detection for class syntax
- Better handling of method calls and property access
- Enhanced diagnostics for OOP-related errors

## [0.2.0] - 2024-08-26

### 🚀 Language Server Protocol (LSP) Support
- **Auto-completion** for Nova keywords and built-in functions
- **Hover documentation** for function signatures and descriptions
- **Real-time error checking** and diagnostics
- **Enhanced syntax highlighting** with LSP integration

### 📦 Built-in Functions Support
- Enhanced hover support for: `print()`, `len()`, `type()`, `str()`, `num()`
- Array functions: `push()`, `pop()`, `split()`, `join()`
- Comprehensive documentation for all functions

## [0.1.0] - 2024-08-26

### 🎯 Initial Release
- **Basic syntax highlighting** for Nova programming language
- **File association** for `.nova` files
- **Language configuration** with auto-indentation
- **Bracket matching** and comment support
- **TextMate grammar** for syntax coloring

### 📋 Supported Features
- Keywords: `let`, `fn`, `if`, `else`, `while`, `for`, `in`, `and`, `or`, `null`, `true`, `false`
- Operators: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `=`
- Built-in functions: `print`, `len`, `type`, `str`, `num`, `push`, `pop`
- String literals with escape sequence support
- Numeric literals (integers and floats)
- Line comments with `//` and block comments `/* */`
- Automatic bracket and quote pairing