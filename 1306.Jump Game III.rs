impl Solution {
    /// Recursive DFS with in-place visited marking for zero-reachability.
    ///
    /// # Intuition
    /// From index `i` we can jump to `i + arr[i]` or `i - arr[i]`. Marking
    /// visited cells by negating their value avoids any auxiliary data structure
    /// and keeps the hot path entirely on the stack.
    ///
    /// # Approach
    /// Recurse from `start`. At each index:
    /// 1. Return `true` if `arr[i] == 0`.
    /// 2. Skip (return `false`) if already visited (value < 0) or out of bounds.
    /// 3. Negate `arr[i]` to mark visited, then recurse on both neighbours.
    /// The negation trick means visited checks are a single comparison with no
    /// extra allocation — the array itself is the visited set.
    ///
    /// # Complexity
    /// - Time: O(n) — each index visited at most once
    /// - Space: O(n) — implicit call stack depth bounded by n
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        Self::dfs(&mut arr, start as usize)
    }

    fn dfs(arr: &mut Vec<i32>, i: usize) -> bool {
        if arr[i] == 0 {
            return true;
        }
        if arr[i] < 0 {
            return false;
        }

        let jump = arr[i] as usize;
        arr[i] = -arr[i]; // mark visited

        let n = arr.len();
        (i + jump < n && Self::dfs(arr, i + jump)) || (i >= jump && Self::dfs(arr, i - jump))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_reach_via_two_paths() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
    }

    #[test]
    fn example_reach_from_start() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
    }

    #[test]
    fn example_cannot_reach() {
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
    }

    #[test]
    fn start_is_zero() {
        assert!(Solution::can_reach(vec![0, 1, 2], 0));
    }

    #[test]
    fn single_element_zero() {
        assert!(Solution::can_reach(vec![0], 0));
    }

    #[test]
    fn single_element_nonzero() {
        assert!(!Solution::can_reach(vec![1], 0));
    }

    #[test]
    fn cycle_without_zero() {
        assert!(!Solution::can_reach(vec![2, 0, 2], 2));
    }

    #[test]
    fn large_jump_to_zero() {
        assert!(Solution::can_reach(vec![4, 1, 1, 1, 0], 0));
    }
}
