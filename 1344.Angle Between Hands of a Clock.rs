impl Solution {
    /// Calculate the smaller angle between clock hands using direct formula.
    ///
    /// # Intuition
    /// The minute hand moves 6° per minute (360°/60). The hour hand moves 0.5°
    /// per minute (360°/720), so its position depends on both hour and minutes.
    ///
    /// # Approach
    /// 1. Compute minute hand angle: minutes × 6°
    /// 2. Compute hour hand angle: (hour % 12) × 30° + minutes × 0.5°
    /// 3. Take the absolute difference, then return the smaller of that or 360° minus it
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let minute_angle = minutes as f64 * 6.0;
        let hour_angle = (hour % 12) as f64 * 30.0 + minutes as f64 * 0.5;
        let diff = (minute_angle - hour_angle).abs();
        diff.min(360.0 - diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!((Solution::angle_clock(12, 30) - 165.0).abs() < 1e-5);
    }

    #[test]
    fn test_example_2() {
        assert!((Solution::angle_clock(3, 30) - 75.0).abs() < 1e-5);
    }

    #[test]
    fn test_example_3() {
        assert!((Solution::angle_clock(3, 15) - 7.5).abs() < 1e-5);
    }

    #[test]
    fn test_midnight() {
        assert!((Solution::angle_clock(12, 0) - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_edge_hour_12() {
        // 12:00 — both hands at 0°
        assert!((Solution::angle_clock(12, 0) - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_six_oclock() {
        // 6:00 — minute at 0°, hour at 180°
        assert!((Solution::angle_clock(6, 0) - 180.0).abs() < 1e-5);
    }

    #[test]
    fn test_non_integer_result() {
        // 1:57 — minute at 342°, hour at 58.5° → diff 283.5° → smaller 76.5°
        assert!((Solution::angle_clock(1, 57) - 76.5).abs() < 1e-5);
    }
}
