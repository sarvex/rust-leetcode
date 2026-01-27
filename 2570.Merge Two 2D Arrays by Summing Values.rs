impl Solution {
    /// Merge two sorted 2D arrays by summing values at matching IDs.
    ///
    /// # Intuition
    /// Both arrays are sorted by ID. A counting array accumulates values,
    /// then we collect non-zero entries.
    ///
    /// # Approach
    /// 1. Use a fixed-size counting array indexed by ID
    /// 2. Accumulate values from both input arrays
    /// 3. Collect all IDs with positive sums into the result
    ///
    /// # Complexity
    /// - Time: O(n + m + k) where k is the ID range
    /// - Space: O(k)
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cnt = vec![0i32; 1001];

        for x in &nums1 {
            cnt[x[0] as usize] += x[1];
        }
        for x in &nums2 {
            cnt[x[0] as usize] += x[1];
        }

        cnt.iter()
            .enumerate()
            .filter(|(_, &v)| v > 0)
            .map(|(i, &v)| vec![i as i32, v])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapping_ids() {
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]],
            ),
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
        );
    }

    #[test]
    fn test_disjoint_ids() {
        assert_eq!(
            Solution::merge_arrays(vec![vec![1, 1]], vec![vec![2, 2]]),
            vec![vec![1, 1], vec![2, 2]]
        );
    }
}
