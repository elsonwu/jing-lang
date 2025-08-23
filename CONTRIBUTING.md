# Contributing to Jing

Thank you for your interest in contributing to Jing! This guide will help you get started.

## 🚀 Quick Start

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Write tests for your changes
5. Ensure all tests pass: `cargo test`
6. Format your code: `cargo fmt`
7. Lint your code: `cargo clippy`
8. Commit your changes using conventional commits (see below)
9. Push to your fork and create a pull request

## 📝 Conventional Commits

We use [Conventional Commits](https://conventionalcommits.org/) for automatic changelog generation and semantic versioning.

### Commit Message Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **build**: Changes that affect the build system or external dependencies
- **ci**: Changes to CI configuration files and scripts
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit

### Examples

```bash
# Feature
feat(lexer): add support for hexadecimal numbers

# Bug fix
fix(parser): handle empty function parameter list correctly

# Breaking change
feat(vm)!: redesign stack-based execution model

BREAKING CHANGE: The VM now uses a different bytecode format
```

### Scope Guidelines

Common scopes for this project:
- `lexer`: Changes to tokenization
- `parser`: Changes to AST generation
- `compiler`: Changes to bytecode generation
- `vm`: Changes to virtual machine execution
- `value`: Changes to value types and operations
- `error`: Changes to error handling
- `cli`: Changes to command-line interface
- `docs`: Documentation changes
- `tests`: Test-related changes

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test lexer_tests

# Run tests and check coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Test Guidelines

1. **Unit Tests**: Add tests for individual functions/methods
2. **Integration Tests**: Add end-to-end tests for language features
3. **Error Cases**: Test error conditions and edge cases
4. **Documentation**: Update documentation for new features

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = "test input";
        
        // Act  
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

## 🎯 Code Style

### Formatting

We use `rustfmt` with custom configuration:

```bash
cargo fmt
```

### Linting

We use `clippy` with strict settings:

```bash
cargo clippy -- -D warnings
```

### Code Guidelines

1. **Error Handling**: Use `Result<T, JingError>` for all fallible operations
2. **Documentation**: Document public APIs with examples
3. **Naming**: Use descriptive names, avoid abbreviations
4. **Complexity**: Keep functions focused and simple
5. **Testing**: Write tests for new functionality

## 📚 Project Structure

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library exports  
├── lexer.rs         # Tokenization
├── parser.rs        # AST construction
├── compiler.rs      # Bytecode generation
├── vm.rs           # Virtual machine
├── value.rs        # Value types
└── error.rs        # Error handling

tests/
├── integration_tests.rs  # End-to-end tests
├── lexer_tests.rs        # Lexer unit tests
├── parser_tests.rs       # Parser unit tests
└── ...

examples/
├── hello.jing           # Example programs
└── ...
```

## 🔄 Release Process

Releases are automated using release-please:

1. **Make Changes**: Use conventional commits
2. **Merge to Main**: Release-please creates a release PR
3. **Review & Merge**: The release PR updates version and changelog
4. **Automatic Release**: GitHub releases are created with built binaries

## 📋 Pull Request Guidelines

### PR Title Format

Use conventional commit format for PR titles:

```
feat(parser): add support for array literals
fix(vm): resolve stack overflow in recursive calls
docs: update installation instructions
```

### PR Description

Include:
- **Description**: What does this PR do?
- **Motivation**: Why is this change needed?
- **Testing**: How was this tested?
- **Breaking Changes**: Any breaking changes?
- **Related Issues**: Link to related issues

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] Code is linted (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Changelog entry is appropriate (automatic via conventional commits)
- [ ] Breaking changes are documented

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Reproduction**: Steps to reproduce the bug
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**: Rust version, OS, etc.
6. **Code Sample**: Minimal example that demonstrates the issue

## 💡 Feature Requests

For feature requests, please include:

1. **Description**: What feature would you like?
2. **Motivation**: Why is this feature needed?
3. **Use Cases**: How would you use this feature?
4. **Implementation**: Any ideas for implementation?

## ❓ Getting Help

- **Issues**: Create an issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check the README and inline documentation

## 🎉 Recognition

Contributors will be recognized in:
- Release notes
- Contributors list
- Git history

Thank you for contributing to Jing! 🚀
