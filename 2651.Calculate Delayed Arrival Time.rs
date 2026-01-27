impl Solution {
    /// Calculate delayed arrival time on a 24-hour clock.
    ///
    /// # Intuition
    /// Simple modular addition wraps around at 24 hours.
    ///
    /// # Approach
    /// Return (arrival_time + delayed_time) % 24.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_wrap() {
        assert_eq!(Solution::find_delayed_arrival_time(15, 5), 20);
    }

    #[test]
    fn test_wrap_around() {
        assert_eq!(Solution::find_delayed_arrival_time(23, 5), 4);
    }

    #[test]
    fn test_zero_delay() {
        assert_eq!(Solution::find_delayed_arrival_time(10, 0), 10);
    }

    #[test]
    fn test_exact_24() {
        assert_eq!(Solution::find_delayed_arrival_time(0, 24), 0);
    }
}
