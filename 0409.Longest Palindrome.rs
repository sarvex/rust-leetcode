impl Solution {
    /// Finds the longest palindrome length buildable from the given characters.
    ///
    /// # Intuition
    /// A palindrome uses each character an even number of times, plus at most
    /// one odd-count character in the center.
    ///
    /// # Approach
    /// 1. Count character frequencies using a fixed-size array.
    /// 2. Sum the largest even portion of each frequency.
    /// 3. If any character was left over, add 1 for the center.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 128-entry array
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq = [0u32; 128];
        for b in s.bytes() {
            freq[b as usize] += 1;
        }
        let pairs: u32 = freq.iter().map(|&f| f / 2).sum();
        let total = pairs * 2;
        if total < s.len() as u32 {
            total as i32 + 1
        } else {
            total as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_case() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_palindrome("aaaa".to_string()), 4);
    }
}
