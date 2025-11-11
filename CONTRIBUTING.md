# Contributing to Omniscient

Thank you for your interest in contributing to Omniscient! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs)
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/daneb/omniscient.git
cd omniscient

# Build the project
cargo build

# Run tests
cargo test

# Run with cargo
cargo run -- --help
```

## Code Quality Standards

We maintain high code quality standards:

### Before Submitting

1. **Run tests**: All tests must pass
   ```bash
   cargo test
   ```

2. **Check formatting**: Code must be formatted with rustfmt
   ```bash
   cargo fmt --check
   ```

3. **Run clippy**: No clippy warnings allowed
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Build successfully**: Code must compile without warnings
   ```bash
   cargo build --release
   ```

## Project Structure

```
omniscient/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── capture.rs       # Command capture logic
│   ├── category.rs      # Command categorization
│   ├── config.rs        # Configuration management
│   ├── error.rs         # Error types
│   ├── export.rs        # Export/import functionality
│   ├── models.rs        # Data models
│   ├── redact.rs        # Privacy/redaction
│   ├── shell.rs         # Shell integration
│   └── storage.rs       # Database operations
├── examples/            # Example configurations
├── tests/              # Integration tests
└── Cargo.toml         # Dependencies and metadata
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Write Tests

- Add unit tests for new functionality
- Ensure test coverage remains > 80%
- Test edge cases and error conditions

### 3. Update Documentation

- Add/update doc comments for public APIs
- Update README if adding user-facing features
- Update CHANGELOG.md with your changes

### 4. Commit Guidelines

Use conventional commit messages:

```
feat: Add bash shell support
fix: Correct duplicate detection logic
docs: Update installation instructions
test: Add tests for export functionality
refactor: Simplify categorization logic
```

### 5. Submit Pull Request

- Ensure all tests pass
- Update CHANGELOG.md
- Provide clear description of changes
- Reference related issues

## Code Style

### Rust Guidelines

- Follow Rust naming conventions
- Use descriptive variable names
- Add doc comments to public APIs
- Avoid `unwrap()` in production code
- Use proper error handling with `Result<T>`

### Example

```rust
/// Categorize a command based on its first word
///
/// # Arguments
/// * `command` - The full command string to categorize
///
/// # Returns
/// * Category name as a string
pub fn categorize(&self, command: &str) -> String {
    // Implementation
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test";

        // Act
        let result = function(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

## Adding Features

### New Command Categories

1. Add pattern to `src/category.rs`
2. Add tests for the new category
3. Update documentation

### New CLI Commands

1. Add command variant to `Commands` enum in `src/main.rs`
2. Implement command logic
3. Add tests
4. Update help text

### New Shell Support

1. Create shell-specific hook in `src/shell.rs`
2. Add tests for hook generation
3. Add examples in `examples/`
4. Update README

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Build release binary
5. Create git tag
6. Push tag to trigger release

## Getting Help

- Check existing issues and pull requests
- Ask questions in discussions
- Read the documentation
- Join our community chat (if available)

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Help others learn and grow
- Follow GitHub's community guidelines

## License

By contributing to Omniscient, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Omniscient! 🎉
