impl Solution {
    /// Counts palindromic subsequences of length 5 in a digit string.
    ///
    /// # Intuition
    /// A palindrome of length 5 has the form d1-d2-d3-d2-d1 where:
    /// - Position 0 and 4 have the same digit (d1)
    /// - Position 1 and 3 have the same digit (d2)
    /// - Position 2 is any digit (d3, the middle)
    ///
    /// # Approach
    /// 1. Initialize suffix counts for the entire string (singles and pairs)
    /// 2. Maintain a running `matched_pairs` count tracking valid palindrome contributions
    /// 3. For each position as potential middle:
    ///    - Remove current char from suffix (it becomes the middle)
    ///    - Add accumulated result
    ///    - Add current char to prefix and update matched_pairs
    ///
    /// # Complexity
    /// - Time: O(n × D) where n is string length and D = 10 (digits)
    /// - Space: O(D²) = O(100) = O(1) constant space
    pub fn count_palindromes(s: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        const DIGITS: usize = 10;

        let chars: Vec<usize> = s.bytes().map(|b| (b - b'0') as usize).collect();
        let pair = |d1: usize, d2: usize| d1 * DIGITS + d2;

        let mut prefix_singles = [0u64; DIGITS];
        let mut prefix_pairs = [0u64; DIGITS * DIGITS];
        let mut suffix_singles = [0u64; DIGITS];
        let mut suffix_pairs = [0u64; DIGITS * DIGITS];

        for &ch in chars.iter().rev() {
            for d in 0..DIGITS {
                suffix_pairs[pair(ch, d)] += suffix_singles[d];
            }
            suffix_singles[ch] += 1;
        }

        let mut result: u64 = 0;
        let mut matched_pairs: u64 = 0;

        for &mid in &chars {
            suffix_singles[mid] -= 1;
            for d in 0..DIGITS {
                let right_pair = pair(mid, d);
                let left_pair = pair(d, mid);
                matched_pairs -= prefix_pairs[left_pair] * suffix_singles[d];
                suffix_pairs[right_pair] -= suffix_singles[d];
            }

            result = (result + matched_pairs) % MOD;

            for d in 0..DIGITS {
                let left_pair = pair(d, mid);
                let right_pair = pair(mid, d);
                matched_pairs += prefix_singles[d] * suffix_pairs[right_pair];
                prefix_pairs[left_pair] += prefix_singles[d];
            }
            prefix_singles[mid] += 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_with_repeated_digits() {
        assert_eq!(Solution::count_palindromes("103301".to_string()), 2);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::count_palindromes("0000000".to_string()), 21);
    }

    #[test]
    fn test_two_groups() {
        assert_eq!(Solution::count_palindromes("9999900000".to_string()), 2);
    }

    #[test]
    fn test_too_short() {
        assert_eq!(Solution::count_palindromes("1234".to_string()), 0);
    }

    #[test]
    fn test_exact_five_palindrome() {
        assert_eq!(Solution::count_palindromes("12321".to_string()), 1);
    }

    #[test]
    fn test_exact_five_no_palindrome() {
        assert_eq!(Solution::count_palindromes("12345".to_string()), 0);
    }

    #[test]
    fn test_single_digit_repeated_five() {
        assert_eq!(Solution::count_palindromes("11111".to_string()), 1);
    }

    #[test]
    fn test_single_digit_repeated_six() {
        assert_eq!(Solution::count_palindromes("111111".to_string()), 6);
    }

    #[test]
    fn test_all_unique_digits() {
        assert_eq!(Solution::count_palindromes("12345678".to_string()), 0);
    }
}
