impl Solution {
    /// Finds the minimum time difference between any two time points.
    ///
    /// # Intuition
    /// Convert times to minutes, sort, and check consecutive differences
    /// including the wrap-around from the last to the first.
    ///
    /// # Approach
    /// 1. If there are more than 1440 points, return 0 (pigeonhole).
    /// 2. Convert each "HH:MM" to minutes.
    /// 3. Sort and find the minimum gap, including the circular gap.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() > 1440 {
            return 0;
        }
        let mut minutes: Vec<i32> = time_points
            .iter()
            .map(|t| {
                let b = t.as_bytes();
                ((b[0] - b'0') as i32 * 10 + (b[1] - b'0') as i32) * 60
                    + (b[3] - b'0') as i32 * 10
                    + (b[4] - b'0') as i32
            })
            .collect();
        minutes.sort_unstable();
        let wrap = 1440 - minutes.last().unwrap() + minutes[0];
        minutes
            .windows(2)
            .map(|w| w[1] - w[0])
            .min()
            .unwrap()
            .min(wrap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
            1
        );
    }

    #[test]
    fn test_three_points() {
        assert_eq!(
            Solution::find_min_difference(vec![
                "00:00".to_string(),
                "23:59".to_string(),
                "00:00".to_string(),
            ]),
            0
        );
    }
}
