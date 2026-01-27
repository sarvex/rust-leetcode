impl Solution {
    /// Two-pointer approach for trapping rain water calculation.
    ///
    /// # Intuition
    /// Water trapped above a bar depends on the minimum of the tallest bars to
    /// its left and right. Instead of precomputing prefix arrays, two pointers
    /// converging from both ends can maintain running maximums and compute the
    /// trapped water in a single pass.
    ///
    /// # Approach
    /// Place pointers at both ends of the elevation map with their respective
    /// running max heights. At each step, advance the pointer with the smaller
    /// max — the trapped water at that position is determined by the smaller
    /// side regardless of future bars on the taller side. Accumulate the
    /// difference between the current running max and the bar height.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass with two converging pointers
    /// - Space: O(1) — only scalar variables
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len().wrapping_sub(1));
        let (mut left_max, mut right_max) = (0, 0);
        let mut trapped = 0;

        while left < right {
            if height[left] < height[right] {
                left_max = left_max.max(height[left]);
                trapped += left_max - height[left];
                left += 1;
            } else {
                right_max = right_max.max(height[right]);
                trapped += right_max - height[right];
                right -= 1;
            }
        }

        trapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_elevation() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn plateau_with_dips() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn ascending_only() {
        assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn descending_only() {
        assert_eq!(Solution::trap(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn single_bar() {
        assert_eq!(Solution::trap(vec![5]), 0);
    }

    #[test]
    fn empty_elevation() {
        assert_eq!(Solution::trap(vec![]), 0);
    }
}
