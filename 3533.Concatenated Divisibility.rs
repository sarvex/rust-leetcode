impl Solution {
    /// Bitmask DP with memoization to find lexicographically smallest divisible permutation.
    ///
    /// # Intuition
    /// We need to find the lexicographically smallest permutation where concatenating all numbers
    /// is divisible by k. Sorting indices and trying them in order ensures lexicographic ordering.
    /// Use bitmask DP to track which elements are used and the current remainder modulo k.
    ///
    /// # Approach
    /// 1. Sort indices by corresponding values to ensure lexicographic ordering
    /// 2. Precompute multipliers (10^digits mod k) for each number
    /// 3. Use flat memoization array with (mask * k + rem) indexing for cache efficiency
    /// 4. DFS with closure captures context, reducing parameter passing overhead
    ///
    /// # Complexity
    /// - Time: O(2^n * k * n)
    /// - Space: O(2^n * k) for memoization table
    pub fn concatenated_divisibility(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let full_mask = (1 << n) - 1;

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_unstable_by_key(|&i| nums[i]);

        let mults: Vec<usize> = nums
            .iter()
            .map(|&x| {
                let digits = if x >= 10000 {
                    5
                } else if x >= 1000 {
                    4
                } else if x >= 100 {
                    3
                } else if x >= 10 {
                    2
                } else {
                    1
                };
                (0..digits).fold(1, |acc, _| (acc * 10) % k)
            })
            .collect();

        let nums_mod: Vec<usize> = nums.iter().map(|&x| x as usize % k).collect();
        let mut memo = vec![false; (1 << n) * k];
        let mut result = vec![0i32; n];

        let found = {
            let indices = &indices;
            let mults = &mults;
            let nums_mod = &nums_mod;
            let nums = &nums;

            fn dfs(
                pos: usize,
                mask: usize,
                rem: usize,
                n: usize,
                k: usize,
                full_mask: usize,
                indices: &[usize],
                mults: &[usize],
                nums_mod: &[usize],
                nums: &[i32],
                memo: &mut [bool],
                result: &mut [i32],
            ) -> bool {
                if mask == full_mask {
                    return rem == 0;
                }

                let memo_idx = mask * k + rem;
                if memo[memo_idx] {
                    return false;
                }

                for &i in indices {
                    if mask & (1 << i) != 0 {
                        continue;
                    }
                    let new_rem = (rem * mults[i] + nums_mod[i]) % k;
                    result[pos] = nums[i];
                    if dfs(
                        pos + 1,
                        mask | (1 << i),
                        new_rem,
                        n,
                        k,
                        full_mask,
                        indices,
                        mults,
                        nums_mod,
                        nums,
                        memo,
                        result,
                    ) {
                        return true;
                    }
                }

                memo[memo_idx] = true;
                false
            }

            dfs(
                0,
                0,
                0,
                n,
                k,
                full_mask,
                indices,
                mults,
                nums_mod,
                nums,
                &mut memo,
                &mut result,
            )
        };

        if found {
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_basic_divisibility() {
        // Arrange
        let nums = vec![3, 12, 45];
        let k = 5;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert
        assert_eq!(result, vec![3, 12, 45]);
    }

    #[test]
    fn test_example_2_reorder_required() {
        // Arrange
        let nums = vec![10, 5];
        let k = 10;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert
        assert_eq!(result, vec![5, 10]);
    }

    #[test]
    fn test_example_3_no_valid_permutation() {
        // Arrange
        let nums = vec![1, 2, 3];
        let k = 5;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_single_element_divisible() {
        // Arrange
        let nums = vec![10];
        let k = 5;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_single_element_not_divisible() {
        // Arrange
        let nums = vec![7];
        let k = 3;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_all_same_elements() {
        // Arrange
        let nums = vec![5, 5, 5];
        let k = 5;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert
        assert_eq!(result, vec![5, 5, 5]);
    }

    #[test]
    fn test_large_k_value() {
        // Arrange
        let nums = vec![50, 50];
        let k = 100;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert - 5050 % 100 == 50, so no valid permutation
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_lexicographically_smallest() {
        // Arrange
        let nums = vec![2, 1, 3];
        let k = 1;

        // Act
        let result = Solution::concatenated_divisibility(nums, k);

        // Assert - any permutation works, lexicographically smallest is [1, 2, 3]
        assert_eq!(result, vec![1, 2, 3]);
    }
}
