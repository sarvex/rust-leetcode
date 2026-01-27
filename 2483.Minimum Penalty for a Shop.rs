impl Solution {
    /// Finds the earliest closing hour that minimizes shop penalty.
    ///
    /// # Intuition
    /// Penalty at hour i = (customers arriving after i that are missed) +
    /// (hours open before i with no customers). We can track a running penalty
    /// starting from the total 'Y' count and greedily find the minimum.
    ///
    /// # Approach
    /// 1. Start with penalty = total 'Y' count (closing at hour 0 misses all)
    /// 2. Scan left to right: 'Y' decreases penalty (served), 'N' increases it (wasted)
    /// 3. Track the minimum penalty and its earliest index
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn best_closing_time(customers: String) -> i32 {
        let bytes = customers.as_bytes();
        let total_y = bytes.iter().filter(|&&b| b == b'Y').count() as i32;

        let (best_hour, _, _) = bytes.iter().enumerate().fold(
            (0, total_y, total_y),
            |(best_hour, best_penalty, penalty), (i, &b)| {
                let new_penalty = if b == b'Y' { penalty - 1 } else { penalty + 1 };
                if new_penalty < best_penalty {
                    (i as i32 + 1, new_penalty, new_penalty)
                } else {
                    (best_hour, best_penalty, new_penalty)
                }
            },
        );

        best_hour
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_customers() {
        assert_eq!(Solution::best_closing_time("YYNY".to_string()), 2);
    }

    #[test]
    fn test_all_no() {
        assert_eq!(Solution::best_closing_time("NNNNN".to_string()), 0);
    }

    #[test]
    fn test_all_yes() {
        assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
    }

    #[test]
    fn test_single_yes() {
        assert_eq!(Solution::best_closing_time("Y".to_string()), 1);
    }

    #[test]
    fn test_single_no() {
        assert_eq!(Solution::best_closing_time("N".to_string()), 0);
    }
}
