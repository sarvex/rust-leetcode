use std::collections::HashSet;
use std::sync::OnceLock;

/// Generates all binary palindromes up to 15 bits (covers nums[i] ≤ 5000 and nearest above).
fn generate_binary_palindromes() -> Vec<i32> {
    let mut palindromes = HashSet::new();
    for l in 1..=15 {
        let half_len = (l + 1) / 2;
        let num_bits_to_reverse = l / 2;
        let start_i = 1 << (half_len - 1);
        let end_i = (1 << half_len) - 1;

        for i in start_i..=end_i {
            let mut temp_i = i >> (half_len - num_bits_to_reverse);
            let mut rev_val = 0i32;
            for _ in 0..num_bits_to_reverse {
                rev_val = (rev_val << 1) | (temp_i & 1);
                temp_i >>= 1;
            }
            let palindrome = (i << num_bits_to_reverse) | rev_val;
            palindromes.insert(palindrome);
        }
    }

    let mut sorted: Vec<i32> = palindromes.into_iter().collect();
    sorted.sort_unstable();
    sorted
}

static SORTED_PALINDROMES: OnceLock<Vec<i32>> = OnceLock::new();

impl Solution {
    /// For each number, return minimum operations to convert it to a binary palindrome.
    ///
    /// # Intuition
    /// A binary palindrome reads the same forward and backward. Minimum operations equal
    /// the distance to the nearest such number (above or below).
    ///
    /// # Approach
    /// 1. Generate all binary palindromes up to 15 bits once (via OnceLock); build by
    ///    length and prefix so only O(P) work instead of scanning all integers.
    /// 2. For each num, binary_search gives exact match (0 ops) or insert position;
    ///    compare distance to palindrome at insert_idx and insert_idx - 1.
    ///
    /// # Complexity
    /// - Time: O(P + N log P) with P = number of palindromes up to 15 bits
    /// - Space: O(P)
    pub fn min_operations(nums: Vec<i32>) -> Vec<i32> {
        let palindromes = SORTED_PALINDROMES.get_or_init(generate_binary_palindromes);
        let mut ans = Vec::with_capacity(nums.len());

        for &num in &nums {
            let ops = match palindromes.binary_search(&num) {
                Ok(_) => 0,
                Err(insert_idx) => {
                    let mut min_diff = i32::MAX;
                    if insert_idx < palindromes.len() {
                        min_diff = min_diff.min(palindromes[insert_idx] - num);
                    }
                    if insert_idx > 0 {
                        min_diff = min_diff.min(num - palindromes[insert_idx - 1]);
                    }
                    min_diff
                }
            };
            ans.push(ops);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::min_operations(vec![1, 2, 4]),
            vec![0, 1, 1]
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::min_operations(vec![6, 7, 12]),
            vec![1, 0, 3]
        );
    }

    #[test]
    fn test_single_already_palindrome() {
        assert_eq!(Solution::min_operations(vec![7]), vec![0]);
    }

    #[test]
    fn test_single_one_operation() {
        assert_eq!(Solution::min_operations(vec![2]), vec![1]);
    }

    #[test]
    fn test_large_values() {
        assert_eq!(Solution::min_operations(vec![5000]), vec![47]);
    }
}
