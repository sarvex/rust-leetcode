impl Solution {
    /// Greedy BFS-like level expansion for minimum jumps to reach the end.
    ///
    /// # Intuition
    /// Each "level" of a BFS represents the positions reachable with one
    /// additional jump. Track the farthest reachable position; when the
    /// current index hits the boundary of the current level, start a new jump.
    ///
    /// # Approach
    /// Maintain `farthest` (max reachable index) and `current_end` (boundary
    /// of the current jump level). For each index before the last, update
    /// `farthest`. When the index reaches `current_end`, increment the jump
    /// count and extend the boundary to `farthest`.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — scalar variables only
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut farthest = 0;
        let mut current_end = 0;

        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == current_end {
                jumps += 1;
                current_end = farthest;
            }
        }

        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn greedy_first_jump() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }

    #[test]
    fn already_at_end() {
        assert_eq!(Solution::jump(vec![1, 2]), 1);
    }
}
