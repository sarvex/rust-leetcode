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
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len() as i64;
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let f0: i64 = nums
            .iter()
            .enumerate()
            .map(|(i, &v)| i as i64 * v as i64)
            .sum();
        (1..nums.len())
            .fold((f0, f0), |(prev, best), k| {
                let current = prev + sum - n * nums[nums.len() - k] as i64;
                (current, best.max(current))
            })
            .1 as i32
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

    #[test]
    fn test_empty() {
        assert_eq!(Solution::max_rotate_function(vec![]), 0);
    }

    #[test]
    fn test_two_elements() {
        // F(0) = 0*1 + 1*2 = 2
        // F(1) = 0*2 + 1*1 = 1
        assert_eq!(Solution::max_rotate_function(vec![1, 2]), 2);
    }

    #[test]
    fn test_all_same() {
        // All rotations give same result
        assert_eq!(Solution::max_rotate_function(vec![5, 5, 5, 5]), 30);
    }

    #[test]
    fn test_zeros() {
        assert_eq!(Solution::max_rotate_function(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_large_values() {
        // Use i64 internally to avoid overflow
        assert_eq!(
            Solution::max_rotate_function(vec![1000000007, 1000000007]),
            1000000007
        );
    }
}
