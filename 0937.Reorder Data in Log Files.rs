use std::cmp::Ordering;

impl Solution {
    /// Reorders logs with letter-logs before digit-logs per sorting rules.
    ///
    /// # Intuition
    /// Letter-logs are sorted by content then identifier. Digit-logs maintain
    /// their original order. Letter-logs always precede digit-logs.
    ///
    /// # Approach
    /// Custom sort: classify each log as letter or digit by the first character
    /// after the identifier. Sort letter-logs lexicographically by content,
    /// then by identifier on tie. Digit-logs sort after letter-logs.
    ///
    /// # Complexity
    /// - Time: O(n * L * log n) where L is max log length
    /// - Space: O(L) for comparisons
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by(|a, b| {
            let (id_a, body_a) = a.split_once(' ').unwrap();
            let (id_b, body_b) = b.split_once(' ').unwrap();
            let is_letter_a = body_a.as_bytes()[0].is_ascii_alphabetic();
            let is_letter_b = body_b.as_bytes()[0].is_ascii_alphabetic();

            match (is_letter_a, is_letter_b) {
                (true, true) => body_a.cmp(body_b).then_with(|| id_a.cmp(id_b)),
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (false, false) => Ordering::Equal,
            }
        });
        logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let logs = vec![
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let expected: Vec<String> = vec![
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        assert_eq!(Solution::reorder_log_files(logs), expected);
    }
}
