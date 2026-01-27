impl Solution {
    /// Greedy farthest-reach tracking for jump game reachability.
    ///
    /// # Intuition
    /// If the farthest reachable index from positions `[0..i]` is at least `i`,
    /// then position `i` is reachable. Track the running maximum reach; if it
    /// ever falls behind the current index, the end is unreachable.
    ///
    /// # Approach
    /// Iterate through the array, updating the farthest reachable position as
    /// `max(farthest, i + nums[i])`. If `farthest < i` at any point, return
    /// false. Return true after the loop completes.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — scalar variable only
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = 0usize;

        for (i, &jump) in nums.iter().enumerate() {
            if farthest < i {
                return false;
            }
            farthest = farthest.max(i + jump as usize);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reachable() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn unreachable() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn single_element() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn all_zeros_except_first() {
        assert!(!Solution::can_jump(vec![1, 0, 0, 0]));
    }
}
