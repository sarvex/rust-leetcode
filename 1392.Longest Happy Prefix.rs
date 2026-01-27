impl Solution {
    /// KMP failure function to find the longest happy prefix.
    ///
    /// # Intuition
    /// The longest happy prefix is the longest proper prefix that is also a
    /// suffix. This is exactly what the KMP failure function computes. The
    /// value at the last position gives the length of the longest happy prefix.
    ///
    /// # Approach
    /// 1. Build the KMP failure (partial match) table
    /// 2. The last entry gives the length of the longest prefix-suffix
    /// 3. Return the corresponding substring
    ///
    /// # Complexity
    /// - Time: O(n) for building the failure table
    /// - Space: O(n) for the failure table
    pub fn longest_prefix(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut fail = vec![0usize; n];

        let mut k = 0;
        for i in 1..n {
            while k > 0 && bytes[k] != bytes[i] {
                k = fail[k - 1];
            }
            if bytes[k] == bytes[i] {
                k += 1;
            }
            fail[i] = k;
        }

        s[..fail[n - 1]].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_prefix_exists() {
        assert_eq!(Solution::longest_prefix("level".to_string()), "l");
    }

    #[test]
    fn longer_prefix() {
        assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab");
    }

    #[test]
    fn no_happy_prefix() {
        assert_eq!(Solution::longest_prefix("abcdef".to_string()), "");
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::longest_prefix("a".to_string()), "");
    }
}
