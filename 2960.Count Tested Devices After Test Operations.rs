impl Solution {
    /// Counts the number of devices that pass the battery test.
    ///
    /// # Intuition
    /// Each tested device decrements all subsequent batteries by 1. Instead of
    /// actually decrementing, track how many devices have been tested so far;
    /// a device passes if its original percentage exceeds this count.
    ///
    /// # Approach
    /// 1. Fold over the battery percentages, accumulating the tested count.
    /// 2. A device passes when its percentage exceeds the running count.
    ///
    /// # Complexity
    /// - Time: O(n) single pass
    /// - Space: O(1)
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        battery_percentages
            .iter()
            .fold(0, |ans, &x| ans + if x > ans { 1 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_devices() {
        assert_eq!(Solution::count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
    }

    #[test]
    fn test_all_zero() {
        assert_eq!(Solution::count_tested_devices(vec![0, 0, 0]), 0);
    }
}
