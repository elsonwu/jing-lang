# Development Guidelines for Jing Language

This document establishes the core development practices that must be followed for all contributions to the Jing language project.

## 🎯 Core Principles

### 1. Documentation-First Development
- **Update documentation BEFORE or WITH every feature/change**
- All new features must include documentation updates
- All API changes must update relevant reference documentation
- Examples must be provided for new capabilities

### 2. Test-Driven Quality
- Write tests for all new features and bug fixes
- Maintain comprehensive test coverage
- All tests must pass before commits
- Add both unit tests and integration tests where applicable

### 3. Quality Gates
Every commit must pass all quality gates:
- ✅ Code formatting (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Compilation (`cargo build`)
- ✅ All tests (`cargo test`)

### 4. Conventional Commits
All commits must follow [Conventional Commits](https://conventionalcommits.org/) specification for automatic changelog generation and semantic versioning.

## 📋 Pre-Commit Checklist

Before every commit, ensure you have completed ALL of the following:

### ✅ Documentation Updates
- [ ] Updated README.md if adding new features
- [ ] Updated LANGUAGE_REFERENCE.md for language changes
- [ ] Updated HOW_IT_WORKS.md for implementation changes
- [ ] Added/updated examples in `examples/` directory
- [ ] Updated API documentation in source code
- [ ] Updated CHANGELOG.md with user-facing changes

### ✅ Testing Requirements  
- [ ] Added unit tests for new functions/modules
- [ ] Added integration tests for new language features
- [ ] All existing tests still pass
- [ ] New tests cover edge cases and error conditions
- [ ] Test names are descriptive and follow naming conventions

### ✅ Code Quality
- [ ] Code is formatted: `cargo fmt`
- [ ] No linting warnings: `cargo clippy`
- [ ] Code compiles without warnings: `cargo build`
- [ ] All tests pass: `cargo test`

### ✅ Commit Message
- [ ] Follows conventional commit format
- [ ] Type is appropriate (feat, fix, docs, test, etc.)
- [ ] Description is clear and concise
- [ ] Breaking changes are documented in footer

## 🔄 Development Workflow

### Feature Development Process

1. **Create Feature Branch**
   ```bash
   git checkout -b feat/feature-name
   ```

2. **Implement Feature**
   - Write failing tests first (TDD approach)
   - Implement feature to make tests pass
   - Update documentation as you go

3. **Quality Check**
   ```bash
   # Format code
   cargo fmt
   
   # Check linting
   cargo clippy -- -D warnings
   
   # Verify compilation
   cargo build
   
   # Run all tests
   cargo test
   ```

4. **Documentation Update**
   - Update all relevant documentation files
   - Add examples demonstrating new features
   - Update help text and API documentation

5. **Commit with Conventional Format**
   ```bash
   git add .
   git commit -m "feat(component): add new feature description

   More detailed explanation of the feature and its impact.
   
   BREAKING CHANGE: description if applicable"
   ```

6. **Create Pull Request**
   - Include comprehensive description
   - Reference any related issues
   - Ensure CI passes before requesting review

## 📚 Documentation Standards

### Required Documentation Updates

When making changes, you MUST update the corresponding documentation:

| Change Type | Required Documentation Updates |
|-------------|-------------------------------|
| New Language Feature | README.md, LANGUAGE_REFERENCE.md, examples/ |
| New Builtin Function | LANGUAGE_REFERENCE.md, function help text |
| Architecture Change | HOW_IT_WORKS.md, VISUAL_GUIDE.md |
| New Module/Component | Module-level documentation, API docs |
| Bug Fix | CHANGELOG.md (if user-facing) |
| Performance Improvement | CHANGELOG.md, relevant technical docs |

### Documentation Style Guide

1. **Use Clear Headers**: Organize content with descriptive headers
2. **Provide Examples**: Every feature should have working code examples
3. **Be Concise**: Write clear, concise explanations
4. **Update Existing**: Don't just add new docs, update existing ones
5. **Cross-Reference**: Link related concepts and documentation

## 🧪 Testing Standards

### Test Categories

1. **Unit Tests** (in `src/` modules)
   - Test individual functions and components
   - Focus on edge cases and error conditions
   - Quick to run and isolated

2. **Integration Tests** (in `tests/` directory)
   - Test full compilation pipeline
   - Test language features end-to-end
   - Verify behavior of complete programs

3. **Example Tests**
   - All examples in `examples/` should run successfully
   - Examples serve as both documentation and tests

### Test Naming Conventions

```rust
#[test]
fn test_feature_happy_path() { }        // Normal usage

#[test]
fn test_feature_edge_case_empty() { }   // Edge cases

#[test]
fn test_feature_error_invalid_input() { } // Error conditions
```

### Test Requirements

- **Every public function** must have at least one test
- **Every error condition** must have a test verifying the error
- **Integration tests** for all language features
- **Performance tests** for critical path operations (optional but recommended)

## 🔧 Implementation Guidelines

### Code Organization

1. **Separation of Concerns**: Each module has a single responsibility
2. **Modular Design**: Use traits and interfaces for extensibility  
3. **Error Handling**: Use `Result<T, JingError>` consistently
4. **Documentation**: Every public function/struct needs doc comments

### Architecture Patterns

1. **Pipeline Pattern**: Lexer → Parser → Compiler → VM
2. **Trait-Based Extensions**: Use BuiltinFunction trait for new functions
3. **Registry Pattern**: Central registration for extensible features
4. **Error Propagation**: Use `?` operator and proper error types

## 📈 Continuous Improvement

### Regular Maintenance Tasks

- Update dependencies monthly
- Review and update documentation quarterly
- Conduct architecture reviews for major changes
- Maintain benchmark suite for performance tracking

### Quality Metrics

Monitor and maintain:
- Test coverage > 80%
- Documentation coverage for all public APIs
- Zero warnings in CI builds
- Fast test suite execution (< 30 seconds)

## 🚫 Violations and Enforcement

**Commits that violate these guidelines will be rejected.**

Common violations:
- Adding features without updating documentation
- Committing code that doesn't pass all quality gates
- Using non-conventional commit messages
- Missing or inadequate test coverage
- Breaking existing functionality without proper deprecation

## 🆘 Getting Help

If you need help following these guidelines:
1. Check the existing codebase for examples
2. Ask questions in issues or discussions
3. Review the CONTRIBUTING.md for additional context
4. Look at recent commits for conventional commit examples

Remember: **These guidelines exist to maintain code quality and make the project maintainable for everyone.** Following them helps create a professional, reliable codebase that serves as a great educational resource.
