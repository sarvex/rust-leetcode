impl Solution {
    /// Counts Binary Palindromic Numbers up to n
    ///
    /// # Intuition
    /// Binary palindromes have a structure where the first half determines the entire number.
    /// Instead of checking each number individually, we can generate and count all possible
    /// palindromes efficiently by iterating over their "half" representations.
    ///
    /// # Approach
    /// 1. Handle 0 as a special case (it's a palindrome)
    /// 2. For each bit length shorter than n's length, all palindromes are valid - count them
    ///    using the formula: 2^floor((len-1)/2) palindromes per length
    /// 3. For palindromes with the same bit length as n, use binary search on the "half" value
    ///    to find the maximum half that produces a palindrome ≤ n
    ///
    /// # Complexity
    /// - Time: O(log²n) - O(log n) bit lengths, O(log n) binary search for same-length palindromes
    /// - Space: O(1)
    pub fn count_binary_palindromes(n: i64) -> i32 {
        if n == 0 {
            return 1;
        }

        let n_bits = (64 - n.leading_zeros()) as usize;

        // Count 0 plus all palindromes with fewer bits than n
        let count_shorter: i64 = 1 + (1..n_bits).map(|len| 1i64 << ((len - 1) / 2)).sum::<i64>();

        // Count palindromes with same bit length as n that are ≤ n
        let count_same_len = Self::count_palindromes_with_length(n, n_bits);

        (count_shorter + count_same_len) as i32
    }

    /// Counts palindromes of exactly `len` bits that are ≤ n
    fn count_palindromes_with_length(n: i64, len: usize) -> i64 {
        let half_len = (len + 1) / 2;
        let min_half = 1i64 << (half_len - 1);
        let max_half = (1i64 << half_len) - 1;

        // Binary search for largest half where palindrome ≤ n
        let (mut lo, mut hi) = (min_half, max_half);
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if Self::make_palindrome(mid, len) <= n {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        if Self::make_palindrome(lo, len) <= n {
            lo - min_half + 1
        } else {
            0
        }
    }

    /// Constructs a palindrome of given length from its first half
    fn make_palindrome(half: i64, len: usize) -> i64 {
        let half_len = (len + 1) / 2;
        let base = half << (len - half_len);

        // Mirror bits from the first half to complete the palindrome
        (0..len / 2).fold(base, |result, i| {
            let bit = (half >> (half_len - 1 - i)) & 1;
            result | (bit << i)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Palindromes: 0, 1, 3, 5, 7, 9
        assert_eq!(Solution::count_binary_palindromes(9), 6);
    }

    #[test]
    fn test_example_2() {
        // Only "0" is a palindrome
        assert_eq!(Solution::count_binary_palindromes(0), 1);
    }

    #[test]
    fn test_small_values() {
        // 0 -> "0" (palindrome)
        assert_eq!(Solution::count_binary_palindromes(0), 1);

        // 0, 1 -> "0", "1"
        assert_eq!(Solution::count_binary_palindromes(1), 2);

        // 0, 1, (2 is "10" - not palindrome)
        assert_eq!(Solution::count_binary_palindromes(2), 2);

        // 0, 1, 3 ("11")
        assert_eq!(Solution::count_binary_palindromes(3), 3);

        // 0, 1, 3, (4 is "100" - not palindrome)
        assert_eq!(Solution::count_binary_palindromes(4), 3);

        // 0, 1, 3, 5 ("101")
        assert_eq!(Solution::count_binary_palindromes(5), 4);
    }

    #[test]
    fn test_powers_of_two() {
        // 2^4 = 16: palindromes are 0,1,3,5,7,9,15 = 7
        assert_eq!(Solution::count_binary_palindromes(16), 7);

        // 2^5 = 32: adds 17("10001"), 21("10101"), 27("11011"), 31("11111") = 11
        // Note: 32 itself ("100000") is not a palindrome
        assert_eq!(Solution::count_binary_palindromes(32), 11);
    }

    #[test]
    fn test_large_value() {
        // Test with a large value to ensure no overflow
        let result = Solution::count_binary_palindromes(1_000_000_000_000_000);
        assert!(result > 0);
    }

    #[test]
    fn test_make_palindrome() {
        // half=2 (10), len=4 -> 1001 = 9
        assert_eq!(Solution::make_palindrome(2, 4), 9);

        // half=3 (11), len=4 -> 1111 = 15
        assert_eq!(Solution::make_palindrome(3, 4), 15);

        // half=5 (101), len=5 -> 10101 = 21
        assert_eq!(Solution::make_palindrome(5, 5), 21);

        // half=5 (101), len=6 -> 101101 = 45
        assert_eq!(Solution::make_palindrome(5, 6), 45);
    }
}
