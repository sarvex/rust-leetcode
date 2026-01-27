impl Solution {
    /// Determines if two time-range events overlap.
    ///
    /// # Intuition
    /// Two intervals [s1, e1] and [s2, e2] conflict unless one ends before the
    /// other starts. String comparison works since times are in "HH:MM" format.
    ///
    /// # Approach
    /// 1. Check if event1 ends before event2 starts or vice versa
    /// 2. Negate to detect overlap
    ///
    /// # Complexity
    /// - Time: O(1) — fixed-length string comparisons
    /// - Space: O(1)
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        !(event1[1] < event2[0] || event1[0] > event2[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn test_overlap() {
        assert!(Solution::have_conflict(
            vec![s("01:15"), s("02:00")],
            vec![s("02:00"), s("03:00")]
        ));
    }

    #[test]
    fn test_contained() {
        assert!(Solution::have_conflict(
            vec![s("01:00"), s("02:00")],
            vec![s("01:20"), s("03:00")]
        ));
    }

    #[test]
    fn test_no_overlap() {
        assert!(!Solution::have_conflict(
            vec![s("10:00"), s("11:00")],
            vec![s("14:00"), s("15:00")]
        ));
    }

    #[test]
    fn test_same_event() {
        assert!(Solution::have_conflict(
            vec![s("09:00"), s("10:00")],
            vec![s("09:00"), s("10:00")]
        ));
    }
}
