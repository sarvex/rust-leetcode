impl Solution {
    /// Counts special integers in [1, n] where all digits are distinct.
    ///
    /// # Intuition
    /// Use combinatorics: count numbers with fewer digits via permutations,
    /// then handle same-length numbers with a prefix-based digit-tracking technique.
    ///
    /// # Approach
    /// 1. Count all k-digit special numbers where k < len(n) using permutations
    /// 2. For same-length numbers, iterate positions with a bitmask of used digits
    /// 3. At each position, count valid completions for smaller digits
    /// 4. If n itself has all distinct digits, include it
    ///
    /// # Complexity
    /// - Time: O(d²) where d is the number of digits in n (at most 10)
    /// - Space: O(d)
    pub fn count_special_numbers(n: i32) -> i32 {
        let digits: Vec<usize> = n.to_string().bytes().map(|b| (b - b'0') as usize).collect();
        let len = digits.len();
        let mut count = 0usize;

        // Count special numbers with fewer digits
        for num_digits in 1..len {
            count += 9 * Self::permutation(9, num_digits - 1);
        }

        // Count special numbers with exactly len digits that are <= n
        let mut used_mask = 0usize;

        for (pos, &digit) in digits.iter().enumerate() {
            let start_digit = if pos == 0 { 1 } else { 0 };

            for d in start_digit..digit {
                if used_mask & (1 << d) == 0 {
                    let remaining = len - pos - 1;
                    let available = 10 - pos - 1;
                    count += Self::permutation(available, remaining);
                }
            }

            if used_mask & (1 << digit) != 0 {
                break;
            }
            used_mask |= 1 << digit;

            if pos == len - 1 {
                count += 1;
            }
        }

        count as i32
    }

    /// Computes P(n, r) = n! / (n-r)!
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
        assert_eq!(Solution::count_special_numbers(20), 19);
    }

    #[test]
    fn test_example_2() {
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
        assert_eq!(Solution::count_special_numbers(10), 10);
    }

    #[test]
    fn test_repeated_digits() {
        assert_eq!(Solution::count_special_numbers(11), 10);
    }

    #[test]
    fn test_large_number() {
        assert_eq!(Solution::count_special_numbers(2_000_000_000), 5_974_650);
    }

    #[test]
    fn test_all_same_digits() {
        assert_eq!(Solution::count_special_numbers(111), 99);
    }
}
