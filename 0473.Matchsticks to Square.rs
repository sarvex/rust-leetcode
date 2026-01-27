impl Solution {
    /// Determines if matchsticks can form a square using backtracking.
    ///
    /// # Intuition
    /// Partition the matchsticks into 4 groups of equal sum. Sorting in
    /// descending order prunes impossible branches early.
    ///
    /// # Approach
    /// 1. Check if the total sum is divisible by 4.
    /// 2. Sort matchsticks in descending order for early pruning.
    /// 3. Backtrack: try placing each matchstick into one of four sides,
    ///    skipping duplicate side lengths to avoid redundant work.
    ///
    /// # Complexity
    /// - Time: O(4^n) worst case, heavily pruned in practice
    /// - Space: O(n) recursion depth
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let side = sum / 4;
        matchsticks.sort_unstable_by(|a, b| b.cmp(a));

        fn backtrack(sticks: &[i32], sides: &mut [i32; 4], idx: usize, target: i32) -> bool {
            if idx == sticks.len() {
                return true;
            }
            for i in 0..4 {
                if i > 0 && sides[i - 1] == sides[i] {
                    continue;
                }
                sides[i] += sticks[idx];
                if sides[i] <= target && backtrack(sticks, sides, idx + 1, target) {
                    return true;
                }
                sides[i] -= sticks[idx];
            }
            false
        }

        let mut sides = [0i32; 4];
        backtrack(&matchsticks, &mut sides, 0, side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_form() {
        assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]));
    }

    #[test]
    fn test_cannot_form() {
        assert!(!Solution::makesquare(vec![3, 3, 3, 3, 4]));
    }

    #[test]
    fn test_single() {
        assert!(!Solution::makesquare(vec![5]));
    }
}
