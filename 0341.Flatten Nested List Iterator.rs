// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

/// Flattens a nested list structure using pre-computation via DFS.
///
/// # Intuition
/// Recursively flatten the entire structure at construction time into a
/// plain vector, then serve elements sequentially.
///
/// # Approach
/// 1. DFS through the nested list, collecting all integers into a vector.
/// 2. `next()` returns the current element and advances the index.
/// 3. `has_next()` checks if the index is within bounds.
///
/// # Complexity
/// - Time: O(n) construction, O(1) per next/has_next
/// - Space: O(n) for the flattened vector
struct NestedIterator {
    nums: Vec<i32>,
    index: usize,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut nums = Vec::new();
        Self::flatten(&nested_list, &mut nums);
        Self { nums, index: 0 }
    }

    fn next(&mut self) -> i32 {
        let val = self.nums[self.index];
        self.index += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.index < self.nums.len()
    }

    fn flatten(list: &[NestedInteger], result: &mut Vec<i32>) {
        for item in list {
            match item {
                NestedInteger::Int(x) => result.push(*x),
                NestedInteger::List(inner) => Self::flatten(inner, result),
            }
        }
    }
}
