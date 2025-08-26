# Contributing to Nova

Thank you for your interest in contributing to the Nova programming language! This document provides guidelines and information for contributors.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Community](#community)

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/nova-lang.git
   cd nova-lang
   ```
3. Set up the development environment (see below)
4. Create a branch for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Setup Commands

```bash
# Install development tools
make dev-setup

# Build the project
make build

# Run tests
make test

# Run benchmarks
make benchmark

# Format code
make format

# Run linter
make lint
```

## How to Contribute

### Bug Reports

When filing a bug report, please include:

- Nova version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Code sample that demonstrates the issue

### Feature Requests

For feature requests:

- Check if it's already requested in Issues
- Explain the use case and benefit
- Provide examples of how it would be used
- Consider implementation complexity

### Pull Requests

1. **Small, focused changes**: Keep PRs focused on a single feature/fix
2. **Write tests**: Add tests for new functionality
3. **Update documentation**: Update relevant docs and examples
4. **Follow code style**: Run `make format` and `make lint`
5. **Write good commit messages**: Use conventional commit format

#### PR Checklist

- [ ] Tests pass (`make test`)
- [ ] Code is formatted (`make format`)
- [ ] Lints pass (`make lint`)
- [ ] Documentation updated
- [ ] Examples added/updated if needed
- [ ] CHANGELOG.md updated for user-facing changes

## Code Style

### Rust Code

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Write meaningful variable and function names
- Add comments for complex logic

### Nova Code (Examples/Tests)

- Use 4 spaces for indentation
- Keep lines under 100 characters
- Add comments for non-obvious code
- Follow existing example patterns

### Naming Conventions

- **Files**: snake_case (e.g., `string_utils.rs`)
- **Functions**: snake_case (e.g., `parse_expression`)
- **Types**: PascalCase (e.g., `TokenType`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `MAX_DEPTH`)

## Testing

### Running Tests

```bash
# Run all Rust tests
cargo test

# Run Nova language tests
make test

# Run specific test file
cargo run tests/core_tests.nova

# Run benchmarks
make benchmark
```

### Writing Tests

#### Rust Tests

Add unit tests in the same file as your code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

#### Nova Tests

Add tests to `tests/core_tests.nova` or create new test files:

```nova
start_test_suite("My Feature");
assert_equal(my_function(input), expected, "Test description");
finish_test_suite();
```

### Test Coverage

- Aim for high test coverage on new code
- Test edge cases and error conditions
- Include integration tests for major features

## Documentation

### Types of Documentation

1. **API Documentation**: Rust doc comments (`///`)
2. **Language Documentation**: Markdown files in `docs/`
3. **Examples**: Nova code in `examples/`
4. **Specifications**: Formal language spec in `spec/`

### Writing Good Documentation

- **Clear and concise**: Explain what, why, and how
- **Include examples**: Show usage patterns
- **Keep it updated**: Update docs with code changes
- **Link related concepts**: Help users navigate

### Documentation Structure

```
docs/
├── website/          # Website source
├── api/             # API documentation  
├── tutorials/       # Learning materials
└── reference/       # Complete reference
```

## Project Structure

```
nova-lang/
├── src/             # Rust source code
│   ├── main.rs      # CLI entry point
│   ├── lib.rs       # Library root
│   ├── lexer.rs     # Tokenization
│   ├── parser.rs    # Parsing
│   ├── ast.rs       # Abstract syntax tree
│   ├── interpreter.rs # Evaluation
│   └── ...
├── examples/        # Nova code examples
├── tests/           # Nova language tests
├── benchmarks/      # Performance tests
├── std/             # Standard library
├── docs/            # Documentation
├── spec/            # Language specification
└── grammars/        # Syntax highlighting
```

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release PR
4. Tag release after merge
5. Publish to package registries

## Code Review Process

### For Contributors

- Respond to feedback promptly
- Make requested changes
- Keep discussions focused and respectful
- Test your changes thoroughly

### For Reviewers

- Be constructive and helpful
- Focus on code quality and correctness
- Suggest improvements
- Approve when satisfied

## Community Guidelines

### Communication

- **Be respectful**: Treat everyone with kindness
- **Be inclusive**: Welcome people of all backgrounds
- **Be patient**: Help newcomers learn
- **Stay on topic**: Keep discussions relevant

### Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct.

## Getting Help

- **GitHub Issues**: Ask questions or report problems
- **GitHub Discussions**: General discussion and ideas
- **Discord**: Real-time community chat (link in README)

## Recognition

Contributors are recognized in:

- Git commit history
- Release notes
- Contributors file
- Project documentation

Thank you for helping make Nova better! 🚀