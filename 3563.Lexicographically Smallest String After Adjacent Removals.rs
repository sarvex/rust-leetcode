impl Solution {
    /// Lexicographically smallest string after adjacent removals using interval DP.
    ///
    /// # Intuition
    /// Use interval DP to precompute which even-length substrings can be fully removed,
    /// then greedily build the lexicographically smallest result.
    ///
    /// # Approach
    /// 1. `e[i][j]` indicates if substring s[i..j] can be completely removed
    /// 2. Base case: empty substrings (i==j) are trivially removable
    /// 3. For each even-length substring, check if:
    ///    - Outer chars are consecutive and inner is removable, OR
    ///    - It splits into two removable even-length parts
    /// 4. Build result backwards: at each position, take minimum of keeping char or skipping removable prefix
    ///
    /// # Complexity
    /// - Time: O(n³) for interval DP
    /// - Space: O(n²) for the DP tables
    pub fn lexicographically_smallest_string(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut e = vec![vec![false; n + 1]; n + 1];

        for i in (0..n).rev() {
            e[i][i] = true;
            for j in 1..=(n - i) / 2 {
                let end = i + j * 2;
                if e[i + 1][end - 1] {
                    let d = (s[i] as i32 - s[end - 1] as i32).abs();
                    if d == 1 || d == 25 {
                        e[i][end] = true;
                        continue;
                    }
                }
                for k in 1..j {
                    if e[i][i + k * 2] && e[i + k * 2][end] {
                        e[i][end] = true;
                        break;
                    }
                }
            }
        }

        let mut sub = vec![String::new(); n + 1];
        for i in (0..n).rev() {
            let base = format!("{}{}", s[i] as char, &sub[i + 1]);
            sub[i] = (1..=(n - i) / 2)
                .filter(|&j| e[i][i + j * 2])
                .map(|j| sub[i + j * 2].clone())
                .fold(base, |acc, s| acc.min(s));
        }

        sub[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::lexicographically_smallest_string("abc".to_string()),
            "a"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::lexicographically_smallest_string("bcda".to_string()),
            ""
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::lexicographically_smallest_string("zdce".to_string()),
            "zdce"
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::lexicographically_smallest_string("a".to_string()),
            "a"
        );
    }

    #[test]
    fn test_circular_za() {
        assert_eq!(
            Solution::lexicographically_smallest_string("za".to_string()),
            ""
        );
    }

    #[test]
    fn test_circular_az() {
        assert_eq!(
            Solution::lexicographically_smallest_string("az".to_string()),
            ""
        );
    }

    #[test]
    fn test_no_removal_possible() {
        assert_eq!(
            Solution::lexicographically_smallest_string("ac".to_string()),
            "ac"
        );
    }

    #[test]
    fn test_nested_removal() {
        assert_eq!(
            Solution::lexicographically_smallest_string("abba".to_string()),
            ""
        );
    }

    #[test]
    fn test_choose_smaller() {
        assert_eq!(
            Solution::lexicographically_smallest_string("afedc".to_string()),
            "a"
        );
    }

    #[test]
    fn test_empty_result() {
        assert_eq!(
            Solution::lexicographically_smallest_string("ba".to_string()),
            ""
        );
    }
}
