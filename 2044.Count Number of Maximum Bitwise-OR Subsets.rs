impl Solution {
    /// Counts subsets achieving the maximum bitwise OR via DFS.
    ///
    /// # Intuition
    /// Enumerate all subsets by DFS, tracking the cumulative OR. Count
    /// subsets matching the maximum possible OR (OR of all elements).
    ///
    /// # Approach
    /// 1. DFS over element choices, accumulating the OR value.
    /// 2. At each step, track the maximum OR and count of subsets reaching it.
    ///
    /// # Complexity
    /// - Time: O(2^n)
    /// - Space: O(n) recursion depth
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max_or = nums.iter().fold(0, |acc, &x| acc | x);

        fn dfs(nums: &[i32], idx: usize, current: i32, target: i32) -> i32 {
            if idx == nums.len() {
                return if current == target { 1 } else { 0 };
            }
            dfs(nums, idx + 1, current, target) + dfs(nums, idx + 1, current | nums[idx], target)
        }

        dfs(&nums, 0, 0, max_or)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
