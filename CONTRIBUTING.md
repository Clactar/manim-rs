# Contributing to manim-rs

Thank you for your interest in contributing to manim-rs! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/manim-rs.git
   cd manim-rs
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Check code quality**
   ```bash
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

## Development Workflow

### 1. Create a Branch

Use descriptive branch names:
- `feature/add-circle-animation`
- `fix/vector-normalization-bug`
- `docs/improve-api-examples`
- `perf/optimize-rendering`

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

Follow the coding standards in `.cursor/rules/`:
- Write comprehensive documentation for all public APIs
- Add tests for new functionality
- Include benchmarks for performance-related changes
- Keep commits focused and atomic

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench

# Check documentation builds
cargo doc --no-deps --open
```

### 4. Format and Lint

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

### 5. Commit Your Changes

Follow conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `perf`: Performance improvement
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Build/tooling changes

**Example:**
```
feat(animation): add custom easing function support

Implement support for user-defined easing functions through
the Ease trait. This allows arbitrary animation curves.

Closes #42
```

### 6. Push and Create a Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear description of changes
- Reference to related issues
- Screenshots/videos for visual changes
- Benchmark results for performance improvements

## Pull Request Guidelines

### Required Checks
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation builds (`cargo doc --no-deps`)
- [ ] New features have tests
- [ ] Public APIs have documentation with examples
- [ ] CHANGELOG.md is updated (for significant changes)

### Review Process
1. Maintainers will review your PR
2. Address feedback and update your branch
3. Once approved, your PR will be merged

## Documentation Standards

Every public item must have documentation:

```rust
/// A 2D vector in Euclidean space.
///
/// This type is optimized for performance with inline operations.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
///
/// let v = Vector2D::new(3.0, 4.0);
/// assert_eq!(v.magnitude(), 5.0);
/// ```
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}
```

## Testing Guidelines

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let result = v1 + v2;
        assert_eq!(result, Vector2D::new(4.0, 6.0));
    }
}
```

### Integration Tests
Place in `tests/` directory for end-to-end scenarios.

### Benchmarks
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("operation_name", |b| {
        b.iter(|| {
            // Code to benchmark
            black_box(expensive_operation());
        });
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

## Performance Guidelines

- Profile before optimizing
- Use `cargo bench` to measure improvements
- Document performance characteristics in PRs
- Consider SIMD for numeric operations
- Use `rayon` for parallelizable workloads

## Adding Examples

Examples should be:
1. Self-contained and runnable
2. Well-commented
3. Demonstrate a specific feature
4. Produce visual output when possible

```rust
//! Basic circle animation example
//!
//! Demonstrates creating and animating a simple shape.

use manim_rs::prelude::*;

fn main() -> Result<()> {
    // Your example code here
    Ok(())
}
```

## Need Help?

- Open an issue for questions
- Check existing issues and discussions
- Join our community discussions

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.

