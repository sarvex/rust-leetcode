impl Solution {
    /// Computes product of array except self using prefix and suffix passes.
    ///
    /// # Intuition
    /// For each element, the result is the product of all elements to its left
    /// times all elements to its right. Two passes compute these without division.
    ///
    /// # Approach
    /// 1. Forward pass: build prefix products in the result array.
    /// 2. Backward pass: multiply each position by the running suffix product.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) excluding the output array
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];
        for i in 1..n {
            result[i] = result[i - 1] * nums[i - 1];
        }
        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn with_zero() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
