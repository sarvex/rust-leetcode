# Rust Agent Rules

Enterprise-grade, performant Rust solutions using **Rust 2024 edition**. Code that wins at LeetCode and ships to production.

## Rust 2024 Edition

- Use `edition = "2024"` in Cargo.toml
- Follow Rust 2024 match ergonomics rules (see [Match Ergonomics](#match-ergonomics-rust-2024))
- Run `cargo fmt` and `cargo clippy -- -D warnings` before committing

## Code Structure

```rust
impl Solution {
    /// Brief description of the approach.
    ///
    /// # Intuition
    /// Key insight that unlocks the solution.
    ///
    /// # Approach
    /// Algorithm explanation with key steps.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn solve(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|n| **n > 0).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::solve(vec![1, -2, 3]), 4);
    }

    #[test]
    fn test_edge_empty() {
        assert_eq!(Solution::solve(vec![]), 0);
    }
}
```

## Naming

| Type                 | Convention             | Example                    |
| -------------------- | ---------------------- | -------------------------- |
| Variables, functions | `snake_case`           | `num_to_index`, `find_max` |
| Types, traits, enums | `PascalCase`           | `TreeNode`, `ListNode`     |
| Constants            | `SCREAMING_SNAKE_CASE` | `MOD`, `MAX_SIZE`          |

Use descriptive names: `char_count` over `map`, `left_sum` over `ls`.

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
let prefix: Vec<_> = nums.iter().take_while(|x| **x > 0).collect();

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

## Match Ergonomics (Rust 2024)

Rust 2024 disallows reference patterns (`&`) when default binding mode is already `ref`:

```rust
// ERROR in Rust 2024
.filter(|(i, &c)| ...)      // & inside implicit ref binding
.take_while(|&&c| ...)      // same issue

// CORRECT: dereference in body
.filter(|(i, c)| *c == target)
.take_while(|c| **c == target)

// CORRECT: explicit outer pattern
.filter(|&(i, c)| c == target)
```

Run `cargo fix --edition` to auto-migrate.

## Performance Patterns

### Preallocate Collections

```rust
let mut result = Vec::with_capacity(n);
let mut seen = HashMap::with_capacity(n);
```

### Avoid Unnecessary Cloning

```rust
// Prefer
fn process(s: &str) -> usize { s.len() }

// Over
fn process(s: String) -> usize { s.len() }
```

### Use Entry API for HashMap

```rust
*freq.entry(key).or_insert(0) += 1;
```

### Bit Manipulation

```rust
// Check if power of 2
n > 0 && (n & (n - 1)) == 0

// Count set bits
n.count_ones()

// XOR for finding single element
nums.iter().fold(0, |acc, x| acc ^ x)
```

## Common Algorithm Patterns

### Two Pointers

```rust
let (mut l, mut r) = (0, nums.len() - 1);
while l < r {
    match nums[l] + nums[r] {
        sum if sum == target => return vec![l, r],
        sum if sum < target => l += 1,
        _ => r -= 1,
    }
}
```

### Sliding Window

```rust
let (mut start, mut end, mut best) = (0, 0, 0);
while end < nums.len() {
    // expand: add nums[end] to window state
    while /* window invalid */ {
        // shrink: remove nums[start] from state
        start += 1;
    }
    best = best.max(end - start + 1);
    end += 1;
}
```

### Binary Search

```rust
// Standard library
nums.binary_search(&target).unwrap_or_else(|i| i)

// Custom bounds
let idx = nums.partition_point(|&x| x < target);
```

### DFS/BFS

```rust
// DFS recursive
fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    node.map_or(0, |n| {
        let n = n.borrow();
        1 + dfs(n.left.clone()).max(dfs(n.right.clone()))
    })
}

// BFS iterative
let mut queue = VecDeque::from([root]);
while let Some(node) = queue.pop_front() {
    // process, then queue.push_back(children)
}
```

### Dynamic Programming

```rust
// Bottom-up with space optimization
let (mut prev, mut curr) = (0, 1);
for i in 2..=n {
    (prev, curr) = (curr, prev + curr);
}
```

### Union-Find

```rust
fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]); // path compression
    }
    parent[x]
}

fn union(parent: &mut [usize], rank: &mut [usize], x: usize, y: usize) {
    let (px, py) = (find(parent, x), find(parent, y));
    if px != py {
        match rank[px].cmp(&rank[py]) {
            std::cmp::Ordering::Less => parent[px] = py,
            std::cmp::Ordering::Greater => parent[py] = px,
            std::cmp::Ordering::Equal => { parent[py] = px; rank[px] += 1; }
        }
    }
}
```

## Error Handling

**LeetCode context**: Constraints are guaranteed. Use direct indexing and `unwrap()` when the problem guarantees validity.

**Production context**: Use `Result<T, E>` with `?` propagation:

```rust
fn parse_input(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}
```

## Safety Guidelines

- Prefer safe abstractions; use `unsafe` only when necessary and isolate it
- Validate inputs at system boundaries
- Use `#[cfg(debug_assertions)]` for expensive runtime checks during development

```rust
// Isolated unsafe with safety comment
// SAFETY: index is bounds-checked above
unsafe { *slice.get_unchecked(index) }
```

## Testing Checklist

Every solution needs:

- [ ] Example cases from problem statement
- [ ] Edge cases: empty input, single element, all same values
- [ ] Boundary conditions: i32::MIN, i32::MAX, large n
- [ ] Negative cases where applicable

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::solve(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_edge_two_elements() {
        assert_eq!(Solution::solve(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Solution::solve(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }
}
```

## Tooling

```bash
# Format and lint
cargo fmt && cargo clippy -- -D warnings

# Run tests
cargo test

# Check for security vulnerabilities
cargo audit

# Benchmark (for optimization work)
cargo bench
```

## Quick Reference

| Need            | Use                                                |
| --------------- | -------------------------------------------------- |
| O(1) lookup     | `HashMap`, `HashSet`                               |
| Sorted keys     | `BTreeMap`, `BTreeSet`                             |
| Queue (FIFO)    | `VecDeque`                                         |
| Stack (LIFO)    | `Vec` with `push`/`pop`                            |
| Priority queue  | `BinaryHeap` (max-heap; wrap in `Reverse` for min) |
| Counting        | `HashMap<K, usize>` with entry API                 |
| Memoization     | `HashMap<State, Result>`                           |
| Graph adjacency | `Vec<Vec<usize>>` or `HashMap<usize, Vec<usize>>`  |
