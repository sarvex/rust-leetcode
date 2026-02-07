impl Solution {
    /// Minimum operations so the remaining suffix has no duplicates.
    ///
    /// # Intuition
    /// Scan right-to-left; the first duplicate seen is the rightmost. We must remove through that
    /// index, so min start = i+1 and k = ceil((i+1)/3) = (i/3)+1.
    ///
    /// # Approach
    /// Single pass from the end with a `seen` set. On first duplicate, return `(i/3)+1`; else 0.
    ///
    /// # Complexity
    /// - Time: O(n), with early return
    /// - Space: O(1), fixed `[bool; 100001]`
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut seen = [false; 100001];
        for (i, n) in nums.into_iter().enumerate().rev() {
            if seen[n as usize] {
                return (i / 3) as i32 + 1;
            }
            seen[n as usize] = true;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_operations(vec![3, 8, 3, 6, 5, 8]), 1);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_operations(vec![2, 2]), 1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_operations(vec![4, 3, 5, 1, 2]), 0);
    }

    #[test]
    fn test_empty_stop() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_operations(vec![1]), 0);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(Solution::min_operations(vec![7, 7]), 1);
    }
}
