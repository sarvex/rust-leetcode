impl Solution {
    /// Finds the maximum product subarray by tracking both max and min products.
    ///
    /// # Intuition
    /// A negative number can turn the smallest product into the largest.
    /// Tracking both the running maximum and minimum products handles sign flips.
    ///
    /// # Approach
    /// 1. Initialize max product, min product, and answer to the first element.
    /// 2. For each subsequent element, compute candidates from `x`, `x * max`, `x * min`.
    /// 3. Update max and min products, and track the global maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = nums[0];
        let mut min_prod = nums[0];
        let mut result = nums[0];
        for &x in nums.iter().skip(1) {
            let (prev_max, prev_min) = (max_prod, min_prod);
            max_prod = x.max(x * prev_max).max(x * prev_min);
            min_prod = x.min(x * prev_max).min(x * prev_min);
            result = result.max(max_prod);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_subarray() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn negative_flips() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(Solution::max_product(vec![-2, -3, -4]), 12);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_product(vec![-2]), -2);
    }
}
