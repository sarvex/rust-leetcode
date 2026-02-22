const FACTORIAL: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

impl Solution {
    /// Returns true if some permutation of n's digits (no leading zero) is digitorial.
    ///
    /// # Intuition
    /// The sum of factorials of digits does not depend on their order. So for all
    /// permutations of n's digits, S = Σ digit! is the same. We only need to check
    /// whether we can form the number S using our digits (multiset subset, no leading zero).
    ///
    /// # Approach
    /// 1. Compute S = sum of factorials of each digit of n.
    /// 2. Count digit frequencies in n.
    /// 3. Check that the decimal digits of S use at most the available counts (multiset
    ///    of digits of S is a sub-multiset of digits of n). No need to generate permutations.
    ///
    /// # Complexity
    /// - Time: O(log n) — number of digits of n and S.
    /// - Space: O(1) — fixed-size digit counts.
    ///
    /// # Panics
    /// Never panics; n ≥ 1 and digits in 0..=9.
    pub fn is_digitorial_permutation(n: i32) -> bool {
        let mut digits_n = [0i32; 10];
        let mut s = 0i32;
        let mut x = n;
        while x > 0 {
            let d = (x % 10) as usize;
            digits_n[d] += 1;
            s += FACTORIAL[d];
            x /= 10;
        }

        if s <= 0 {
            return false;
        }

        let mut remaining = s;
        while remaining > 0 {
            let d = (remaining % 10) as usize;
            digits_n[d] -= 1;
            if digits_n[d] < 0 {
                return false;
            }
            remaining /= 10;
        }

        // Permutation must use every digit of n exactly once
        digits_n.iter().all(|&c| c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::is_digitorial_permutation(145));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::is_digitorial_permutation(10));
    }

    #[test]
    fn test_single_digit_digitorial() {
        assert!(Solution::is_digitorial_permutation(1));
        assert!(Solution::is_digitorial_permutation(2));
    }

    #[test]
    fn test_single_digit_not_digitorial() {
        assert!(!Solution::is_digitorial_permutation(3));
        assert!(!Solution::is_digitorial_permutation(4));
    }

    #[test]
    fn test_sum_digits_differ_from_n() {
        assert!(!Solution::is_digitorial_permutation(12));
    }

    #[test]
    fn test_large_n() {
        assert!(!Solution::is_digitorial_permutation(999_999_999));
    }
}
