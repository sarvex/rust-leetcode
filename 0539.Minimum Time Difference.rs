pub struct Solution;

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
        let mut minutes: Vec<i32> = Vec::with_capacity(time_points.len());
        for t in &time_points {
            let b = t.as_bytes();
            let hour = ((b[0] - b'0') as i32 * 10 + (b[1] - b'0') as i32) * 60;
            let minute = (b[3] - b'0') as i32 * 10 + (b[4] - b'0') as i32;
            minutes.push(hour + minute);
        }
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

    #[test]
    fn test_same_time() {
        assert_eq!(
            Solution::find_min_difference(vec!["12:00".to_string(), "12:00".to_string()]),
            0
        );
    }

    #[test]
    fn test_noon_and_midnight() {
        // 12:00 and 00:00 -> difference is 720 minutes (12 hours)
        assert_eq!(
            Solution::find_min_difference(vec!["12:00".to_string(), "00:00".to_string()]),
            720
        );
    }

    #[test]
    fn test_adjacent_minutes() {
        assert_eq!(
            Solution::find_min_difference(vec!["05:30".to_string(), "05:31".to_string()]),
            1
        );
    }

    #[test]
    fn test_multiple_times() {
        // Times: 01:00 (60), 02:00 (120), 03:00 (180)
        // Min diff = 60
        assert_eq!(
            Solution::find_min_difference(vec![
                "01:00".to_string(),
                "02:00".to_string(),
                "03:00".to_string()
            ]),
            60
        );
    }

    #[test]
    fn test_wrap_around() {
        // 23:00 (1380) and 01:00 (60)
        // Direct diff: 1320, wrap around: 1440 - 1380 + 60 = 120
        assert_eq!(
            Solution::find_min_difference(vec!["23:00".to_string(), "01:00".to_string()]),
            120
        );
    }
}
