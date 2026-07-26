impl Solution {
    /// Count unique XOR triplet values using bit-coverage insight on permutations.
    ///
    /// # Intuition
    /// Since `nums` is a permutation of `[1, n]`, every value in `{1..=n}` appears exactly once.
    /// XOR triplets `nums[i] XOR nums[j] XOR nums[k]` with `i <= j <= k` can use repeated
    /// elements (when indices coincide). Setting `i = j` gives `a XOR b` for any pair, and
    /// setting all equal gives any single element. The critical observation is:
    ///
    /// - For `n == 1`: only `1 XOR 1 XOR 1 = 1` → answer is 1.
    /// - For `n == 2`: values `{1, 2}` are reachable but not 0 → answer is 2.
    /// - For `n >= 3`: `1 XOR 2 XOR 3 = 0` is reachable, and with all of `{1..n}` available,
    ///   every value from `0` to `2^k - 1` is reachable, where `2^k` is the smallest power
    ///   of 2 strictly greater than `n`.
    ///
    /// # Approach
    /// The permutation `[1..n]` contains enough distinct values to fill all bit patterns up to
    /// the highest set bit of `n`. For `n >= 3`, the reachable XOR set is `{0, 1, ..., 2^k - 1}`
    /// where `k = ⌈log2(n + 1)⌉`. This equals the next power of 2 above `n`.
    ///
    /// # Complexity
    /// - Time: O(1) — only examines `n = nums.len()`
    /// - Space: O(1)
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        match n {
            1 => 1,
            2 => 2,
            _ => {
                // Smallest power of 2 strictly greater than n covers all reachable XOR values.
                let mut power = 1i32;
                while power <= n as i32 {
                    power <<= 1;
                }
                power
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // n=2, unique XOR values are {1, 2}
        assert_eq!(Solution::unique_xor_triplets(vec![1, 2]), 2);
    }

    #[test]
    fn test_example_2() {
        // n=3, unique XOR values are {0, 1, 2, 3}
        assert_eq!(Solution::unique_xor_triplets(vec![3, 1, 2]), 4);
    }

    #[test]
    fn test_single_element() {
        // n=1, only 1 XOR 1 XOR 1 = 1
        assert_eq!(Solution::unique_xor_triplets(vec![1]), 1);
    }

    #[test]
    fn test_n_4() {
        // n=4, next power of 2 above 4 is 8, but 4 itself is a power of 2
        // smallest power strictly > 4 is 8
        assert_eq!(Solution::unique_xor_triplets(vec![2, 1, 4, 3]), 8);
    }

    #[test]
    fn test_n_5() {
        // n=5, next power of 2 above 5 is 8
        assert_eq!(Solution::unique_xor_triplets(vec![1, 2, 3, 4, 5]), 8);
    }

    #[test]
    fn test_n_8() {
        // n=8, next power of 2 strictly above 8 is 16
        let nums: Vec<i32> = (1..=8).collect();
        assert_eq!(Solution::unique_xor_triplets(nums), 16);
    }

    #[test]
    fn test_large_n() {
        // n=100000, next power of 2 above 100000 is 131072
        let nums: Vec<i32> = (1..=100_000).collect();
        assert_eq!(Solution::unique_xor_triplets(nums), 131_072);
    }
}
