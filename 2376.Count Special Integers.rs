impl Solution {
    /// Count special integers in [1, n] where all digits are distinct
    ///
    /// # Intuition
    /// A brute force approach checking each number would be too slow for n up to 2×10⁹.
    /// Instead, use combinatorics: count numbers with fewer digits, then handle
    /// numbers with the same digit count using a prefix-based counting technique.
    ///
    /// # Approach
    /// 1. Count all k-digit special numbers where k < len(n) using permutations
    /// 2. For numbers with exactly len(n) digits, iterate through each position:
    ///    - Count numbers where a smaller digit is placed at current position
    ///    - Track used digits with a bitmask to ensure distinctness
    /// 3. If n itself is special, include it in the count
    ///
    /// # Complexity
    /// - Time: O(d²) where d is the number of digits in n (at most 10)
    /// - Space: O(d) for storing digits
    pub fn count_special_numbers(n: i32) -> i32 {
        let digits: Vec<usize> = n.to_string().bytes().map(|b| (b - b'0') as usize).collect();
        let len = digits.len();

        let mut count = 0usize;

        // Count special numbers with fewer digits than n
        for num_digits in 1..len {
            // First digit: 9 choices (1-9), remaining: permutation of unused digits
            count += 9 * Self::permutation(9, num_digits - 1);
        }

        // Count special numbers with exactly len digits that are <= n
        let mut used_mask = 0usize;

        for (pos, &digit) in digits.iter().enumerate() {
            // Try all digits smaller than current digit at this position
            let start_digit = if pos == 0 { 1 } else { 0 };

            for d in start_digit..digit {
                if used_mask & (1 << d) == 0 {
                    // Place digit d here, remaining positions can be any permutation
                    // of (10 - pos - 1) unused digits taken (len - pos - 1) at a time
                    let remaining_positions = len - pos - 1;
                    let available_digits = 10 - pos - 1;
                    count += Self::permutation(available_digits, remaining_positions);
                }
            }

            // Check if current digit is already used (would create duplicate)
            if used_mask & (1 << digit) != 0 {
                break;
            }

            // Mark current digit as used and continue
            used_mask |= 1 << digit;

            // If we've successfully placed all digits, n itself is special
            if pos == len - 1 {
                count += 1;
            }
        }

        count as i32
    }

    /// Computes P(n, r) = n! / (n-r)! = n × (n-1) × ... × (n-r+1)
    #[inline]
    fn permutation(n: usize, r: usize) -> usize {
        if r > n {
            return 0;
        }
        (n - r + 1..=n).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // All integers 1-20 except 11 are special
        assert_eq!(Solution::count_special_numbers(20), 19);
    }

    #[test]
    fn test_example_2() {
        // All single-digit numbers are special
        assert_eq!(Solution::count_special_numbers(5), 5);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_special_numbers(135), 110);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::count_special_numbers(1), 1);
        assert_eq!(Solution::count_special_numbers(9), 9);
    }

    #[test]
    fn test_boundary_10() {
        // 1-9 are all special, 10 is also special
        assert_eq!(Solution::count_special_numbers(10), 10);
    }

    #[test]
    fn test_repeated_digits() {
        // 1-10 special, 12-19 special (8), 20 special = 19
        // 11 is not special
        assert_eq!(Solution::count_special_numbers(11), 10);
    }

    #[test]
    fn test_large_number() {
        // Maximum constraint: 2 * 10^9
        assert_eq!(Solution::count_special_numbers(2_000_000_000), 5_974_650);
    }

    #[test]
    fn test_all_same_digits() {
        // 111 itself is not special
        assert_eq!(Solution::count_special_numbers(111), 99);
    }
}
