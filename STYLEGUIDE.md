# UrbanFlux Style Guide

> **Purpose**: Maintain consistent, readable, and maintainable code across the UrbanFlux project.

---

## 🎯 Core Principles

1. **Clarity over Cleverness**: Write code that's easy to understand
2. **Safety First**: Leverage Rust's type system and ownership model
3. **Performance-Aware**: Write efficient code, but profile before optimizing
4. **Test-Driven**: Write tests alongside your code
5. **Document Intent**: Explain why, not what

---

## 📝 Naming Conventions

### Files and Directories
- **snake_case** for all file and directory names
- Example: `service_request.rs`, `etl_pipeline.rs`

### Rust Code
- **PascalCase** for types (structs, enums, traits)
  ```rust
  struct ServiceRequest { }
  enum EtlMode { }
  trait DataLoader { }
  ```

- **snake_case** for functions, variables, and modules
  ```rust
  fn process_csv_chunk() { }
  let row_count = 0;
  mod etl_runner;
  ```

- **SCREAMING_SNAKE_CASE** for constants
  ```rust
  const MAX_CHUNK_SIZE: usize = 100_000;
  const DEFAULT_TIMEOUT_SECS: u64 = 30;
  ```

### Database
- **snake_case** for tables, columns, and indexes
  ```sql
  CREATE TABLE service_requests (
    unique_key BIGINT,
    created_at TIMESTAMPTZ
  );
  ```

---

## 🏗️ Code Organization

### Module Structure
```rust
// Public exports first
pub use self::extract::*;
pub use self::transform::*;

// Module declarations
pub mod extract;
pub mod transform;
pub mod load;

// Imports grouped and sorted
use anyhow::Result;
use std::path::Path;
use tokio::fs::File;
```

### Import Order
1. Standard library (`std`, `core`)
2. External crates (alphabetical)
3. Internal modules (alphabetical)
4. Blank line between groups

```rust
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::fs::File;
use tracing::{info, warn};

use crate::config::Config;
use crate::db::schema::ServiceRequest;
```

---

## ✍️ Code Style

### Line Length
- Maximum **100 characters** per line
- Use `rustfmt` to enforce automatically

### Function Length
- Keep functions **under 50 lines** when possible
- Extract complex logic into helper functions
- If a function is complex, break it down

### Error Handling
```rust
// ✅ Good - Use ? operator
pub async fn load_config() -> Result<Config> {
    let content = tokio::fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

// ✅ Good - Add context
pub async fn load_config(path: &Path) -> Result<Config> {
    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}

// ❌ Bad - Unwrap without context
pub async fn load_config() -> Config {
    let content = tokio::fs::read_to_string("config.toml").await.unwrap();
    toml::from_str(&content).unwrap()
}
```

### Comments
```rust
// Good: Explain why, not what
// We use a HashSet here to ensure O(1) deduplication
// instead of O(N²) with vector iteration
let mut seen_keys = HashSet::new();

// Bad: Stating the obvious
// Create a new HashSet
let mut seen_keys = HashSet::new();
```

### Documentation Comments
```rust
/// Loads and validates NYC 311 service request records from a CSV file.
///
/// # Arguments
///
/// * `path` - Path to the CSV file
/// * `chunk_size` - Number of rows to process per chunk
///
/// # Errors
///
/// Returns an error if:
/// - File cannot be read
/// - CSV parsing fails
/// - Data validation fails
///
/// # Examples
///
/// ```
/// let records = load_csv("data.csv", 10_000).await?;
/// ```
pub async fn load_csv(path: &str, chunk_size: usize) -> Result<Vec<ServiceRequest>> {
    // Implementation
}
```

---

## 🧪 Testing Conventions

### Test Function Names
- Use descriptive names with `test_` prefix
- Follow pattern: `test_<functionality>_<condition>_<expected_result>`

```rust
#[tokio::test]
async fn test_csv_parser_valid_input_returns_records() {
    // Test implementation
}

#[tokio::test]
async fn test_csv_parser_malformed_row_returns_error() {
    // Test implementation
}

#[test]
fn test_validator_invalid_borough_returns_false() {
    // Test implementation
}
```

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_functionality() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = process(input).await.unwrap();
        
        // Assert
        assert_eq!(result.len(), 100);
    }
}
```

---

## 🔄 Git Commit Conventions

### Commit Message Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Build process, dependencies, tooling
- `ci`: CI/CD changes

### Examples
```bash
feat(etl): add CSV streaming reader

Implement async CSV reader with configurable chunk size.
Uses csv-async crate with tokio runtime for high throughput.

Closes #42

---

fix(db): correct PostgreSQL connection pool timeout

Previous timeout of 5s was too short for large bulk inserts.
Increased to 30s and added retry logic.

Fixes #67

---

chore(deps): upgrade tokio to 1.42

---

docs(readme): add installation instructions
```

### Branch Naming
- `feat/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `refactor/component-name` - Code refactoring
- `docs/topic` - Documentation updates
- `test/component-name` - Test additions/updates

Examples:
```bash
feat/csv-streaming-reader
fix/postgres-connection-leak
refactor/error-handling
docs/api-documentation
test/integration-tests
```

---

## 🚫 Anti-Patterns to Avoid

### ❌ Overuse of `unwrap()`
```rust
// Bad
let value = some_option.unwrap();

// Good
let value = some_option.context("Expected value to be present")?;
```

### ❌ Silent Error Handling
```rust
// Bad
if let Err(_) = risky_operation() {
    return;
}

// Good
if let Err(e) = risky_operation() {
    warn!("Operation failed: {}", e);
    return Err(e);
}
```

### ❌ Magic Numbers
```rust
// Bad
if count > 100000 {
    flush_buffer();
}

// Good
const FLUSH_THRESHOLD: usize = 100_000;
if count > FLUSH_THRESHOLD {
    flush_buffer();
}
```

### ❌ Deeply Nested Code
```rust
// Bad
if condition1 {
    if condition2 {
        if condition3 {
            // Deep nesting
        }
    }
}

// Good - Early returns
if !condition1 {
    return Err(Error::Condition1Failed);
}
if !condition2 {
    return Err(Error::Condition2Failed);
}
if !condition3 {
    return Err(Error::Condition3Failed);
}
// Continue with main logic
```

---

## 🔍 Code Review Checklist

- [ ] Code follows naming conventions
- [ ] Functions are reasonably sized (<50 lines preferred)
- [ ] Errors are handled with context
- [ ] No `unwrap()` or `expect()` in production code
- [ ] Tests are included for new functionality
- [ ] Documentation is updated
- [ ] `cargo fmt` has been run
- [ ] `cargo clippy` passes with no warnings
- [ ] Commit messages follow conventions
- [ ] No secrets or sensitive data committed

---

## 📊 Performance Guidelines

1. **Profile Before Optimizing**: Use `cargo flamegraph` or similar tools
2. **Avoid Premature Optimization**: Correctness first, then optimize
3. **Use Appropriate Data Structures**:
   - `Vec` for ordered, indexed access
   - `HashMap` for key-value lookups
   - `HashSet` for uniqueness checks
4. **Async for I/O-Bound**: CPU-bound work may not benefit from async
5. **Batch Operations**: Group database operations when possible

---

## 🔒 Security Guidelines

1. **No Secrets in Code**: Use environment variables
2. **Validate All Input**: Especially from external sources (CSV, APIs)
3. **Use Prepared Statements**: Prevent SQL injection (SQLx handles this)
4. **Sanitize Logs**: Don't log passwords or sensitive data
5. **Keep Dependencies Updated**: Run `cargo audit` regularly

---

## 📚 Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Tokio Best Practices](https://tokio.rs/tokio/topics/best-practices)

---

**Remember**: These are guidelines, not rigid rules. Use good judgment and prioritize team collaboration.
