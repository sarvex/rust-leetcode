impl Solution {
    /// Linear DP with array rotation for maximizing cyclic partition score.
    ///
    /// # Intuition
    /// Rotate the array starting from the maximum element to handle cyclic nature.
    /// Solve both forward and backward rotations, taking the best result.
    ///
    /// # Approach
    /// 1. Find max element index and create forward/backward rotations
    /// 2. For each rotation, use linear DP tracking (max - min) ranges per segment
    /// 3. Track best score using first j elements, incrementally add segments
    ///
    /// # Complexity
    /// - Time: O(n * k)
    /// - Space: O(n)
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        let length = nums.len();
        let partition_count = (k as usize).min(length);

        let max_element_index = (0..length).max_by_key(|&index| nums[index]).unwrap_or(0);

        let forward_rotation: Vec<i64> = (0..length)
            .map(|index| nums[(max_element_index + index) % length] as i64)
            .collect();

        let backward_rotation: Vec<i64> = (0..length)
            .map(|index| nums[(max_element_index + 1 + index) % length] as i64)
            .rev()
            .collect();

        fn solve_linear(arr: &[i64], partition_count: usize) -> i64 {
            const NEG_INFINITY: i64 = i64::MIN / 4;
            let length = arr.len();

            let mut current_dp = vec![NEG_INFINITY; length + 1];
            let mut next_dp = vec![NEG_INFINITY; length + 1];

            let mut running_min = i64::MAX;
            let mut running_max = i64::MIN;
            for index in 0..length {
                let value = arr[index];
                running_min = running_min.min(value);
                running_max = running_max.max(value);
                current_dp[index + 1] = running_max - running_min;
            }

            let mut result = current_dp[length];

            for _ in 1..partition_count {
                next_dp.fill(NEG_INFINITY);

                let mut best_minus_value = NEG_INFINITY;
                let mut best_plus_value = NEG_INFINITY;
                let mut best_score = NEG_INFINITY;

                for position in _segment..length {
                    let current_value = arr[position];
                    let previous_score = current_dp[position];

                    best_minus_value = best_minus_value.max(previous_score - current_value);
                    best_plus_value = best_plus_value.max(previous_score + current_value);

                    let candidate_score = (best_minus_value + current_value)
                        .max(best_plus_value - current_value)
                        .max(best_score);

                    next_dp[position + 1] = candidate_score;
                    best_score = candidate_score;
                }

                std::mem::swap(&mut current_dp, &mut next_dp);
                result = result.max(current_dp[length]);
            }

            result
        }

        let forward_score = solve_linear(&forward_rotation, partition_count);
        let backward_score = solve_linear(&backward_rotation, partition_count);
        forward_score.max(backward_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_partitions() {
        assert_eq!(Solution::maximum_score(vec![1, 2, 3, 3], 2), 3);
    }

    #[test]
    fn test_single_partition() {
        assert_eq!(Solution::maximum_score(vec![1, 2, 3, 3], 1), 2);
    }

    #[test]
    fn test_max_partitions() {
        assert_eq!(Solution::maximum_score(vec![1, 2, 3, 3], 4), 3);
    }

    #[test]
    fn test_large_value_difference() {
        assert_eq!(
            Solution::maximum_score(vec![1, 1000000000, 1000000000], 3),
            999999999
        );
    }

    #[test]
    fn test_eight_elements_four_partitions() {
        assert_eq!(
            Solution::maximum_score(
                vec![
                    563861991, 113020722, 111783372, 603350366, 771729177, 91633423, 664375708,
                    85118934
                ],
                4
            ),
            2201760791
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_score(vec![5], 1), 0);
    }

    #[test]
    fn test_two_elements_one_partition() {
        assert_eq!(Solution::maximum_score(vec![1, 10], 1), 9);
    }

    #[test]
    fn test_two_elements_two_partitions() {
        assert_eq!(Solution::maximum_score(vec![1, 10], 2), 9);
    }

    #[test]
    fn test_uniform_array() {
        assert_eq!(Solution::maximum_score(vec![5, 5, 5, 5], 2), 0);
    }

    #[test]
    fn test_increasing_array() {
        let result = Solution::maximum_score(vec![1, 2, 3, 4, 5], 2);
        assert!(result >= 4);
    }

    #[test]
    fn test_large_array_many_partitions() {
        let nums = vec![
            162, 79, 100, 157, 111, 134, 17, 108, 121, 191, 78, 64, 5, 107, 76, 156, 115, 147, 152,
            124, 99, 176, 85, 187, 90, 155, 33, 89, 79, 148, 118, 37, 133, 53, 79, 22, 118, 36, 65,
            30, 43, 93, 132, 181, 123, 149, 11, 142, 154, 48, 174, 184, 108, 163, 52, 69, 3, 162,
            152, 90, 1, 170, 164, 82, 85, 26, 165, 50, 24, 140, 98, 11, 100, 183, 148, 74, 63, 33,
            143, 26, 89, 143, 54, 177, 49, 168, 193, 191, 134, 106, 166, 42, 112, 193, 163, 5, 71,
            65, 22, 1, 116, 76, 71, 200, 94, 11, 108, 196, 9, 159, 11, 51, 156, 1, 38, 23, 52, 131,
            165, 102, 105, 179, 81, 28, 26, 165, 62, 81, 22, 161, 46, 46, 137, 61, 181, 125, 135,
            157, 155, 24,
        ];
        assert_eq!(Solution::maximum_score(nums, 95), 5848);
    }

    #[test]
    fn test_large_array_108_partitions() {
        let nums = vec![
            135, 195, 110, 67, 87, 168, 145, 5, 105, 9, 74, 152, 117, 177, 107, 184, 61, 34, 30,
            192, 63, 193, 102, 34, 67, 145, 136, 180, 122, 190, 49, 69, 80, 54, 58, 129, 93, 167,
            200, 61, 12, 188, 158, 78, 80, 154, 126, 138, 76, 141, 102, 108, 91, 32, 1, 190, 33,
            89, 25, 171, 24, 56, 69, 186, 111, 153, 126, 10, 164, 105, 4, 188, 96, 24, 118, 179,
            73, 50, 86, 119, 102, 127, 4, 66, 94, 32, 123, 84, 155, 75, 193, 159, 52, 147, 57, 155,
            17, 49, 87, 67, 3, 57, 122, 106, 112, 87, 134, 188, 35, 127, 195, 36, 20, 164, 133,
            178, 149, 65, 177, 51, 189, 90, 84, 73, 22, 106, 137, 23, 129, 165, 105, 60, 65, 42,
            35, 61, 48, 25, 60, 73, 66, 10, 144, 26, 105, 73, 158, 158, 84, 200, 27, 17, 102, 186,
            20, 138, 43, 170, 6, 171, 124, 158, 57, 48, 110, 109, 74, 153, 100, 112, 125, 22, 36,
            73, 18, 30, 116, 113,
        ];
        let result = Solution::maximum_score(nums, 108);
        assert!(result > 0);
    }
}
