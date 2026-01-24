impl Solution {
    /// Maximum XOR of two subsequences using linear basis over GF(2).
    ///
    /// # Intuition
    /// When selecting two overlapping subsequences, elements in both cancel out in X XOR Y.
    /// Thus we partition elements into: first-only, second-only, or both/neither.
    /// X XOR Y equals the XOR of (first-only elements) XOR (second-only elements).
    /// Since all possible subset XORs form a linear subspace closed under XOR,
    /// maximizing X XOR Y means finding the maximum element in this subspace.
    ///
    /// # Approach
    /// 1. Build a linear basis using Gaussian elimination - each basis element has a unique leading bit
    /// 2. The maximum subspace element is obtained by greedily XORing basis elements to maximize result
    ///
    /// # Complexity
    /// - Time: O(n * log(max_val)) where log(max_val) ≤ 30 for values up to 10^9
    /// - Space: O(log(max_val)) for the basis array
    pub fn max_xor_subsequences(nums: Vec<i32>) -> i32 {
        const BITS: usize = 30;
        let mut basis = [0i32; BITS];

        for num in nums {
            let mut cur = num;
            for bit in (0..BITS).rev() {
                if cur >> bit & 1 == 1 {
                    if basis[bit] == 0 {
                        basis[bit] = cur;
                        break;
                    }
                    cur ^= basis[bit];
                }
            }
        }

        // Process from high to low bits for correct greedy maximization
        basis.iter().rev().fold(0, |acc, &b| acc.max(acc ^ b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_xor_subsequences(vec![1, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_xor_subsequences(vec![5, 2]), 7);
    }

    #[test]
    fn all_same_elements() {
        assert_eq!(Solution::max_xor_subsequences(vec![4, 4, 4, 4]), 4);
    }

    #[test]
    fn powers_of_two() {
        assert_eq!(Solution::max_xor_subsequences(vec![1, 2, 4, 8]), 15);
    }

    #[test]
    fn single_element_dominates() {
        assert_eq!(
            Solution::max_xor_subsequences(vec![1000000000, 1]),
            1000000001
        );
    }

    #[test]
    fn zeros_included() {
        assert_eq!(Solution::max_xor_subsequences(vec![0, 0, 7]), 7);
    }

    #[test]
    fn two_elements_min() {
        assert_eq!(Solution::max_xor_subsequences(vec![0, 0]), 0);
    }

    #[test]
    fn high_bit_dominates() {
        assert_eq!(Solution::max_xor_subsequences(vec![748, 615]), 748);
    }
}
