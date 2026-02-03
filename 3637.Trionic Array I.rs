impl Solution {
    /// Single pass with monotonic phase validation.
    ///
    /// # Intuition
    /// A valid trionic array must have exactly two direction changes:
    /// increasing -> decreasing -> increasing, with each phase having length at least one.
    ///
    /// # Approach
    /// Walk the array with an index that expands each strictly monotonic phase in order.
    /// If any phase has zero length or if the final phase does not reach the end, the
    /// array cannot satisfy the required `p` and `q` indices.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 4 {
            return false;
        }

        let mut index = 0usize;

        while index + 1 < n && nums[index] < nums[index + 1] {
            index += 1;
        }
        if index == 0 {
            return false;
        }
        let peak = index;

        while index + 1 < n && nums[index] > nums[index + 1] {
            index += 1;
        }
        if index == peak {
            return false;
        }
        let valley = index;

        while index + 1 < n && nums[index] < nums[index + 1] {
            index += 1;
        }
        if index == valley {
            return false;
        }

        index == n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::is_trionic(vec![2, 1, 3]));
    }

    #[test]
    fn no_middle_decrease() {
        assert!(!Solution::is_trionic(vec![1, 2, 3, 4]));
    }

    #[test]
    fn equal_adjacent_breaks_strictness() {
        assert!(!Solution::is_trionic(vec![1, 2, 2, 1, 3]));
    }

    #[test]
    fn extra_direction_change() {
        assert!(!Solution::is_trionic(vec![1, 4, 2, 3, 1, 5]));
    }
}
