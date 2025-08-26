# Nova Language Makefile

.PHONY: build test benchmark clean install docs

# Build the Nova interpreter
build:
	cargo build --release

# Run all tests
test: build
	cargo test
	@echo "Running Nova language tests..."
	cargo run --bin nova test_suite/integration/core_tests.nova

# Run performance benchmarks
benchmark: build
	@echo "Running Nova language benchmarks..."
	cargo run --bin nova test_suite/benchmark/performance.nova

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Install Nova globally (Linux/macOS)
install: build
	sudo cp target/release/nova /usr/local/bin/
	@echo "Nova installed to /usr/local/bin/nova"

# Run examples
examples: build
	@echo "Running Hello World example..."
	cargo run --bin nova docs/examples/hello.nova
	@echo ""
	@echo "Running Calculator example..."
	cargo run --bin nova docs/examples/calculator.nova
	@echo ""
	@echo "Running Array example..."
	cargo run --bin nova docs/examples/arrays.nova
	@echo ""
	@echo "Running Collections example..."
	cargo run --bin nova docs/examples/collections.nova
	@echo ""
	@echo "Running DateTime example..."
	cargo run --bin nova docs/examples/datetime.nova
	@echo ""
	@echo "Running Random example..."
	cargo run --bin nova docs/examples/random.nova
	@echo ""
	@echo "Running Crypto example..."
	cargo run --bin nova docs/examples/crypto.nova

# Start REPL
repl: build
	cargo run --bin nova

# Generate documentation
docs:
	cargo doc --open
	@echo "Documentation generated and opened in browser"

# Format code
format:
	cargo fmt

# Lint code
lint:
	cargo clippy

# Run all quality checks
check: format lint test

# Development setup
dev-setup:
	rustup component add rustfmt clippy
	@echo "Development environment set up"

# Package for distribution
package: build
	mkdir -p dist
	cp target/release/nova dist/
	cp -r docs/examples dist/examples
	cp -r nova/stdlib dist/stdlib
	cp README.md LICENSE dist/
	tar -czf nova-lang.tar.gz -C dist .
	@echo "Package created: nova-lang.tar.gz"

# Help
help:
	@echo "Nova Language Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build      - Build the Nova interpreter"
	@echo "  test       - Run all tests"
	@echo "  benchmark  - Run performance benchmarks"
	@echo "  examples   - Run example programs"
	@echo "  repl       - Start interactive REPL"
	@echo "  install    - Install Nova globally"
	@echo "  docs       - Generate and open documentation"
	@echo "  clean      - Clean build artifacts"
	@echo "  format     - Format code"
	@echo "  lint       - Lint code"
	@echo "  check      - Run all quality checks"
	@echo "  package    - Create distribution package"
	@echo "  help       - Show this help"