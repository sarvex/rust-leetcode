impl Solution {
    /// Two-pointer greedy approach for maximum water container area.
    ///
    /// # Intuition
    /// The area between two lines is limited by the shorter line. Starting
    /// with the widest container and moving the shorter pointer inward is
    /// the only way to potentially find a taller line that increases area.
    ///
    /// # Approach
    /// Place pointers at both ends. Compute the area as `min(height[l], height[r]) * (r - l)`.
    /// Track the maximum area, then advance the pointer with the smaller height.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass with two converging pointers
    /// - Space: O(1) — scalar variables only
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;

        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            max_area = max_area.max(area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn equal_heights() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn ascending() {
        assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn descending() {
        assert_eq!(Solution::max_area(vec![5, 4, 3, 2, 1]), 6);
    }
}
