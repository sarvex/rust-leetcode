impl Solution {
    /// Determine whether a uniform-parity array can be constructed.
    ///
    /// # Intuition
    /// For each index we may either keep `nums1[i]` or replace it with a positive
    /// difference `nums1[i] - nums1[j]`. The parity of a difference is even when the
    /// two operands share parity and odd when they differ. Crucially, the difference
    /// must be at least `1`, so the subtracted element `nums1[j]` must be strictly
    /// smaller than `nums1[i]` and of the required parity — the ordering of values
    /// matters, not just their counts.
    ///
    /// # Approach
    /// Consider the two possible targets.
    ///
    /// - All-even: evens stay as-is, but every odd `x` must be turned even by
    ///   subtracting a strictly smaller odd. The smallest odd has no smaller odd to
    ///   use, so the target is reachable only when there are no odds at all
    ///   (`o == 0`).
    /// - All-odd: odds stay as-is, but every even `x` must be turned odd by
    ///   subtracting a strictly smaller odd. The binding case is the smallest even,
    ///   which needs an odd below it. That holds for every even exactly when the
    ///   global minimum is odd (`min_odd < min_even`), or when there are no evens
    ///   (`e == 0`).
    ///
    /// The answer is the disjunction: `o == 0 || e == 0 || min_odd < min_even`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut min_odd = i32::MAX;
        let mut min_even = i32::MAX;

        for &x in &nums1 {
            if x & 1 == 1 {
                min_odd = min_odd.min(x);
            } else {
                min_even = min_even.min(x);
            }
        }

        let no_odd = min_odd == i32::MAX;
        let no_even = min_even == i32::MAX;

        no_odd || no_even || min_odd < min_even
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Global minimum 1 is odd, so all evens can be turned odd.
        assert!(Solution::uniform_array(vec![1, 4, 7]));
    }

    #[test]
    fn test_example_2() {
        // Smallest value 2 is even and only odd 3 is larger: neither target works.
        assert!(!Solution::uniform_array(vec![2, 3]));
    }

    #[test]
    fn test_example_3() {
        // No odds at all: already all-even.
        assert!(Solution::uniform_array(vec![4, 6]));
    }

    #[test]
    fn test_single_element() {
        // n == 1: keep the element, trivially uniform.
        assert!(Solution::uniform_array(vec![7]));
        assert!(Solution::uniform_array(vec![8]));
    }

    #[test]
    fn test_all_odd() {
        // e == 0: already all-odd.
        assert!(Solution::uniform_array(vec![1, 3, 5, 7]));
    }

    #[test]
    fn test_all_even() {
        // o == 0: already all-even.
        assert!(Solution::uniform_array(vec![2, 4, 6, 8]));
    }

    #[test]
    fn test_two_odds_cannot_go_even() {
        // Odds {3, 5}: smallest odd 3 has no smaller odd, and evens {4} sit above
        // the smallest odd, so all-odd works while all-even does not — overall true.
        assert!(Solution::uniform_array(vec![3, 4, 5]));
    }

    #[test]
    fn test_even_below_all_odds_fails() {
        // Smallest value 2 is even; odds {5, 7} are all larger, so no even can be
        // converted to odd and no odd can be converted to even.
        assert!(!Solution::uniform_array(vec![2, 5, 7]));
    }

    #[test]
    fn test_odd_min_enables_all_odd() {
        // Global minimum 1 is odd, so every even has a smaller odd to subtract.
        assert!(Solution::uniform_array(vec![1, 2, 6, 10]));
    }

    #[test]
    fn test_mixed_min_even() {
        // min_even = 2 < min_odd = 3: evens block the all-odd target, and the single
        // smallest odd blocks all-even.
        assert!(!Solution::uniform_array(vec![2, 3, 8]));
    }

    #[test]
    fn test_boundary_values() {
        // Large distinct values with an odd global minimum.
        assert!(Solution::uniform_array(vec![1, 1_000_000_000, 999_999_998]));
    }
}
