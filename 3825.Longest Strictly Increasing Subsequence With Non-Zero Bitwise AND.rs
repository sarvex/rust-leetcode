impl Solution {
    /// Maximize LIS length by selecting a shared set bit.
    ///
    /// # Intuition
    /// A bitwise AND is non-zero iff some bit is set in every element. That means any valid
    /// subsequence must lie entirely in the elements that share at least one common set bit.
    ///
    /// # Approach
    /// For each bit position (0..=30), filter elements that have the bit set and compute the
    /// length of the strictly increasing subsequence via the patience (tails) method. The best
    /// answer across all bits is the result. If no bit is present in any element, the maximum
    /// remains zero.
    ///
    /// # Complexity
    /// - Time: O(31 * n log n)
    /// - Space: O(n)
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        const MAX_BITS: usize = 31;
        let mut best = 0usize;

        for bit in 0..MAX_BITS {
            let mut tails: Vec<i32> = Vec::new();

            for &value in &nums {
                if ((value as u32) >> bit) & 1 == 0 {
                    continue;
                }
                let idx = tails.partition_point(|v| *v < value);
                if idx == tails.len() {
                    tails.push(value);
                } else {
                    tails[idx] = value;
                }
            }

            best = best.max(tails.len());
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_subsequence(vec![5, 4, 7]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 6]), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_subsequence(vec![0, 1]), 1);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_duplicates_strictly_increasing() {
        assert_eq!(Solution::longest_subsequence(vec![4, 4, 4]), 1);
    }

    #[test]
    fn test_lis_blocked_by_and() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
    }
}
