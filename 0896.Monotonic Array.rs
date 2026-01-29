pub struct Solution;

impl Solution {
    /// Checks if an array is monotonically increasing or decreasing.
    ///
    /// # Intuition
    /// An array is monotonic if it never both increases and decreases.
    /// Track whether ascending and descending transitions occur.
    ///
    /// # Approach
    /// Scan adjacent pairs. Flag ascending and descending transitions.
    /// If both flags are set, return false.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let (mut asc, mut desc) = (false, false);
        for pair in nums.windows(2) {
            match pair[0].cmp(&pair[1]) {
                std::cmp::Ordering::Less => asc = true,
                std::cmp::Ordering::Greater => desc = true,
                _ => {}
            }
            if asc && desc {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    }

    #[test]
    fn test_decreasing() {
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    }

    #[test]
    fn test_not_monotonic() {
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }

    #[test]
    fn test_constant() {
        assert!(Solution::is_monotonic(vec![1, 1, 1]));
    }

    #[test]
    fn test_single_element() {
        assert!(Solution::is_monotonic(vec![1]));
    }
}
