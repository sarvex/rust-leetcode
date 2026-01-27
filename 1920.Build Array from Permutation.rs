impl Solution {
    /// Builds a permutation array where ans[i] = nums[nums[i]].
    ///
    /// # Intuition
    /// Direct index mapping produces the result in a single pass.
    ///
    /// # Approach
    /// 1. Map each element through the double indirection nums[nums[i]].
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&v| nums[v as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
            vec![0, 1, 2, 4, 5, 3]
        );
    }

    #[test]
    fn test_identity() {
        assert_eq!(Solution::build_array(vec![0, 1, 2]), vec![0, 1, 2]);
    }
}
