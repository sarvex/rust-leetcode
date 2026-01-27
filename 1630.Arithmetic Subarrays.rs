impl Solution {
    /// Sort-and-check for each queried subarray.
    ///
    /// # Intuition
    /// For each query `[l, r]`, extract the subarray, sort it, and verify
    /// constant consecutive differences. This determines if the subarray
    /// can be rearranged into an arithmetic sequence.
    ///
    /// # Approach
    /// 1. For each query, extract and sort the subarray
    /// 2. Check that all consecutive differences are equal using `windows(3)`
    ///
    /// # Complexity
    /// - Time: O(m · k log k) where m = queries, k = subarray length
    /// - Space: O(k) per query
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.iter()
            .zip(r.iter())
            .map(|(&li, &ri)| {
                let mut sub = nums[li as usize..=ri as usize].to_vec();
                sub.sort_unstable();
                sub.windows(3).all(|w| w[0] - w[1] == w[1] - w[2])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_results() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5],
            ),
            vec![true, false, true]
        );
    }

    #[test]
    fn all_arithmetic() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(vec![1, 2, 3, 4], vec![0], vec![3]),
            vec![true]
        );
    }
}
