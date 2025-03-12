# LeetCode Rust Solutions Guide

## Build and Test Commands

```bash
# Run a specific test file
cargo test --test <problem_number>

# Run a specific test
cargo test --test <problem_number> -- <test_name>

# Run tests with output
cargo test -- --nocapture
```

## Code Style Guidelines

### Formatting and Structure

- Use Rust's standard formatting with `rustfmt`
- Include detailed documentation comments with `///`
- Group implementation logic within `impl Solution`
- Place tests in a `#[cfg(test)]` module

### Naming Conventions

- Use snake_case for variables and functions
- Descriptive variable naming (e.g., `num_to_index` not `map`)

### Error Handling

- Use pattern matching and `Option`/`Result` types
- Prefer `if let` and `match` over `.unwrap()`
- Return empty collections when no solution exists

### Performance

- Initialize collections with capacity when size is known
- Use references (`&`) to avoid unnecessary cloning
- Leverage Rust's standard library (HashMap, BTreeMap, etc.)
