impl Solution {
    /// Determine whether a uniform-parity array can be constructed.
    ///
    /// # Intuition
    /// Each `nums2[i]` is either `nums1[i]` (same parity) or `nums1[i] - nums1[j]`
    /// whose parity is even when `nums1[i]`, `nums1[j]` share parity and odd otherwise.
    /// Whether a target (all-even or all-odd) is reachable therefore depends only on
    /// the counts of even and odd values.
    ///
    /// # Approach
    /// Let `o` be the number of odd values and `e` the number of even values.
    ///
    /// - All-even: an odd `nums1[i]` must become even, needing another odd element to
    ///   subtract (`o >= 2`). Evens keep themselves. Fails only when `o == 1`.
    /// - All-odd: an even `nums1[i]` must become odd, needing an odd element to
    ///   subtract (`o >= 1`). Odds keep themselves. Fails only when `e >= 1 && o == 0`.
    ///
    /// The answer is the disjunction of the two reachable conditions.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let odd = nums1.iter().filter(|&&x| x & 1 == 1).count();
        let even = nums1.len() - odd;

        let all_even = odd != 1;
        let all_odd = even == 0 || odd >= 1;

        all_even || all_odd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::uniform_array(vec![2, 3]));
    }

    #[test]
    fn test_example_2() {
        assert!(Solution::uniform_array(vec![4, 6]));
    }

    #[test]
    fn test_single_element() {
        // n == 1: keep the element, trivially uniform.
        assert!(Solution::uniform_array(vec![7]));
        assert!(Solution::uniform_array(vec![8]));
    }

    #[test]
    fn test_single_odd_with_evens() {
        // e = 2, o = 1: all-even fails (o == 1) but all-odd works (o >= 1).
        assert!(Solution::uniform_array(vec![2, 4, 5]));
    }

    #[test]
    fn test_all_even() {
        // o = 0: all-even trivially works.
        assert!(Solution::uniform_array(vec![2, 4, 6, 8]));
    }

    #[test]
    fn test_all_odd() {
        // e = 0: all-odd trivially works.
        assert!(Solution::uniform_array(vec![1, 3, 5, 7]));
    }

    #[test]
    fn test_two_odd_and_evens() {
        // o = 2: all-even works.
        assert!(Solution::uniform_array(vec![1, 3, 4, 6]));
    }

    #[test]
    fn test_mixed_multiple() {
        // e = 3, o = 3: both targets reachable.
        assert!(Solution::uniform_array(vec![2, 4, 6, 1, 3, 5]));
    }
}
