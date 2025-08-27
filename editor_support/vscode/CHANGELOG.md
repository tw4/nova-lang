# Change Log

All notable changes to the "Nova Language Extension" will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-08-27

### Added

#### Advanced IDE Features
- **Code Formatting**: Automatic code formatting with smart indentation
- **Refactoring Tools**: Symbol renaming and extract function capabilities
- **Project Scaffolding**: Built-in project templates:
  - Hello World starter project
  - CLI application template
  - Web application template
- **Enhanced Symbol Provider**: Workspace-wide symbol search and navigation
- **Advanced Syntax Validation**: Real-time error detection and warnings
- **Bracket Auto-closing**: Enhanced bracket matching for all pairs

#### New Themes
- **Nova Light Theme**: Professional light theme variant
- **Improved Dark Theme**: Enhanced contrast and readability

#### Developer Experience
- **Comprehensive Logging**: Detailed console output for debugging
- **Robust Error Handling**: Better error messages and fallback providers
- **JavaScript Fallback Providers**: Reliable completion when language server is unavailable
- **Performance Optimization**: Reduced extension size and improved startup time

#### Project Management
- **Template System**: Quick project creation with `nova.createProject`
- **Debug Support**: Basic debugging configuration setup
- **Build Integration**: Task providers for common Nova development workflows

### Enhanced

#### Auto-Completion
- **Extended Built-in Functions**: Added more mathematical and utility functions
- **Better Context Awareness**: Improved completion relevance
- **Snippet Expansion**: Enhanced code templates with placeholders
- **Performance**: Faster completion response times

#### Syntax Highlighting
- **Better Token Recognition**: More accurate syntax parsing
- **Enhanced Color Scheme**: Improved readability in both themes
- **String Interpolation**: Better highlighting for embedded expressions

### Fixed
- **Language Server Stability**: Disabled problematic Rust language server, using reliable JavaScript providers
- **Extension Activation**: More robust activation process
- **Memory Usage**: Optimized for better performance
- **File Association**: Improved `.nova` file detection

### Technical Improvements
- **Modular Architecture**: Separated concerns into dedicated modules
- **Better Testing**: Comprehensive test files and validation
- **Documentation**: Improved README and setup instructions
- **Build Process**: Optimized packaging with `.vscodeignore`

## [1.0.0] - 2025-08-26

### Added

#### Syntax Highlighting
- Complete syntax highlighting for Nova language
- Support for all Nova keywords: `class`, `fn`, `let`, `const`, `if`, `else`, `while`, `for`, `in`, `return`, `break`, `continue`, `true`, `false`, `null`, `and`, `or`, `not`, `try`, `catch`, `finally`, `throw`, `import`, `from`, `export`, `extends`, `super`, `this`, `static`, `private`, `public`, `async`, `await`, `as`, `new`, `constructor`
- Number literal highlighting (integer, float, hex, binary, octal)
- String literal highlighting with escape sequences
- String interpolation support (`f"${expression}"` syntax)
- Comment highlighting (line and block comments)
- Operator highlighting (arithmetic, comparison, logical, bitwise, assignment)
- Function and method highlighting
- Class and inheritance highlighting
- Built-in function highlighting

#### Auto-Completion
- **Keywords**: All Nova language keywords with intelligent suggestions
- **Built-in Functions**: Complete set of Nova built-in functions:
  - Core: `print`, `len`, `type`, `str`, `num`
  - Array: `push`, `pop`, `split`, `join`
  - Math: `abs`, `max`, `min`, `sqrt`, `pow`, `floor`, `ceil`, `round`, `random`
- **Code Snippets**: Ready-to-use templates for:
  - Class definitions (`class`, `class-extends`)
  - Function definitions (`fn`)
  - Control flow (`if`, `if-else`, `while`, `for`)
  - Error handling (`try-catch`, `try-catch-finally`)
  - Class members (`constructor`, `method`)
  - Variable declarations (`let`, `const`)
  - Data structures (`array`, `object`)
  - Import/export statements
- **Context-Sensitive Suggestions**: Smart completions based on current context
- **Property Access**: Dot notation completions

#### IntelliSense Features
- **Hover Information**: Detailed documentation for:
  - All keywords with syntax examples
  - Built-in functions with parameter information
  - Usage examples in Nova syntax
- **Go to Definition**: Navigate to function and class definitions within files
- **Bracket Matching**: Auto-closing for `{}`, `[]`, `()`, `""`, `''`
- **Comment Support**: Toggle line and block comments
- **Indentation Rules**: Smart indentation for Nova code blocks

#### Language Configuration
- **File Association**: `.nova` file extension support
- **Bracket Pairs**: Proper bracket matching and auto-closing
- **Word Patterns**: Nova identifier recognition
- **Folding**: Code folding support for blocks and regions
- **Auto-Indentation**: Intelligent indentation rules

#### Theming
- **Nova Dark Theme**: Custom dark theme optimized for Nova syntax
- **Semantic Highlighting**: Enhanced syntax colors for better readability

### Technical Details
- Built from Rust-based Nova compiler token definitions
- Comprehensive TextMate grammar for accurate syntax highlighting
- VS Code API integration for advanced IntelliSense features
- Modular architecture with separate files for different concerns

### File Structure
```
nova-language-support/
├── extension.js              # Main extension logic
├── package.json             # Extension manifest
├── README.md               # Documentation
├── CHANGELOG.md            # This file
├── language-configuration.json  # Language configuration
├── syntaxes/
│   └── nova.tmLanguage.json    # Syntax highlighting rules
├── snippets/
│   └── nova.json              # Code snippets
├── themes/
│   └── nova-dark.json         # Nova dark theme
├── icons/                     # Extension icons (placeholder)
└── test.nova                  # Test file for validation
```

### Requirements
- Visual Studio Code 1.74.0 or higher
- Nova files with `.nova` extension

### Known Limitations
- Advanced language server features not yet implemented
- Limited IntelliSense for user-defined symbols across files
- No debugging support (future enhancement)

---

**Enjoy coding with Nova!** 🌟