# Contributing to A-lang

Thank you for your interest in contributing to A-lang! 🎉

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)

## Code of Conduct

This project adheres to our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/a-lang.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Submit a pull request

## How to Contribute

### 🐛 Reporting Bugs

- Use GitHub Issues
- Include A-lang version (`alang --version`)
- Provide minimal reproduction code
- Describe expected vs actual behavior
- Include error messages

### 💡 Suggesting Features

- Open a GitHub Discussion first
- Explain the use case
- Provide examples
- Consider backwards compatibility

### 📝 Improving Documentation

- Fix typos and unclear wording
- Add examples
- Improve code comments
- Translate documentation

### 🔧 Code Contributions

Areas where we need help:
- FFI improvements (more type signatures)
- Standard library functions
- Performance optimizations
- Platform support (Windows FFI)
- Bug fixes

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code recommended)

### Build from Source

```bash
# Clone
git clone https://github.com/A-The-Programming-Language/a-lang.git
cd a-lang

# Build
cargo build

# Run tests
cargo test

# Run REPL
cargo run

# Run example
cargo run examples/hello.al
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests
cargo test --test '*'
```

### Adding Tests

- Unit tests: Add to the same file as the code
- Integration tests: Add to `tests/` directory
- Example tests: Ensure examples in `examples/` work

### Test Coverage

We aim for 90%+ test coverage. Please add tests for:
- New features
- Bug fixes
- Edge cases

## Pull Request Process

1. **Update Tests**: Add or update tests for your changes
2. **Update Documentation**: Update README, comments, or docs as needed
3. **Run Tests**: Ensure all tests pass (`cargo test`)
4. **Format Code**: Run `cargo fmt`
5. **Lint**: Run `cargo clippy`
6. **Write Clear Commit Messages**: Use conventional commits format
7. **Submit PR**: Fill out the PR template completely

### Commit Message Format

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

Examples:
```
feat(ffi): add support for struct types
fix(parser): handle empty array literals
docs(readme): update installation instructions
```

## Coding Standards

### Rust Code

- Follow Rust style guide
- Use `cargo fmt` for formatting
- Fix all `cargo clippy` warnings
- Add documentation comments (`///`) for public APIs
- Keep functions small and focused
- Use descriptive variable names

### A-lang Code (Examples)

- Use JavaScript-style syntax
- Include comments for complex logic
- Follow naming conventions:
  - Variables: `camelCase`
  - Functions: `camelCase`
  - Constants: `UPPER_CASE`

### Documentation

- Use clear, concise language
- Include code examples
- Update table of contents
- Check for spelling/grammar

## Project Structure

```
a-lang/
├── src/
│   ├── main.rs           # REPL entry point
│   ├── lib.rs            # Library root
│   ├── lexer/            # Tokenizer
│   ├── parser/           # AST parser
│   ├── interpreter/      # Execution engine
│   ├── reactive/         # Reactive system
│   ├── time_travel/      # Time-travel debugger
│   ├── stdlib/           # Standard library
│   └── ...
├── examples/             # Example scripts
├── tests/                # Integration tests
└── docs/                 # Documentation

```

## Review Process

1. **Automatic Checks**: CI runs tests and linters
2. **Code Review**: Maintainers review your code
3. **Feedback**: Address review comments
4. **Approval**: At least one maintainer approval needed
5. **Merge**: We'll merge your PR!

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Chat**: Join our community (link TBD)
- **Email**: dev@alang.dev

## Recognition

Contributors will be:
- Added to CONTRIBUTORS.md
- Mentioned in release notes
- Listed on the website (coming soon)

## License

By contributing, you agree that your contributions will be licensed under both:
- MIT License (LICENSE-MIT)
- Apache License 2.0 (LICENSE-APACHE)

---

Thank you for contributing to A-lang! 🚀
