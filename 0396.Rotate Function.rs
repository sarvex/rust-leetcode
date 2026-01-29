impl Solution {
    /// Maximizes the rotation function using an incremental formula.
    ///
    /// # Intuition
    /// When the array rotates by one position, F(k) changes by a predictable
    /// amount: F(k) = F(k-1) + sum - n * nums[n-k]. This avoids recomputing
    /// each rotation from scratch.
    ///
    /// # Approach
    /// 1. Compute the total sum and initial F(0).
    /// 2. Iterate through rotations, updating F incrementally.
    /// 3. Track the maximum value seen.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        let f0: i32 = nums.iter().enumerate().map(|(i, v)| i as i32 * *v).sum();
        (1..nums.len())
            .fold((f0, f0), |(prev, best), k| {
                let current = prev + sum - n * nums[nums.len() - k];
                (current, best.max(current))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::max_rotate_function(vec![100]), 0);
    }

    #[test]
    fn test_negative() {
        assert_eq!(Solution::max_rotate_function(vec![-1, -2, -3]), -4);
    }
}
