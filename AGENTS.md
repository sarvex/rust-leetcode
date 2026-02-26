# Rust LeetCode Solutions - Agent Guide

A comprehensive collection of 1,100+ LeetCode problem solutions implemented in **Rust 2024 Edition**. This is a flat-file repository where each solution is a standalone Rust file with no Cargo project structure.

## Project Overview

| Attribute | Value |
|-----------|-------|
| **Language** | Rust (Edition 2024) |
| **Total Solutions** | 1,121 files |
| **Total Lines** | ~100,000 lines |
| **License** | MIT |
| **Repository Type** | Flat file (no Cargo.toml) |
| **File Pattern** | `XXXX.Problem Name.rs` |

This repository contains production-ready solutions with comprehensive documentation, unit tests, and optimal algorithmic complexity. Each solution is designed to be self-contained and directly runnable in LeetCode's Rust environment.

## Repository Structure

### File Organization

```
rust-leetcode/
├── XXXX.Problem Title.rs     # Individual solution files
├── .rules                     # Comprehensive Rust coding standards
├── .trunk/trunk.yaml          # Trunk linter/formatter configuration
├── .vscode/settings.json      # VS Code workspace settings
├── .github/                   # GitHub automation
│   ├── renovate.json         # Renovate bot configuration
│   └── dependabot.yml        # Dependabot configuration
├── setup.sh                   # Unix setup script
├── setup.ps1                  # PowerShell setup script
├── AGENTS.md                  # This file
├── README.md                  # Human-readable documentation
└── LICENSE                    # MIT License
```

### File Naming Convention

All solution files follow the strict pattern:
```
[4-digit zero-padded number].[Problem Title].rs
```

Examples:
- `0001.Two Sum.rs`
- `0146.LRU Cache.rs`
- `3292.Minimum Number of Valid Strings to Form Target II.rs`

## Code Structure

### Standard Solution Template

Every solution file follows this consistent structure:

```rust
use std::collections::HashMap;

// Uncomment when needed for local testing:
// #[derive(Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

impl Solution {
    /// Brief algorithm description.
    ///
    /// # Intuition
    /// Key insight that unlocks the solution.
    ///
    /// # Approach
    /// Step-by-step algorithm explanation.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    ///
    /// # Panics (if applicable)
    /// Conditions under which the function panics.
    pub fn method_name(params: Type) -> ReturnType {
        // Implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        assert_eq!(Solution::method_name(input), expected);
    }

    #[test]
    fn test_edge_cases() {
        // Boundary conditions
    }
}
```

### Documentation Requirements

All public methods must include doc comments with these sections:

1. **Tagline**: Brief one-line description of the approach
2. **# Intuition**: High-level insight and reasoning
3. **# Approach**: Algorithm steps and data structures used
4. **# Complexity**: Time and space complexity analysis
5. **# Panics** (optional): Panic conditions if any

## Common Structs (Commented in Files)

Since this is a flat-file repository without Cargo.toml, common LeetCode structs are included as **commented code** at the top of relevant files. These should be **uncommented only for local testing**:

### ListNode (Linked List Problems)
```rust
// #[derive(Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { val, next: None }
//     }
// }
```

### TreeNode (Binary Tree Problems)
```rust
// #[derive(Debug)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
```

> **Note**: On LeetCode, these structs are pre-defined by the platform. Keep them commented in files to avoid conflicts when copying solutions directly.

## Common Imports and Patterns

### Most Frequently Used Imports

```rust
// Data structures
use std::collections::HashMap;      // 67 occurrences
use std::collections::VecDeque;     // 38 occurrences - BFS, queues
use std::collections::HashSet;      // 31 occurrences
use std::collections::BinaryHeap;   // 20 occurrences - priority queues
use std::collections::BTreeMap;     // 5 occurrences - ordered map
use std::collections::BTreeSet;     // 3 occurrences - ordered set

// Smart pointers (for tree/graph problems)
use std::rc::Rc;                    // 69 occurrences
use std::cell::RefCell;             // 69 occurrences

// Comparison utilities
use std::cmp::Reverse;              // 12 occurrences - min-heap pattern
use std::cmp::Ordering;             // 8 occurrences
```

### Common Algorithm Patterns

#### Two Pointers
```rust
let (mut left, mut right) = (0, nums.len() - 1);
while left < right {
    match nums[left] + nums[right] {
        sum if sum == target => return vec![left, right],
        sum if sum < target => left += 1,
        _ => right -= 1,
    }
}
```

#### Sliding Window
```rust
let (mut start, mut end, mut best) = (0, 0, 0);
while end < nums.len() {
    // expand window
    while window_invalid() { start += 1; }
    best = best.max(end - start + 1);
    end += 1;
}
```

#### Hash Map for O(1) Lookup
```rust
let mut num_to_index = HashMap::with_capacity(nums.len());
for (i, num) in nums.iter().enumerate() {
    if let Some(&j) = num_to_index.get(&(target - num)) {
        return vec![j as i32, i as i32];
    }
    num_to_index.insert(num, i);
}
```

#### BFS with VecDeque
```rust
let mut queue = VecDeque::from([root]);
while let Some(node) = queue.pop_front() {
    // process node
    queue.push_back(left_child);
    queue.push_back(right_child);
}
```

#### Binary Heap (Min/Max Heap)
```rust
// Min-heap pattern
let mut heap = BinaryHeap::new();
heap.push(Reverse((cost, node)));
while let Some(Reverse((cost, node))) = heap.pop() {
    // process
}
```

#### Greedy for Lexicographically Smallest
When constructing a lexicographically smallest array with constraints:
```rust
// Process from largest to smallest, decide each element's properties
// Result will naturally be in lexicographic order when collected appropriately
for val in (1..=n).rev() {
    if can_make_negative(val, remaining_target) {
        negatives.push(-val);  // Larger negatives first = lexicographically smaller
        target += val;
    } else {
        positives.push(val);
        target -= val;
    }
}
// Combine: negatives (descending) + reversed(positives) (ascending)
```

## Choosing the Right Pattern

| Problem Characteristic | Recommended Approach |
|------------------------|---------------------|
| Sorted array, find pair | Two pointers (O(n)) |
| Subarray/sliding condition | Sliding window |
| Find duplicates/frequency | HashMap |
| Tree traversal, level by level | BFS with VecDeque |
| Tree traversal, path finding | DFS recursive |
| Need min/max repeatedly | BinaryHeap |
| Range queries, updates | Segment tree or BIT |
| Overlapping subproblems | Dynamic Programming |

## Code Quality Standards

### Rust 2024 Edition Compliance

- **No reference patterns in closures** when default binding mode is `ref`:
  ```rust
  // WRONG in Rust 2024
  .filter(|(i, &c)| ...)
  
  // CORRECT
  .filter(|(i, c)| *c == target)
  ```

### Formatting and Linting

The project uses **Trunk** for unified linting and formatting:

```yaml
# .trunk/trunk.yaml
lint:
  enabled:
    - rustfmt@1.65.0      # Code formatting
    - checkov@3.2.396     # Security scanning
    - markdownlint@0.44.0 # Markdown linting
    - prettier@3.5.3      # General formatting
    - shellcheck@0.10.0   # Shell script linting
    - shfmt@3.6.0         # Shell formatting
    - trufflehog@3.88.20  # Secret scanning
```

### Required Commands

```bash
# Format all code
cargo fmt

# Check with clippy (strict mode)
cargo clippy -- -D warnings

# Run tests (when in a Cargo project)
cargo test
```

## Testing Strategy

Every solution includes comprehensive unit tests covering:

1. **Example cases**: From problem statement
2. **Edge cases**: Empty input, single element, minimum/maximum values
3. **Boundary conditions**: i32::MIN, i32::MAX, large n
4. **Negative cases**: Invalid inputs where applicable

### Test Module Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions for test data construction
    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> { ... }
    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> { ... }

    #[test]
    fn test_example_from_problem() { ... }

    #[test]
    fn test_edge_empty_input() { ... }

    #[test]
    fn test_edge_single_element() { ... }

    #[test]
    fn test_boundary_max_values() { ... }
}
```

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Variables, functions | `snake_case` | `num_to_index`, `find_max` |
| Types, traits, enums | `PascalCase` | `TreeNode`, `ListNode` |
| Constants | `SCREAMING_SNAKE_CASE` | `MOD`, `MAX_SIZE` |
| Solution struct | `Solution` | `impl Solution` |
| Helper structs | Descriptive | `LRUCache`, `Node` |

### Variable Naming Guidelines

- Use descriptive names: `char_count` over `map`
- Avoid single-letter names except for:
  - Loop indices: `i`, `j`, `k`
  - Coordinates: `x`, `y`, `z`
  - Well-known math: `n`, `m` for dimensions
- Prefer `left`/`right` over `l`/`r` for pointers
- Use `prev`/`curr`/`next` for iteration state

## Working with This Repository

### Running Individual Solutions

Since there's no Cargo.toml, to run a solution locally:

1. Create a temporary Cargo project:
   ```bash
   cargo new temp_solution && cd temp_solution
   ```

2. Copy solution content to `src/main.rs`

3. Uncomment the required structs (ListNode/TreeNode) at the top of the file

4. Run with:
   ```bash
   cargo run
   # or
   cargo test
   ```

### Adding New Solutions

1. Create file with correct naming: `XXXX.Problem Title.rs`
2. Follow the standard template structure
3. Include commented ListNode/TreeNode if problem requires them
4. Include all required documentation sections
5. Add comprehensive unit tests
6. Run formatting: `cargo fmt`
7. Ensure no clippy warnings

## Iterators vs Loops

**Always prefer functional/iterator style over raw for loops.** Iterators are more expressive, composable, and often optimize better.

```rust
// Preferred: functional style
let sum: i32 = nums.iter().filter(|x| **x > 0).sum();

let max_even = nums.iter().filter(|x| *x % 2 == 0).max();

let squared: Vec<_> = nums.iter().map(|x| x * x).collect();

// Early exit with find/take_while/any/all
let first_negative = nums.iter().find(|x| **x < 0);
let has_zero = nums.iter().any(|x| *x == 0);

// Accumulation with fold/scan
let running_sum: Vec<_> = nums.iter().scan(0, |acc, x| { *acc += x; Some(*acc) }).collect();
```

**Only use loops when**:
- Mutating external state in complex ways that `fold` cannot express cleanly
- Multiple simultaneous mutable borrows are required
- The algorithm is inherently stateful (e.g., two-pointer with complex conditions)

```rust
// Acceptable: complex two-pointer with early return
let (mut l, mut r) = (0, nums.len() - 1);
while l < r {
    match nums[l] + nums[r].cmp(&target) {
        Ordering::Equal => return Some((l, r)),
        Ordering::Less => l += 1,
        Ordering::Greater => r -= 1,
    }
}
```

## Code Style Examples

### ✅ Good: Iterator chains
```rust
let sum: i32 = nums.iter().filter(|x| **x > 0).sum();
```

### ❌ Bad: Unnecessary loops
```rust
let mut sum = 0;
for i in 0..nums.len() {
    if nums[i] > 0 {
        sum += nums[i];
    }
}
```

### ✅ Good: Entry API
```rust
*count.entry(num).or_insert(0) += 1;
```

### ❌ Bad: Manual check + insert
```rust
if !count.contains_key(&num) {
    count.insert(num, 0);
}
*count.get_mut(&num).unwrap() += 1;
```

### ✅ Good: Pattern matching with match ergonomics
```rust
.filter(|(i, c)| *c == target)
```

### ❌ Bad: Reference patterns (Rust 2024 error)
```rust
.filter(|(i, &c)| ...)  // ERROR in Rust 2024
```

## Complexity Quick Reference

| Pattern | Typical Time | Typical Space |
|---------|-------------|---------------|
| HashMap lookup | O(1) | O(n) |
| Binary search | O(log n) | O(1) |
| Two pointers | O(n) | O(1) |
| Sliding window | O(n) | O(1) or O(k) |
| BFS/DFS | O(V + E) | O(V) |
| Dijkstra | O((V + E) log V) | O(V) |
| Union-Find | O(α(n)) ≈ O(1) | O(n) |

## Performance Targets

For solutions to be accepted on LeetCode:

- **Easy**: < 10ms runtime, < 2MB memory
- **Medium**: < 50ms runtime, < 3MB memory  
- **Hard**: < 100ms runtime, < 5MB memory

If exceeding these, consider:
- Using `with_capacity()` for collections
- Avoiding `clone()` in hot paths
- Using bit manipulation instead of HashSet for small ranges
- In-place algorithms for space optimization

## Common Pitfalls to Avoid

- **Don't use raw for loops** when iterators suffice (see Iterators vs Loops)
- **Don't forget Rust 2024 match ergonomics** - no `&` patterns in closures
- **Don't use `unimplemented!()`** in final solutions - always provide working code
- **Don't omit doc comments** - every public method needs full documentation
- **Don't use `println!()`** - LeetCode solutions shouldn't have I/O
- **Don't leave ListNode/TreeNode uncommented** - keep them commented for LeetCode compatibility

## Mandatory Performance Optimization

**All solutions must be optimized for the given constraints from the start.** Do not provide naive or unoptimized solutions that "work" but will TLE (Time Limit Exceeded) or MLE (Memory Limit Exceeded) on LeetCode.

### Required Optimizations by Constraint

| Constraint | Required Approach |
|------------|-------------------|
| `n <= 10^5` | O(n log n) or better; avoid O(n²) |
| `n <= 10^6` | O(n) or O(n log n); preallocate collections |
| `n <= 10^9` or `queries <= 10^5` | O(log n) per query; use segment trees, BIT, or binary search |
| Range queries, updates | Segment tree, Fenwick tree, or sparse table |
| Path queries on tree | LCA with binary lifting or heavy-light decomposition |
| Pair sum/duplicate finding | HashMap/O(1) lookup, NOT O(n) scan |

### Mandatory Implementation Patterns

1. **HashMap with Combined Keys**: Use `i64` keys instead of tuple keys `(i32, i32)`:
   ```rust
   // GOOD: Single i64 key, no tuple hashing overhead
   let key = ((xor as u32 as i64) << 32) | (parity as u32 as i64);
   map.insert(key, value);
   
   // BAD: Tuple hashing is slower
   map.insert((xor, parity), value);
   ```

2. **Preallocate All Collections**:
   ```rust
   let mut map = HashMap::with_capacity(n);
   let mut vec = Vec::with_capacity(n);
   ```

3. **Use Bit Operations for Parity**:
   ```rust
   // GOOD: Single cycle
   if num & 1 == 0 { /* even */ }
   
   // BAD: Modulo is slower
   if num % 2 == 0 { /* even */ }
   ```

4. **Avoid `clone()` in Hot Paths**: Use references or indices instead

5. **Use `entry()` API for HashMap Updates**:
   ```rust
   *map.entry(key).or_insert(0) += 1;  // Single lookup
   ```

6. **Avoid Unnecessary Sorts**: When the problem asks for lexicographically smallest/largest output or sorted result, construct the answer in sorted order directly instead of building unsorted and sorting at the end.
   ```rust
   // GOOD: O(n) - construct in sorted order directly
   for i in (1..=n).rev() {  // descending
       if condition(i) { result.push(i); }
   }
   
   // BAD: O(n log n) - sorting at the end
   for i in 1..=n {
       if condition(i) { result.push(i); }
   }
   result.sort();  // Unnecessary if we can construct in order
   ```

### Verification Checklist

Before submitting any solution, verify:
- [ ] Solution handles `n = 10^5` within 100ms
- [ ] Solution handles `n = 10^6` within 500ms  
- [ ] No O(n²) loops for `n >= 10^4`
- [ ] HashMap keys use primitive types only (no tuples for hot paths)
- [ ] All collections use `with_capacity()` when size is known

## Task Guidelines

### Adding a New Solution
1. Check if problem number already exists (avoid duplicates)
2. Create file with pattern: `XXXX.Problem Name.rs`
3. Include commented ListNode/TreeNode structs if needed
4. Use `cargo new temp_test` to test compilation locally
5. Include at least 3 test cases: example, edge case, stress test

### Refactoring Existing Code
1. Preserve all existing test cases - they define correctness
2. Run benchmarks if optimizing for performance
3. Update complexity comments if algorithm changes

### Fixing Compilation Errors
1. Check Rust 2024 edition compatibility first
2. Verify match ergonomics in closures
3. Ensure all imports are used (clippy `-- -D warnings`)

### Handling Missing Files
When a user references a file (e.g., `@3755.Find Maximum Balanced XOR Subarray Length.rs`) that does not exist:
1. **Always create the missing file** with the solution - do not ask the user for confirmation
2. Use the standard file naming pattern: `XXXX.Problem Name.rs`
3. Follow the standard solution template structure
4. Include the complete solution with proper documentation and unit tests
5. Ensure the file compiles and follows all coding standards

## Pre-Submission Checklist

Before marking any task complete, verify:

- [ ] `cargo fmt` produces no changes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] All unit tests pass (`cargo test`)
- [ ] Doc comments include Intuition, Approach, Complexity
- [ ] No `TODO`, `FIXME`, or `unimplemented!()` markers remain
- [ ] File follows naming: `XXXX.Problem Name.rs`
- [ ] ListNode/TreeNode structs are commented out (not uncommented)
- [ ] Edge cases covered: empty, single element, maximum constraints

## Tooling Integration

### VS Code Settings

The `.vscode/settings.json` includes:
- Peacock theme color (#dd0531 - red)
- SonarLint connected mode configuration
- Custom dictionary for LeetCode terms

### GitHub Automation

- **Renovate**: Automated dependency updates (configured via `renovate.json`)
- **Dependabot**: GitHub Actions updates

### SonarLint

Connected to project `sarvex_rust-leetcode` for continuous code quality analysis.

## Security Considerations

- **No unsafe code**: All solutions use safe Rust only
- **Input validation**: Solutions handle edge cases gracefully
- **No secrets**: Repository contains no API keys or credentials
- **MIT Licensed**: Open source with permissive licensing

## Performance Guidelines

### Preferred Patterns

1. **Preallocate collections** when size is known:
   ```rust
   let mut result = Vec::with_capacity(n);
   ```

2. **Use iterator chains** over manual loops

3. **Leverage entry API** for HashMap updates:
   ```rust
   *freq.entry(key).or_insert(0) += 1;
   ```

4. **Use binary search** from standard library:
   ```rust
   nums.binary_search(&target)
   nums.partition_point(|&x| x < target)
   ```

### Memory Efficiency

- Use references (`&`) to avoid cloning
- Leverage `Option` and `Result` for null-safety
- Prefer stack allocation for small, fixed-size data
- Use `Box<T>` for recursive types (trees, linked lists)

## Troubleshooting

### Common Issues

1. **Clippy warnings**: Run `cargo clippy -- -D warnings` and fix all issues
2. **Formatting**: Run `cargo fmt` before committing
3. **Rust 2024 errors**: Check match ergonomics in closures
4. **Test failures**: Ensure `#[cfg(test)]` module is properly defined
5. **Struct conflicts**: Make sure ListNode/TreeNode are commented when submitting to LeetCode

### Getting Help

- Check `.rules` file for detailed coding standards
- Refer to existing solutions for patterns
- Follow Rust API guidelines: https://rust-lang.github.io/api-guidelines/

---

## Cursor Cloud specific instructions

### Environment

- **Rust**: `rustup update stable` has been run; `rustc` 1.93.1 (stable) supports Rust 2024 Edition.
- **Trunk CLI**: Installed at `/usr/local/bin/trunk` (version 1.22.12). First invocation in a session downloads the versioned backend binary; expect ~3 s latency on the first `trunk check`.

### Testing individual solution files

There is no `Cargo.toml` in the repo. To compile / test any `.rs` file:

1. `cargo new /tmp/temp_solution` (creates a project with `edition = "2024"` by default).
2. Prepend `struct Solution;` to the solution code, uncomment any `ListNode`/`TreeNode` structs, and write to `/tmp/temp_solution/src/main.rs`.
3. Run `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt -- --check` inside `/tmp/temp_solution`.
4. Delete `/tmp/temp_solution` when done.

### Linting

- **Trunk** (`trunk check --no-fix <file>`) runs rustfmt, shellcheck, prettier, markdownlint, checkov, shfmt, and trufflehog per `.trunk/trunk.yaml`. Use `--no-fix` to avoid interactive prompts.
- **cargo fmt / cargo clippy** work inside a temp Cargo project (see above).

### Gotchas

- `trunk check` without `--no-fix` may prompt interactively for autofixes (e.g. markdown formatting); always pass `--no-fix` in non-interactive contexts.
- Solution files reference `Solution` and sometimes `ListNode`/`TreeNode` without definitions because LeetCode provides them. You must add `struct Solution;` and uncomment the struct definitions when testing locally.

---

*Last updated: 2026-02-11 | Rust Edition: 2024*
