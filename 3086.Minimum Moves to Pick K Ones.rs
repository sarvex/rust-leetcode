fn min_cost_for_t(positions: &[i64], prefix: &[i64], t: usize) -> i64 {
    let mut best = i64::MAX;
    for start in 0..=positions.len() - t {
        let mid = start + t / 2;
        let median = positions[mid];
        let left_sum = prefix[mid] - prefix[start];
        let right_sum = prefix[start + t] - prefix[mid + 1];
        let left_count = (mid - start) as i64;
        let right_count = (start + t - mid - 1) as i64;
        let cost_left = median * left_count - left_sum;
        let cost_right = right_sum - median * right_count;
        best = best.min(cost_left + cost_right);
    }
    best
}

impl Solution {
    /// Use median-based swaps for existing ones, and 2-move changes for the rest.
    ///
    /// # Intuition
    /// Moving a one to `aliceIndex` via swaps costs its distance. Creating a one and then
    /// swapping it into `aliceIndex` always costs 2 moves, so the total cost is the sum of
    /// distances for picked existing ones plus 2 per created one.
    ///
    /// # Approach
    /// - Record indices of existing ones.
    /// - Let `need = k - max_changes`; we must pick at least `need` existing ones.
    /// - For any `t` existing ones, the minimum swap cost is achieved by choosing the median
    ///   of a contiguous block of `t` ones. Use prefix sums to evaluate every window in O(m).
    /// - Extra existing ones beyond `need` are only worth picking if their distance is <= 2,
    ///   because a change costs 2. At most five positions are within distance 2 of any index,
    ///   so only `t in [need, need + 5]` can be optimal.
    /// - Combine each candidate `t` with `2 * (k - t)` change cost and take the minimum.
    ///
    /// # Complexity
    /// - Time: O(m * c), where m is the number of ones and c <= 6
    /// - Space: O(m)
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let k = k as usize;
        let max_changes = max_changes as usize;
        let positions: Vec<i64> = nums
            .iter()
            .enumerate()
            .filter_map(|(idx, &value)| if value == 1 { Some(idx as i64) } else { None })
            .collect();
        let ones_count = positions.len();

        if ones_count == 0 {
            return 2 * k as i64;
        }

        let need = k.saturating_sub(max_changes);
        let max_t = k.min(ones_count);
        let upper_t = max_t.min(need + 5);

        let mut prefix = vec![0i64; ones_count + 1];
        for (idx, &pos) in positions.iter().enumerate() {
            prefix[idx + 1] = prefix[idx] + pos;
        }

        let mut answer = i64::MAX;
        if need == 0 {
            answer = 2 * k as i64;
        }

        if upper_t >= 1 {
            for t in need.max(1)..=upper_t {
                let best_existing = min_cost_for_t(&positions, &prefix, t);
                let total = best_existing + 2 * (k - t) as i64;
                answer = answer.min(total);
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1];
        assert_eq!(Solution::minimum_moves(nums, 3, 1), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(Solution::minimum_moves(nums, 2, 3), 4);
    }

    #[test]
    fn test_all_ones_use_swaps() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::minimum_moves(nums, 3, 3), 2);
    }

    #[test]
    fn test_no_changes() {
        let nums = vec![1, 0, 0, 0, 1];
        assert_eq!(Solution::minimum_moves(nums, 2, 0), 4);
    }

    #[test]
    fn test_single_pick_free() {
        let nums = vec![0, 1, 0];
        assert_eq!(Solution::minimum_moves(nums, 1, 0), 0);
    }
}
