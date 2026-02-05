impl Solution {
    /// Precompute XOR-score and range maximum tables.
    ///
    /// # Intuition
    /// Repeated adjacent XOR reduction follows Pascal coefficients modulo 2, so a subarray
    /// score satisfies `score[l][r] = score[l][r - 1] ^ score[l + 1][r]`.
    ///
    /// # Approach
    /// - Build `score[l][r]` for all subarrays using the XOR recurrence.
    /// - Build `best[l][r]` as the maximum score over any subarray inside `[l, r]` using
    ///   `best[l][r] = max(score[l][r], best[l + 1][r], best[l][r - 1])`.
    /// - Answer each query in O(1) by indexing `best`.
    ///
    /// # Complexity
    /// - Time: O(n^2 + q)
    /// - Space: O(n^2)
    pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut score = vec![vec![0_i32; n]; n];
        let mut best = vec![vec![0_i32; n]; n];

        for i in 0..n {
            score[i][i] = nums[i];
            best[i][i] = nums[i];
        }

        for len in 2..=n {
            for left in 0..=n - len {
                let right = left + len - 1;
                score[left][right] = score[left][right - 1] ^ score[left + 1][right];

                let mut max_value = score[left][right];
                let left_best = best[left][right - 1];
                if left_best > max_value {
                    max_value = left_best;
                }
                let right_best = best[left + 1][right];
                if right_best > max_value {
                    max_value = right_best;
                }
                best[left][right] = max_value;
            }
        }

        queries
            .into_iter()
            .map(|query| {
                let left = query[0] as usize;
                let right = query[1] as usize;
                best[left][right]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 8, 4, 32, 16, 1];
        let queries = vec![vec![0, 2], vec![1, 4], vec![0, 5]];
        assert_eq!(
            Solution::maximum_subarray_xor(nums, queries),
            vec![12, 60, 60]
        );
    }

    #[test]
    fn example_two() {
        let nums = vec![0, 7, 3, 2, 8, 5, 1];
        let queries = vec![vec![0, 3], vec![1, 5], vec![2, 4], vec![2, 6], vec![5, 6]];
        assert_eq!(
            Solution::maximum_subarray_xor(nums, queries),
            vec![7, 14, 11, 14, 5]
        );
    }

    #[test]
    fn single_element() {
        let nums = vec![5];
        let queries = vec![vec![0, 0]];
        assert_eq!(Solution::maximum_subarray_xor(nums, queries), vec![5]);
    }

    #[test]
    fn all_same_values() {
        let nums = vec![1, 1, 1];
        let queries = vec![vec![0, 2], vec![1, 2]];
        assert_eq!(Solution::maximum_subarray_xor(nums, queries), vec![1, 1]);
    }
}
