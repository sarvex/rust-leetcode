impl Solution {
    /// Finds good days to rob the bank using prefix sums on direction arrays.
    ///
    /// # Intuition
    /// A day is "good" if the `time` days before it are non-increasing and the
    /// `time` days after it are non-decreasing. By encoding each adjacent pair
    /// as a direction (+1, -1, 0) and building prefix sums, we can check any
    /// window in O(1).
    ///
    /// # Approach
    /// 1. Build a direction array where each element indicates whether security
    ///    increased, decreased, or stayed the same relative to the previous day.
    /// 2. Compute prefix sums counting increases (`up`) and decreases (`down`).
    /// 3. For each candidate day, verify that the preceding window has no
    ///    increases and the following window has no decreases.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let time = time as usize;
        let n = security.len();
        if time * 2 >= n {
            return vec![];
        }

        let direction: Vec<i8> = std::iter::once(0)
            .chain(security.windows(2).map(|w| match w[1].cmp(&w[0]) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }))
            .collect();

        let mut up = vec![0i32; n + 1];
        let mut down = vec![0i32; n + 1];
        for i in 1..=n {
            up[i] = up[i - 1] + i32::from(direction[i - 1] == 1);
            down[i] = down[i - 1] + i32::from(direction[i - 1] == -1);
        }

        (time..n - time)
            .filter(|&i| up[i + 1] - up[i + 1 - time] == 0 && down[i + 1 + time] - down[i + 1] == 0)
            .map(|i| i as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let result = Solution::good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_single_good_day() {
        let result = Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_time_too_large() {
        let result = Solution::good_days_to_rob_bank(vec![1, 2, 3], 2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_all_equal() {
        let result = Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 2);
        assert_eq!(result, vec![2]);
    }
}
