use std::collections::HashMap;

impl Solution {
    /// Rank transform using sorted deduplication and HashMap lookup.
    ///
    /// # Intuition
    /// Sort unique values to determine rank order, then map each element
    /// to its rank via a HashMap for O(1) lookup.
    ///
    /// # Approach
    /// 1. Collect sorted unique values from the array.
    /// 2. Build a HashMap from value → rank (1-indexed).
    /// 3. Map each original element to its rank.
    ///
    /// # Complexity
    /// - Time: O(n log n) — sorting dominates
    /// - Space: O(n) — HashMap and sorted unique values
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        sorted.dedup();

        let rank: HashMap<i32, i32> = sorted
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32 + 1))
            .collect();

        arr.iter().map(|v| rank[v]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_elements() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn test_mixed_with_duplicates() {
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::array_rank_transform(vec![]), vec![]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::array_rank_transform(vec![42]), vec![1]);
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(
            Solution::array_rank_transform(vec![-5, -1, -3]),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(
            Solution::array_rank_transform(vec![i32::MIN, 0, i32::MAX]),
            vec![1, 2, 3]
        );
    }
}
