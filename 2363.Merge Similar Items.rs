impl Solution {
    /// Merges two item lists by summing weights of matching values.
    ///
    /// # Intuition
    /// Since values are bounded (1..=1000), a counting array provides O(1) access
    /// for accumulation and natural sorted output via index order.
    ///
    /// # Approach
    /// 1. Accumulate weights into a fixed-size array indexed by value
    /// 2. Filter non-zero entries and collect as result pairs
    ///
    /// # Complexity
    /// - Time: O(n + m + V) where V = 1001
    /// - Space: O(V)
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut weight = [0i32; 1001];
        for item in items1.iter().chain(items2.iter()) {
            weight[item[0] as usize] += item[1];
        }
        weight
            .iter()
            .enumerate()
            .filter(|(_, &w)| w > 0)
            .map(|(val, &w)| vec![val as i32, w])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let items1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
        let items2 = vec![vec![3, 1], vec![1, 5]];
        assert_eq!(
            Solution::merge_similar_items(items1, items2),
            vec![vec![1, 6], vec![3, 9], vec![4, 5]]
        );
    }

    #[test]
    fn test_no_overlap() {
        let items1 = vec![vec![1, 1]];
        let items2 = vec![vec![2, 2]];
        assert_eq!(
            Solution::merge_similar_items(items1, items2),
            vec![vec![1, 1], vec![2, 2]]
        );
    }

    #[test]
    fn test_complete_overlap() {
        let items1 = vec![vec![1, 3]];
        let items2 = vec![vec![1, 7]];
        assert_eq!(
            Solution::merge_similar_items(items1, items2),
            vec![vec![1, 10]]
        );
    }
}
