impl Solution {
    /// Count passengers older than 60 from encoded detail strings.
    ///
    /// # Intuition
    /// The age is encoded at string positions 11..13. Parse those two characters
    /// and compare against 60.
    ///
    /// # Approach
    /// 1. For each detail string, extract the age substring at indices 11..13
    /// 2. Parse as integer and filter those > 60
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|s| {
                s.as_bytes()
                    .get(11..13)
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .and_then(|age_str| age_str.parse::<i32>().ok())
                    .is_some_and(|age| age > 60)
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn test_mixed_ages() {
        assert_eq!(
            Solution::count_seniors(vec![
                s("7868190130M7522"),
                s("5303914400F9211"),
                s("9273338290F4010"),
            ]),
            2
        );
    }

    #[test]
    fn test_no_seniors() {
        assert_eq!(
            Solution::count_seniors(vec![s("1313579440F2036"), s("2## 921522980M5644")]),
            0
        );
    }
}
