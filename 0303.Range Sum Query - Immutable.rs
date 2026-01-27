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
struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
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
}
