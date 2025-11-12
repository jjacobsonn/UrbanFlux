# Contributing to UrbanFlux

Thank you for your interest in contributing to UrbanFlux! This document provides guidelines and instructions for contributing.

---

##  Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

---

##  Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards

**Positive behaviors include:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community

**Unacceptable behaviors include:**
- Harassment, trolling, or discriminatory comments
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

---

##  Getting Started

### Prerequisites

1. **Rust 1.80+**: Install via [rustup](https://rustup.rs/)
2. **Docker & Docker Compose**: For running PostgreSQL
3. **Git**: For version control
4. **Make**: For build automation (optional but recommended)

### Initial Setup

```bash
# Fork the repository on GitHub

# Clone your fork
git clone https://github.com/YOUR_USERNAME/UrbanFlux.git
cd UrbanFlux

# Add upstream remote
git remote add upstream https://github.com/jjacobsonn/UrbanFlux.git

# Copy environment file
cp .env.example .env

# Install dependencies and build
cargo build

# Run tests
cargo test

# Start infrastructure
make up
```

---

##  Development Process

### 1. Find or Create an Issue

- Check existing [issues](https://github.com/jjacobsonn/UrbanFlux/issues)
- Comment on an issue to claim it
- For new features, create an issue first to discuss

### 2. Create a Branch

```bash
# Update your fork
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feat/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

### 3. Make Your Changes

- Write code following our [Style Guide](STYLEGUIDE.md)
- Add tests for new functionality
- Update documentation as needed
- Commit frequently with clear messages

### 4. Test Your Changes

```bash
# Format code
make fmt

# Run linter
make lint

# Run tests
make test

# Test Docker build
docker build -t urbanflux:test .
```

### 5. Keep Your Branch Updated

```bash
# Fetch upstream changes
git fetch upstream

# Rebase on main
git rebase upstream/main

# Resolve conflicts if any
# Force push to your fork (if rebased)
git push origin feat/your-feature-name --force-with-lease
```

---

##  Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] Branch is up to date with main

### Submitting a PR

1. **Push to Your Fork**
   ```bash
   git push origin feat/your-feature-name
   ```

2. **Create Pull Request**
   - Go to the original repository on GitHub
   - Click "New Pull Request"
   - Select your fork and branch
   - Fill out the PR template completely

3. **PR Title Format**
   ```
   feat(scope): brief description
   fix(scope): brief description
   docs(scope): brief description
   ```

4. **PR Description Should Include**
   - What changed and why
   - Link to related issues
   - Screenshots (if UI changes)
   - Testing performed
   - Any breaking changes

### Review Process

1. **Automated Checks**: CI must pass
2. **Code Review**: At least one approval required
3. **Maintainer Review**: Final approval from maintainers
4. **Merge**: Squash and merge to main

### After Merge

```bash
# Update your local main
git checkout main
git pull upstream main

# Delete feature branch
git branch -d feat/your-feature-name
git push origin --delete feat/your-feature-name
```

---

##  Coding Standards

### Code Style

Follow the [STYLEGUIDE.md](STYLEGUIDE.md) for:
- Naming conventions
- Code organization
- Error handling
- Documentation

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `perf`: Performance
- `test`: Tests
- `chore`: Maintenance

**Examples:**
```bash
feat(etl): add streaming CSV reader

Implements async CSV reader with configurable chunk size
using csv-async crate for high-throughput processing.

Closes #42

---

fix(db): correct connection pool timeout

Increased timeout from 5s to 30s to handle large bulk inserts.
Added retry logic with exponential backoff.

Fixes #67
```

---

##  Testing Guidelines

### Test Coverage

- **Unit Tests**: For individual functions and modules
- **Integration Tests**: For component interactions
- **Property Tests**: For invariants (using proptest)

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_csv_reader_parses_valid_file() {
        // Arrange
        let test_file = create_test_csv();
        
        // Act
        let result = parse_csv(&test_file).await;
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 100);
    }

    #[test]
    fn test_validator_rejects_invalid_borough() {
        let validator = Validator::new();
        assert!(!validator.validate_borough("INVALID").unwrap());
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

---

##  Documentation

### Code Documentation

```rust
/// Brief one-line description
///
/// More detailed explanation of what this does,
/// including algorithm details if relevant.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Describe when and why this fails
///
/// # Examples
///
/// ```
/// let result = my_function("test", 42)?;
/// assert_eq!(result, expected);
/// ```
pub fn my_function(arg1: &str, arg2: usize) -> Result<String> {
    // Implementation
}
```

### Documentation Updates

When adding features or changing behavior:
- Update relevant `.md` files
- Update code comments
- Update CLI help text
- Add examples where helpful

---

##  Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and general discussion
- **Pull Requests**: Code review and collaboration

### Getting Help

- Check existing [issues](https://github.com/jjacobsonn/UrbanFlux/issues)
- Search [discussions](https://github.com/jjacobsonn/UrbanFlux/discussions)
- Read the [documentation](docs/)
- Ask in a new discussion

### Reporting Bugs

Use the bug report template and include:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Relevant logs

### Suggesting Features

Use the feature request template and include:
- Problem statement
- Proposed solution
- Alternatives considered
- Use cases

---

##  Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Project README

---

##  License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

##  Thank You

Every contribution matters! Whether it's:
- Reporting a bug
- Suggesting a feature
- Improving documentation
- Submitting code
- Reviewing PRs

Your efforts help make UrbanFlux better for everyone.

---

**Questions?** Open an issue or discussion — we're here to help!
