impl Solution {
    /// Finds the maximum number of consecutive 1s using a single pass.
    ///
    /// # Intuition
    /// Track the current streak of 1s; reset on 0 and update the maximum.
    ///
    /// # Approach
    /// 1. Fold over the array, maintaining (best, current_streak).
    /// 2. On 1, increment streak; on 0, reset to 0.
    /// 3. Return the best seen.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(best, streak), &x| {
                let s = if x == 1 { streak + 1 } else { 0 };
                (best.max(s), s)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
    }

    #[test]
    fn test_no_ones() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![0, 0, 0]), 0);
    }
}
