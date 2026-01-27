impl Solution {
    /// Finds the earliest day all flowers fully bloom using greedy scheduling.
    ///
    /// # Intuition
    /// Planting is sequential, but growing happens in parallel. By planting
    /// flowers with the longest grow time first, we maximize overlap between
    /// growing periods and minimize the total time.
    ///
    /// # Approach
    /// 1. Create an index array and sort it by descending grow time.
    /// 2. Simulate planting in sorted order, accumulating planting days.
    /// 3. Track the maximum of `(plant_end + grow_time)` across all flowers.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut indices: Vec<usize> = (0..plant_time.len()).collect();
        indices.sort_unstable_by(|&a, &b| grow_time[b].cmp(&grow_time[a]));

        indices
            .iter()
            .fold((0, 0), |(ans, planted), &i| {
                let planted = planted + plant_time[i];
                (ans.max(planted + grow_time[i]), planted)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
            9
        );
    }

    #[test]
    fn test_single_flower() {
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }

    #[test]
    fn test_equal_grow_times() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 2, 3], vec![5, 5, 5]),
            11
        );
    }

    #[test]
    fn test_zero_grow_time() {
        assert_eq!(Solution::earliest_full_bloom(vec![3, 2], vec![0, 0]), 5);
    }
}
