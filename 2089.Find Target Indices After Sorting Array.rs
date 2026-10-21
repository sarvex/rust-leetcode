impl Solution {
    /// Counting-based index computation without explicit sorting.
    ///
    /// # Intuition
    /// After sorting, the positions occupied by `target` form a contiguous
    /// block. That block starts right after every value strictly smaller than
    /// `target`, and extends for as many copies of `target` as the array holds.
    /// Both counts can be obtained in a single pass, so no sort is required.
    ///
    /// # Approach
    /// Scan `nums` once, tallying how many elements are strictly less than
    /// `target` (`less`) and how many equal it (`equal`). In the sorted array
    /// the target indices are exactly `less, less + 1, ..., less + equal - 1`.
    /// Emit that range.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass to count, plus O(k) to build the k results
    /// - Space: O(1) — excluding the output vector
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut less = 0i32;
        let mut equal = 0i32;

        for &num in &nums {
            if num < target {
                less += 1;
            } else if num == target {
                equal += 1;
            }
        }

        (less..less + equal).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_target_indices() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn single_target_index() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
    }

    #[test]
    fn target_is_maximum() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }

    #[test]
    fn target_absent() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 4), vec![]);
    }

    #[test]
    fn target_is_minimum() {
        assert_eq!(Solution::target_indices(vec![5, 1, 1, 1, 3], 1), vec![0, 1, 2]);
    }

    #[test]
    fn single_element_match() {
        assert_eq!(Solution::target_indices(vec![7], 7), vec![0]);
    }
}
