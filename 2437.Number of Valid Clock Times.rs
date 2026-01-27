impl Solution {
    /// Counts valid clock times matching a pattern with '?' wildcards.
    ///
    /// # Intuition
    /// Enumerate all 24*60 possible times and check each against the pattern.
    /// Any non-'?' character must match exactly.
    ///
    /// # Approach
    /// 1. Iterate hours 0..24 and minutes 0..60
    /// 2. Format each candidate as "HH:MM"
    /// 3. Compare byte-by-byte, treating '?' as always matching
    ///
    /// # Complexity
    /// - Time: O(1) — at most 1440 candidates, 5 chars each
    /// - Space: O(1)
    pub fn count_time(time: String) -> i32 {
        let pattern = time.as_bytes();
        (0..24)
            .flat_map(|h| (0..60).map(move |m| (h, m)))
            .filter(|&(h, m)| {
                let candidate = format!("{:02}:{:02}", h, m);
                candidate
                    .as_bytes()
                    .iter()
                    .zip(pattern.iter())
                    .all(|(&c, &p)| p == b'?' || p == c)
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
    }

    #[test]
    fn test_all_wildcards() {
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }

    #[test]
    fn test_no_wildcards() {
        assert_eq!(Solution::count_time("12:34".to_string()), 1);
    }

    #[test]
    fn test_hour_wildcard() {
        assert_eq!(Solution::count_time("?4:00".to_string()), 3);
    }
}
