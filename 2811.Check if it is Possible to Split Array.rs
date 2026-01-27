impl Solution {
    /// Check if the array can be split into single elements via valid operations.
    ///
    /// # Intuition
    /// For arrays of length ≤ 2, splitting is always possible. For longer arrays,
    /// at least one adjacent pair must sum to ≥ m, because every split sequence
    /// must eventually isolate elements by splitting a pair.
    ///
    /// # Approach
    /// 1. If `n ≤ 2`, return true.
    /// 2. Check if any adjacent pair sums to ≥ m.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        let n = nums.len();
        match n <= 2 {
            true => true,
            false => nums.windows(2).any(|w| w[0] + w[1] >= m),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splittable_array() {
        assert!(Solution::can_split_array(vec![2, 2, 1], 4));
    }

    #[test]
    fn not_splittable() {
        assert!(!Solution::can_split_array(vec![2, 1, 3], 5));
    }

    #[test]
    fn two_elements_always_splittable() {
        assert!(Solution::can_split_array(vec![1, 1], 100));
    }
}
