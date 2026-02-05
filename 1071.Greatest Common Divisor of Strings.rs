impl Solution {
    /// Finds the greatest common divisor of two strings.
    ///
    /// # Intuition
    /// If two strings have a common divisor, concatenating them in either
    /// order yields the same result. The GCD length determines the divisor.
    ///
    /// # Approach
    /// Verify `str1 + str2 == str2 + str1`. Compute GCD of their lengths
    /// using the Euclidean algorithm. Return the prefix of that length.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(m + n) for concatenation check
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{str1}{str2}") != format!("{str2}{str1}") {
            return String::new();
        }

        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        str1[..gcd(str1.len(), str2.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn test_partial() {
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB"
        );
    }

    #[test]
    fn test_no_gcd() {
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            ""
        );
    }
}
