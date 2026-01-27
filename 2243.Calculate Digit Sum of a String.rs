impl Solution {
    /// Repeatedly replaces groups of k digits with their digit sum until short enough.
    ///
    /// # Intuition
    /// Each round partitions the string into chunks of size k, replaces each chunk
    /// with the string representation of its digit sum, and concatenates the results.
    ///
    /// # Approach
    /// Use a while loop that continues until `s.len() <= k`. Each iteration maps
    /// byte chunks to their digit sums via iterator combinators and collects into
    /// a new string.
    ///
    /// # Complexity
    /// - Time: O(n × log_k(n)) where n is the initial string length
    /// - Space: O(n) for intermediate strings
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s = s;
        while s.len() > k {
            s = s
                .as_bytes()
                .chunks(k)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|&c| (c - b'0') as i32)
                        .sum::<i32>()
                        .to_string()
                })
                .collect();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::digit_sum("11111222223".to_string(), 3),
            "135".to_string()
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::digit_sum("00000000".to_string(), 3),
            "000".to_string()
        );
    }

    #[test]
    fn test_already_short() {
        assert_eq!(Solution::digit_sum("123".to_string(), 5), "123".to_string());
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::digit_sum("9".to_string(), 1), "9".to_string());
    }
}
