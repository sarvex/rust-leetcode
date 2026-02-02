impl Solution {
    /// Maximize consecutive selected elements via frequency DP.
    ///
    /// # Intuition
    /// Each number can stay at `x` or become `x + 1`. A valid selection is a
    /// strictly consecutive sequence, so we can use at most one element per
    /// value. This becomes a longest chain where value `v` can be produced by
    /// any element `v` or `v - 1`.
    ///
    /// # Approach
    /// Count frequencies to avoid sorting. Maintain `best[v]` as the maximum
    /// length of a consecutive sequence ending at value `v`.
    /// For each value `v` with frequency `c`:
    /// - One element can extend `best[v - 1]` to `best[v]`.
    /// - One element can extend a sequence ending at `v` to `v + 1`.
    /// If `c == 1`, the `v + 1` update must use the previous `best[v]`
    /// snapshot; if `c >= 2`, another element can use the updated `best[v]`.
    ///
    /// # Complexity
    /// - Time: O(n + max(nums))
    /// - Space: O(max(nums))
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let max_value = nums.iter().copied().max().unwrap_or(0) as usize;
        let min_value = nums.iter().copied().min().unwrap_or(0) as usize;
        let mut counts = vec![0usize; max_value + 1];
        for value in nums {
            counts[value as usize] += 1;
        }

        let mut best = vec![0usize; max_value + 2];
        let mut answer = 0usize;

        for value in min_value..=max_value {
            let count = counts[value];
            if count == 0 {
                continue;
            }

            let prev_best_value = best[value];
            let prev_best_left = if value > 0 { best[value - 1] } else { 0 };

            let updated_value = prev_best_value.max(prev_best_left + 1);
            best[value] = updated_value;

            let mut next_best = best[value + 1].max(prev_best_value + 1);
            if count > 1 {
                next_best = next_best.max(updated_value + 1);
            }
            best[value + 1] = next_best;

            answer = answer.max(updated_value).max(next_best);
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 1, 5, 1, 1];
        assert_eq!(Solution::max_selected_elements(nums), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 4, 7, 10];
        assert_eq!(Solution::max_selected_elements(nums), 1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![10];
        assert_eq!(Solution::max_selected_elements(nums), 1);
    }

    #[test]
    fn test_all_same_values() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(Solution::max_selected_elements(nums), 2);
    }

    #[test]
    fn test_already_consecutive() {
        let nums = vec![3, 4, 5];
        assert_eq!(Solution::max_selected_elements(nums), 3);
    }

    #[test]
    fn test_gap_filled_by_increment() {
        let nums = vec![1, 2, 2, 3];
        assert_eq!(Solution::max_selected_elements(nums), 4);
    }
}
