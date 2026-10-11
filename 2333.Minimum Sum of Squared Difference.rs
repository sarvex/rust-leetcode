#[cfg(test)]
struct Solution;

impl Solution {
    /// Shave the tallest absolute gaps with the shared `k1 + k2` budget.
    ///
    /// # Intuition
    /// A ±1 change on either array shrinks one absolute gap by 1, so the two
    /// budgets are interchangeable. Dropping a gap from `d` to `d - 1` removes
    /// `2d - 1` from the square sum, and that saving is larger for a larger `d`.
    /// Always spend the next change on a current maximum. A gap of 0 cannot be
    /// improved, so leftover budget is unused.
    ///
    /// # Approach
    /// Count how many indices share each absolute difference. Differences are at
    /// most `10^5`. Walk that table from the top. Every gap at height `h` is
    /// moved to `h - 1` while budget remains; a shortfall moves only part of
    /// the group and stops. Items that arrive at `h - 1` are handled on the
    /// next step, which is the same order as always reducing a maximum.
    /// The answer is the sum of squares of the heights that remain, accumulated
    /// in `i64` (the total can reach `10^15`).
    ///
    /// # Complexity
    /// - Time: O(n + D), where D ≤ 10^5 is the largest absolute difference.
    /// - Space: O(D) for the frequency table.
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let max_gap = nums1
            .iter()
            .zip(&nums2)
            .map(|(num1, num2)| num1.abs_diff(*num2) as usize)
            .max()
            .unwrap_or_default();
        let mut gap_counts = vec![0_i64; max_gap + 1];
        for (num1, num2) in nums1.iter().zip(&nums2) {
            gap_counts[num1.abs_diff(*num2) as usize] += 1;
        }

        let mut budget = i64::from(k1) + i64::from(k2);
        for height in (1..=max_gap).rev() {
            if budget == 0 {
                break;
            }
            let moved = budget.min(gap_counts[height]);
            gap_counts[height] -= moved;
            gap_counts[height - 1] += moved;
            budget -= moved;
        }

        gap_counts
            .into_iter()
            .enumerate()
            .map(|(gap, count)| {
                let gap = gap as i64;
                gap * gap * count
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;

    fn brute_force(nums1: &[i32], nums2: &[i32], mut budget: i64) -> i64 {
        let mut gaps: BinaryHeap<i32> = nums1
            .iter()
            .zip(nums2)
            .map(|(num1, num2)| num1.abs_diff(*num2) as i32)
            .collect();
        while budget > 0 {
            match gaps.pop() {
                Some(gap) if gap > 0 => {
                    gaps.push(gap - 1);
                    budget -= 1;
                }
                _ => break,
            }
        }
        gaps.into_iter()
            .map(|gap| i64::from(gap) * i64::from(gap))
            .sum()
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            579
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            43
        );
    }

    #[test]
    fn test_already_equal() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![4, 7, 9], vec![4, 7, 9], 5, 8),
            0
        );
    }

    #[test]
    fn test_single_gap() {
        assert_eq!(Solution::min_sum_square_diff(vec![0], vec![10], 0, 0), 100);
        assert_eq!(Solution::min_sum_square_diff(vec![0], vec![10], 3, 3), 16);
        assert_eq!(Solution::min_sum_square_diff(vec![0], vec![10], 10, 0), 0);
        assert_eq!(Solution::min_sum_square_diff(vec![0], vec![10], 4, 100), 0);
    }

    #[test]
    fn test_budget_larger_than_every_gap() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 100, 100),
            0
        );
    }

    #[test]
    fn test_only_the_largest_gaps_move() {
        // Gaps 5, 5, 3 and a budget of 3 become 4, 3, 3.
        assert_eq!(
            Solution::min_sum_square_diff(vec![5, 5, 5], vec![0, 0, 2], 3, 0),
            34
        );
    }

    #[test]
    fn test_budgets_are_interchangeable() {
        let nums1 = vec![1, 4, 10, 12];
        let nums2 = vec![5, 8, 6, 9];
        assert_eq!(
            Solution::min_sum_square_diff(nums1.clone(), nums2.clone(), 2, 0),
            43
        );
        assert_eq!(Solution::min_sum_square_diff(nums1, nums2, 0, 2), 43);
    }

    #[test]
    fn test_partial_top_level() {
        // Five gaps of 10 and a budget of 7: three stay at 9, two fall to 8.
        assert_eq!(
            Solution::min_sum_square_diff(vec![10; 5], vec![0; 5], 7, 0),
            3 * 81 + 2 * 64
        );
    }

    #[test]
    fn test_multi_level_flatten() {
        // Gaps 17, 15, 8, 1 with budget 20 settle at 7, 7, 6, 1.
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 10, 10),
            135
        );
    }

    #[test]
    fn test_maximum_value_no_budget() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![0], vec![100_000], 0, 0),
            10_000_000_000
        );
    }

    #[test]
    fn test_one_change_on_maximum_value() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![0], vec![100_000], 1, 0),
            9_999_800_001
        );
    }

    #[test]
    fn test_wide_input_exact_level_drop() {
        // 100_000 gaps of 100_000 and a budget of 2e9: each gap falls by 20_000.
        let nums1 = vec![0; 100_000];
        let nums2 = vec![100_000; 100_000];
        assert_eq!(
            Solution::min_sum_square_diff(nums1, nums2, 1_000_000_000, 1_000_000_000),
            100_000_i64 * 80_000 * 80_000
        );
    }

    #[test]
    fn test_matches_brute_force_on_small_inputs() {
        let mut state = 0x1234_5678_u32;
        let mut next = || {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            state
        };
        for length in 1..=12 {
            for _ in 0..40 {
                let nums1: Vec<i32> = (0..length).map(|_| (next() % 40) as i32).collect();
                let nums2: Vec<i32> = (0..length).map(|_| (next() % 40) as i32).collect();
                let k1 = (next() % 80) as i32;
                let k2 = (next() % 80) as i32;
                let expected = brute_force(&nums1, &nums2, i64::from(k1) + i64::from(k2));
                assert_eq!(
                    Solution::min_sum_square_diff(nums1, nums2, k1, k2),
                    expected
                );
            }
        }
    }

    #[test]
    fn test_hand_cases_match_brute_force() {
        let cases = [
            (vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            (vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            (vec![1, 4, 10, 12], vec![5, 8, 6, 9], 8, 0),
            (vec![7, 4], vec![0, 0], 1, 3),
            (vec![0, 0], vec![100, 1], 0, 40),
            (vec![9], vec![0], 0, 0),
            (vec![3, 3, 3, 3], vec![0, 1, 2, 8], 5, 5),
        ];
        for (nums1, nums2, k1, k2) in cases {
            let expected = brute_force(&nums1, &nums2, i64::from(k1) + i64::from(k2));
            assert_eq!(
                Solution::min_sum_square_diff(nums1, nums2, k1, k2),
                expected
            );
        }
    }
}
