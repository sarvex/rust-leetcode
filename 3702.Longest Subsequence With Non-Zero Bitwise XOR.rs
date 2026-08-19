impl Solution {
    /// Greedy XOR parity observation.
    ///
    /// # Intuition
    /// The XOR of the entire array is either zero or non-zero. If non-zero,
    /// the whole array is the answer. If zero, removing any single non-zero
    /// element flips the total XOR to that element's value (non-zero), giving
    /// length `n - 1`. If every element is zero, no subsequence can have
    /// non-zero XOR.
    ///
    /// # Approach
    /// 1. Compute the XOR of all elements.
    /// 2. If the total XOR is non-zero, return `n`.
    /// 3. Otherwise, check if any element is non-zero. If so, return `n - 1`.
    /// 4. If all elements are zero, return `0`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let total_xor = nums.iter().fold(0, |acc, &x| acc ^ x);

        if total_xor != 0 {
            return n;
        }

        // Total XOR is zero — removing one non-zero element yields non-zero XOR
        if nums.iter().any(|&x| x != 0) {
            n - 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 4]), 3);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_single_nonzero() {
        assert_eq!(Solution::longest_subsequence(vec![5]), 1);
    }

    #[test]
    fn test_single_zero() {
        assert_eq!(Solution::longest_subsequence(vec![0]), 0);
    }

    #[test]
    fn test_two_identical() {
        // XOR of [7, 7] = 0, but there's a non-zero element, so answer = 1
        assert_eq!(Solution::longest_subsequence(vec![7, 7]), 1);
    }

    #[test]
    fn test_large_with_zeros() {
        // [0, 0, 1, 0] — total XOR = 1, which is non-zero, return n = 4
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 1, 0]), 4);
    }
}
