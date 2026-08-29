impl Solution {
    /// Sort-and-group by reachable swap chains to place smallest values first.
    ///
    /// # Intuition
    /// Two elements can be swapped directly if their absolute difference is at
    /// most `limit`. By transitivity, if we sort the values, any run of
    /// consecutive sorted values whose adjacent gaps are all `<= limit` forms a
    /// single connected group: every element in that group is mutually
    /// reachable and can be freely rearranged among the original positions its
    /// members occupy. To get the lexicographically smallest array, each such
    /// group should place its smallest available value at its smallest original
    /// index.
    ///
    /// # Approach
    /// 1. Build an index permutation `order` sorted by value.
    /// 2. Walk `order`, breaking into groups whenever the gap between adjacent
    ///    sorted values exceeds `limit`.
    /// 3. For each group, collect the original positions, sort those positions
    ///    ascending, and assign the group's values (already ascending in sorted
    ///    order) to them. The smallest value lands at the smallest index.
    ///
    /// # Complexity
    /// - Time: O(n log n) — dominated by the two sorts
    /// - Space: O(n) — index permutation and result buffer
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| nums[i]);

        let mut result = vec![0; n];
        let mut start = 0;

        while start < n {
            let mut end = start + 1;
            while end < n && nums[order[end]] - nums[order[end - 1]] <= limit {
                end += 1;
            }

            let mut positions: Vec<usize> = order[start..end].to_vec();
            positions.sort_unstable();

            for (slot, &pos) in positions.iter().enumerate() {
                result[pos] = nums[order[start + slot]];
            }

            start = end;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            vec![1, 6, 7, 18, 1, 2]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            vec![1, 7, 28, 19, 10]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![42], 5),
            vec![42]
        );
    }

    #[test]
    fn test_all_connected() {
        // All values within limit of their neighbors when sorted -> one group.
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![5, 4, 3, 2, 1], 1),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_none_connected() {
        // Large gaps -> every element isolated -> unchanged.
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![10, 1, 100], 2),
            vec![10, 1, 100]
        );
    }

    #[test]
    fn test_duplicates() {
        // Equal values are always swappable (gap 0 <= limit).
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![3, 3, 3], 1),
            vec![3, 3, 3]
        );
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 1_000_000_000], 999_999_999),
            vec![1, 1_000_000_000]
        );
    }
}
