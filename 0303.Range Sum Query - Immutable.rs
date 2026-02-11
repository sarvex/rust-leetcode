struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    /// Immutable range sum query using prefix sums.
    ///
    /// # Intuition
    /// Precompute prefix sums so that any range sum is a constant-time subtraction.
    ///
    /// # Approach
    /// 1. Build a prefix sum array of length n + 1.
    /// 2. `sum_range(left, right) = prefix[right + 1] - prefix[left]`.
    ///
    /// # Complexity
    /// - Time: O(n) construction, O(1) per query
    /// - Space: O(n)
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(nums.len() + 1);
        prefix.push(0);
        let mut sum = 0;
        for num in nums {
            sum += num;
            prefix.push(sum);
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[(right + 1) as usize] - self.prefix[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_queries() {
        let arr = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(arr.sum_range(0, 2), 1);
        assert_eq!(arr.sum_range(2, 5), -1);
        assert_eq!(arr.sum_range(0, 5), -3);
    }

    #[test]
    fn single_element_query() {
        let arr = NumArray::new(vec![5, -3, 7]);
        assert_eq!(arr.sum_range(0, 0), 5);
        assert_eq!(arr.sum_range(1, 1), -3);
        assert_eq!(arr.sum_range(2, 2), 7);
    }

    #[test]
    fn full_range() {
        let arr = NumArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(arr.sum_range(0, 4), 15);
    }

    #[test]
    fn single_element_array() {
        let arr = NumArray::new(vec![42]);
        assert_eq!(arr.sum_range(0, 0), 42);
    }

    #[test]
    fn negative_numbers() {
        let arr = NumArray::new(vec![-1, -2, -3, -4]);
        assert_eq!(arr.sum_range(0, 3), -10);
        assert_eq!(arr.sum_range(1, 2), -5);
    }

    #[test]
    fn all_zeros() {
        let arr = NumArray::new(vec![0, 0, 0, 0]);
        assert_eq!(arr.sum_range(0, 3), 0);
    }
}
