# Nova Language Support for VS Code

Syntax highlighting and language support for the Nova programming language.

## Features

- **Syntax Highlighting**: Full syntax highlighting for Nova language files (.nova)
- **Language Configuration**: Auto-indentation and bracket matching
- **File Association**: Automatic recognition of .nova files

## Installation

### From VS Code Marketplace

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Nova Language Support"
4. Click Install

### Manual Installation

1. Download the extension package (.vsix file)
2. Open VS Code
3. Go to Extensions (Ctrl+Shift+X)
4. Click "..." menu and select "Install from VSIX..."
5. Select the downloaded .vsix file

## Usage

Once installed, the extension will automatically provide syntax highlighting for all `.nova` files.

## Nova Language Features

Nova is a modern programming language with:

- Variables and functions
- Arrays and dynamic typing
- Control flow (if/else, loops)
- Built-in functions
- Interactive REPL

## Example Nova Code

```nova
// Hello World in Nova
let greeting = "Hello, World!";
print(greeting);

// Function definition
fn factorial(n) {
    if (n <= 1) {
        1
    } else {
        n * factorial(n - 1)
    }
}

print("5! = " + factorial(5));
```

## Repository

Find the Nova language source code and documentation at:
https://github.com/tw4/nova-lang

## License

MIT License