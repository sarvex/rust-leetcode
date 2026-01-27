impl Solution {
    /// Compute absolute difference between left and right prefix sums.
    ///
    /// # Intuition
    /// Maintain running left and right sums while scanning left to right.
    /// Right sum decreases as left sum increases.
    ///
    /// # Approach
    /// 1. Initialize right as total sum, left as 0
    /// 2. For each element, subtract from right, compute |left - right|, add to left
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for output
    pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right: i32 = nums.iter().sum();

        nums.iter()
            .map(|&v| {
                right -= v;
                let res = (left - right).abs();
                left += v;
                res
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::left_rigth_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::left_rigth_difference(vec![1]), vec![0]);
    }

    #[test]
    fn test_uniform() {
        assert_eq!(
            Solution::left_rigth_difference(vec![5, 5, 5]),
            vec![10, 0, 10]
        );
    }
}
