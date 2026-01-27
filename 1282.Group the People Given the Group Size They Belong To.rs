impl Solution {
    /// Bucket grouping by required group size.
    ///
    /// # Intuition
    /// People with the same group size belong together. Collecting indices into
    /// per-size buckets and then chunking each bucket into groups of that size
    /// produces a valid grouping.
    ///
    /// # Approach
    /// 1. Bucket each person's index by their required group size
    /// 2. For each size bucket, split into chunks of exactly that size
    /// 3. Collect all chunks as the result
    ///
    /// # Complexity
    /// - Time: O(n) for bucketing and chunking
    /// - Space: O(n) for the buckets
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let n = group_sizes.len();
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];

        for (i, &size) in group_sizes.iter().enumerate() {
            buckets[size as usize].push(i as i32);
        }

        buckets
            .into_iter()
            .enumerate()
            .filter(|(_, v)| !v.is_empty())
            .flat_map(|(size, v)| {
                v.chunks(size.max(1))
                    .map(|chunk| chunk.to_vec())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_group_sizes() {
        let result = Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]);
        assert_eq!(result.len(), 3);
        for group in &result {
            assert!(group.len() == 1 || group.len() == 3);
        }
    }

    #[test]
    fn all_size_two() {
        let result = Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]);
        assert_eq!(result.iter().map(|g| g.len()).sum::<usize>(), 6);
    }
}
