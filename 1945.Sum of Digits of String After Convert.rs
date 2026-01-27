impl Solution {
    /// Converts a string to digits and repeatedly sums them k times.
    ///
    /// # Intuition
    /// First convert each letter to its position (a=1, ..., z=26), then
    /// concatenate all digits. Apply digit-sum transformation k times.
    ///
    /// # Approach
    /// 1. Convert letters to their numeric string representation.
    /// 2. Repeat k times: sum all digits of the current number string.
    /// 3. Return the final result.
    ///
    /// # Complexity
    /// - Time: O(n + k * d) where d is digit count
    /// - Space: O(n)
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut num_str: String = s
            .bytes()
            .flat_map(|b| (b - b'a' + 1).to_string().into_bytes())
            .map(|b| b as char)
            .collect();

        for _ in 0..k {
            let sum: i32 = num_str.bytes().map(|b| (b - b'0') as i32).sum();
            num_str = sum.to_string();
        }

        num_str.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36);
    }

    #[test]
    fn test_multiple_transforms() {
        assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::get_lucky("a".to_string(), 1), 1);
    }
}
