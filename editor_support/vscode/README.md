# Nova Language Support for VS Code

A comprehensive Visual Studio Code extension providing advanced language support for the Nova programming language with full Language Server Protocol (LSP) integration, real-time diagnostics, intelligent code completion, and debugging capabilities.

## Features

### 🌟 Language Server Protocol (LSP)
- **Native Rust LSP Server**: High-performance language server built with Rust using Nova's own compiler
- **Real-time Analysis**: Instant syntax checking and error reporting
- **Intelligent Parsing**: Uses Nova's native lexer and parser for accurate analysis
- **Auto-rebuild**: Automatically builds LSP server when needed

### ✨ Advanced Syntax Highlighting  
- Complete syntax highlighting for Nova language constructs
- Support for keywords, operators, strings, numbers, and comments
- Special highlighting for classes, functions, and built-in functions
- String interpolation support with `f"${expression}"` syntax
- Semantic highlighting for better code readability

### 🚀 Intelligent Auto-Completion
- **Context-Aware Suggestions**: Smart completions based on current code context
- **Keywords**: `class`, `fn`, `if`, `else`, `while`, `for`, `let`, `const`, etc.
- **Built-in Functions**: Complete Nova standard library with signatures and documentation
- **Code Snippets**: Advanced templates for classes, functions, loops, and more
- **Symbol Navigation**: IntelliSense for user-defined functions and variables

### 🔍 Advanced IntelliSense Features
- **Real-time Diagnostics**: Instant error detection and reporting
- **Hover Information**: Detailed documentation with code examples
- **Go to Definition**: Navigate to function and class definitions across files
- **Document Symbols**: Outline view with functions and classes
- **Code Lens**: Run buttons for executable code
- **Smart Formatting**: Automatic code formatting with proper indentation

### 📝 Code Snippets
- `class` - Create a new class with constructor
- `class-extends` - Create a class that extends another class  
- `fn` - Create a function
- `if`, `if-else` - Conditional statements
- `while`, `for` - Loops
- `try-catch` - Error handling
- `constructor` - Class constructor
- `method` - Class method
- And many more...

## Nova Language Overview

Nova is a modern programming language implemented in Rust, designed for simplicity and expressiveness.

### Basic Example
```nova
// Variables and basic operations
let name = "Nova";
let version = 1.0;
let message = "Hello " + name + " " + str(version) + "!";
print(message);

// Functions
fn greet(person) {
    return "Hello, " + person + "!";
}

print(greet("World"));
```

### Classes and Objects
```nova
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
    
    fn introduce() {
        return "I'm " + this.name + ", " + str(this.age) + " years old";
    }
}

class Student extends Person {
    constructor(name, age, school) {
        super(name, age);
        this.school = school;
    }
    
    fn study() {
        return this.name + " is studying at " + this.school;
    }
}

let student = new Student("Alice", 20, "Nova University");
print(student.introduce());
print(student.study());
```

### Built-in Functions
```nova
// String operations
let text = "Hello World";
print(len(text));        // 11
print(type(text));       // "string"

// Array operations
let numbers = [1, 2, 3];
let moreNumbers = push(numbers, 4);
print(moreNumbers);      // [1, 2, 3, 4]
print(len(moreNumbers)); // 4

// Math operations
print(abs(-5));          // 5
print(max(10, 20));      // 20
print(sqrt(16));         // 4
print(pow(2, 3));        // 8
```

### 🐛 Debugging & Testing
- **Run Nova Files**: Execute Nova programs directly from VS Code
- **Integrated Terminal**: Run and debug in VS Code's integrated terminal  
- **Task Integration**: Build, test, and run tasks
- **Debug Configuration**: Pre-configured debug setups
- **Error Highlighting**: Visual error indicators in editor

### 🛠️ Development Tools
- **Language Server Management**: Start, stop, and restart LSP server
- **Build Integration**: Compile Nova projects and LSP server
- **Status Bar**: Real-time LSP server status
- **Output Channels**: Detailed logging for troubleshooting
- **Command Palette**: Quick access to all Nova commands

## Installation

### From VS Code Marketplace (Coming Soon)
1. Open VS Code
2. Go to Extensions (`Ctrl+Shift+X` or `Cmd+Shift+X`)
3. Search for "Nova Language Support"
4. Click Install

### From VSIX File
1. Download the `.vsix` file from releases
2. Open VS Code
3. Run "Extensions: Install from VSIX..." command (Ctrl+Shift+P)
4. Select the downloaded file

### Building from Source
1. Clone this repository
2. Navigate to `editor_support/vscode`
3. Install dependencies: `npm install`
4. Build LSP server: `npm run build-server`
5. Package extension: `npm run package`

## Requirements

### System Requirements
- Visual Studio Code version 1.74.0 or higher
- Rust toolchain (for LSP server compilation)
- Nova language runtime

### Automatic Setup
The extension will automatically:
- Detect if the LSP server needs to be built
- Provide commands to build the server
- Fall back to basic features if LSP is unavailable

## Extension Commands

Access these commands via Command Palette (`Ctrl+Shift+P`):

- `Nova: Restart Language Server` - Restart the LSP server
- `Nova: Show Language Server Output` - View LSP server logs  
- `Nova: Build Language Server` - Compile the Rust LSP server
- `Nova: Run Nova File` - Execute the current Nova file

## Configuration

The extension provides these settings in VS Code settings:

```json
{
    "nova.languageServer.enabled": true,
    "nova.languageServer.trace": "off",
    "nova.formatting.enabled": true,
    "nova.diagnostics.enabled": true
}
```

## Known Issues

- Advanced language server features are not yet implemented
- Limited IntelliSense for user-defined functions and classes

## Release Notes

### 1.0.0

Initial release of Nova Language Support:
- Complete syntax highlighting
- Auto-completion for keywords and built-ins
- Code snippets
- Basic hover information
- Go to definition for functions and classes

## Contributing

Contributions are welcome! Please visit our [GitHub repository](https://github.com/tw4/nova-lang) to:
- Report issues
- Suggest features
- Submit pull requests

## License

This extension is licensed under the MIT License. See the LICENSE file for details.

---

**Enjoy coding with Nova!** 🌟