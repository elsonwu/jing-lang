# Development Guide for Jing Language

This guide covers the development workflow, Git hooks, and quality assurance practices for contributing to the Jing programming language.

## 🛠️ Development Environment Setup

### Prerequisites

- Rust toolchain (rustc, cargo, rustfmt, clippy)
- Git
- Make (optional, but recommended)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/elsonwu/jing-lang.git
cd jing-lang

# Set up development environment
make dev-setup

# Or manually install Rust components
rustup component add rustfmt clippy
```

## 🔄 Development Workflow

### Quick Development Cycle

```bash
# Format, check, and test in one command
make dev

# Or run individual commands
make format    # Format code
make check     # Check compilation
make lint      # Run clippy
make test      # Run tests
```

### Full CI Pipeline (Local)

```bash
# Run the complete CI pipeline locally
make ci

# This runs: format + check + lint + test + build
```

## 🪝 Git Hooks

The repository includes automatic Git hooks that ensure code quality before commits and pushes.

### Pre-commit Hook

**Location**: `.git/hooks/pre-commit`

**Runs automatically** before every `git commit` and checks:

1. ✅ Code formatting (`cargo fmt --check`)
2. ✅ Clippy lints (`cargo clippy -- -D warnings`)
3. ✅ Code compilation (`cargo check`)
4. ✅ All tests (`cargo test`)

If any check fails, the commit is **blocked** until issues are fixed.

### Pre-push Hook

**Location**: `.git/hooks/pre-push`

**Runs automatically** before every `git push` and performs additional checks:

1. ✅ Code formatting
2. ✅ Clippy lints (all features)
3. ✅ Release build (`cargo build --release`)
4. ✅ All tests (all features)
5. ✅ Documentation tests (`cargo test --doc`)

### Manual Hook Execution

You can run the hooks manually for testing:

```bash
# Test pre-commit hook
.git/hooks/pre-commit

# Test pre-push hook
.git/hooks/pre-push
```

### Bypassing Hooks (Not Recommended)

In emergency situations, you can bypass hooks:

```bash
# Skip pre-commit hook (NOT RECOMMENDED)
git commit --no-verify -m "Emergency commit"

# Skip pre-push hook (NOT RECOMMENDED)
git push --no-verify
```

**⚠️ Warning**: Bypassing hooks can break CI and should only be done in emergencies.

## 📋 Code Quality Standards

### Formatting

- All code must be formatted with `cargo fmt`
- Use default rustfmt settings (configured in `rustfmt.toml`)
- Line length: 100 characters (rustfmt default)

### Linting

- All clippy warnings must be addressed
- Use `#[allow(clippy::lint_name)]` sparingly and with justification
- Prefer fixing the issue over suppressing the warning

### Testing

- All new features must include tests
- Maintain or improve test coverage
- Run `cargo test` before committing
- Document test cases with clear names and comments

### Documentation

- Public APIs must be documented
- Use `///` for public item documentation
- Include examples in documentation when helpful
- Run `cargo test --doc` to verify documentation examples

## 🚫 Common Issues and Solutions

### "Code formatting check failed"

```bash
# Fix formatting issues
cargo fmt
git add -A
git commit
```

### "Clippy found issues"

```bash
# See specific clippy warnings
cargo clippy

# Fix the reported issues
# Then commit again
git commit
```

### "Tests failed"

```bash
# Run tests to see failures
cargo test

# Fix failing tests
# Then commit again
git commit
```

### Hook Not Running

```bash
# Make sure hook is executable
chmod +x .git/hooks/pre-commit
chmod +x .git/hooks/pre-push

# Verify hook exists
ls -la .git/hooks/
```

## 🔧 Makefile Commands Reference

| Command | Description | Equivalent |
|---------|-------------|------------|
| `make help` | Show available commands | - |
| `make format` | Format code | `cargo fmt` |
| `make check` | Check compilation | `cargo check --all-targets --all-features` |
| `make lint` | Run clippy | `cargo clippy --all-targets --all-features -- -D warnings` |
| `make test` | Run tests | `cargo test --all-features` |
| `make build` | Build release | `cargo build --release` |
| `make clean` | Clean artifacts | `cargo clean` |
| `make ci` | Full CI pipeline | format + check + lint + test + build |
| `make dev` | Quick dev cycle | format + check + test |
| `make install` | Install binary | `cargo install --path .` |

## 🎯 Best Practices

### Commit Messages

- Use conventional commit format: `feat:`, `fix:`, `docs:`, `test:`, etc.
- Keep first line under 50 characters
- Provide detailed description in body when needed

### Branch Naming

- Use descriptive branch names: `feat/new-feature`, `fix/bug-description`
- Avoid special characters and spaces
- Use lowercase with hyphens

### Pull Request Process

1. Create feature branch from `main`
2. Make changes with proper tests
3. Ensure all hooks pass locally
4. Push to your fork
5. Create pull request with clear description
6. Address review feedback
7. Wait for CI to pass
8. Merge when approved

### Modular Architecture

When adding new builtin functions:

1. Create implementation in appropriate `src/builtins/` module
2. Implement the `BuiltinFunction` trait
3. Register the function in `src/builtins/mod.rs`
4. Add tests for the new functionality
5. Update documentation if needed

## 🐛 Debugging Tips

### Hook Issues

```bash
# Check hook permissions
ls -la .git/hooks/

# View hook output manually
.git/hooks/pre-commit

# Check Git configuration
git config --list | grep hook
```

### Build Issues

```bash
# Clean and rebuild
make clean
cargo build

# Check specific targets
cargo check --all-targets
cargo test --all-features
```

### Test Failures

```bash
# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in specific module
cargo test integration_tests
```

## 📞 Getting Help

- Check the main [README.md](README.md) for project overview
- Read [HOW_IT_WORKS.md](HOW_IT_WORKS.md) for architecture details
- See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines
- Review existing tests for implementation examples
- Ask questions in GitHub issues or discussions

---

This development guide ensures consistent code quality and smooth collaboration. The Git hooks automate quality checks, preventing broken code from entering the repository and maintaining professional development standards.
