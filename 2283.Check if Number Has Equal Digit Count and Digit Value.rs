impl Solution {
    /// Checks if each digit's value equals the count of that index digit in the string.
    ///
    /// # Intuition
    /// Build a frequency table of digits, then verify each position i has a value
    /// matching the frequency of digit i.
    ///
    /// # Approach
    /// 1. Count digit frequencies using `as_bytes()` for efficient access
    /// 2. Enumerate characters and check each position's value against the frequency
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed-size frequency array of 10 elements
    pub fn digit_count(num: String) -> bool {
        let bytes = num.as_bytes();
        let mut freq = [0u8; 10];
        bytes.iter().for_each(|b| freq[(b - b'0') as usize] += 1);

        bytes.iter().enumerate().all(|(i, b)| (b - b'0') == freq[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert!(Solution::digit_count("1210".to_string()));
    }

    #[test]
    fn test_example_two() {
        assert!(!Solution::digit_count("030".to_string()));
    }

    #[test]
    fn test_single_zero() {
        assert!(!Solution::digit_count("0".to_string()));
    }

    #[test]
    fn test_2020() {
        assert!(Solution::digit_count("2020".to_string()));
    }
}
