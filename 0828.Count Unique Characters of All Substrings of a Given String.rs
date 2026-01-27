impl Solution {
    /// Counts unique characters across all substrings using contribution technique.
    ///
    /// # Intuition
    /// Each character contributes to substrings where it appears exactly once.
    /// For character at position `j`, it is unique in substrings `[i..=k]`
    /// where `i` is after the previous occurrence and `k` is before the next.
    ///
    /// # Approach
    /// For each character, record its occurrence positions with sentinel
    /// boundaries (-1 and n). The contribution of position `j` is
    /// `(j - prev) * (next - j)`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for position lists
    pub fn unique_letter_string(s: String) -> i32 {
        let n = s.len();
        let mut positions: Vec<Vec<i32>> = vec![vec![-1]; 26];

        for (i, b) in s.bytes().enumerate() {
            positions[(b - b'A') as usize].push(i as i32);
        }

        positions
            .iter()
            .map(|pos| {
                let mut v = pos.clone();
                v.push(n as i32);
                (1..v.len() - 1)
                    .map(|i| (v[i] - v[i - 1]) as i64 * (v[i + 1] - v[i]) as i64)
                    .sum::<i64>()
            })
            .sum::<i64>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abc() {
        assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
    }

    #[test]
    fn test_aba() {
        assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::unique_letter_string("A".to_string()), 1);
    }
}
