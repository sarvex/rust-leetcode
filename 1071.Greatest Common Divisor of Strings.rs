impl Solution {
    /// Finds the greatest common divisor of two strings.
    ///
    /// # intuition
    /// If two strings have a common divisor, then concatenating them in either order should yield the same result.
    /// Once we confirm this property, we can find the GCD of their lengths to determine the length of the common divisor.
    ///
    /// # approach
    /// 1. Check if str1 + str2 equals str2 + str1. If not, return an empty string.
    /// 2. Find the GCD of the lengths of the two strings using Euclidean algorithm.
    /// 3. Return the substring of length GCD from either string.
    ///
    /// # complexity
    /// - Time: O(m + n) where m and n are the lengths of the strings
    /// - Space: O(1) as we only use constant extra space
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new();
        }
        
        let gcd = |mut a: usize, mut b: usize| -> usize {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        };
        
        let len_gcd = gcd(str1.len(), str2.len());
        str1[..len_gcd].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_of_strings() {
        let solution = Solution {};
        
        assert_eq!(
            solution.gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        
        assert_eq!(
            solution.gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        
        assert_eq!(
            solution.gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
    }
}
