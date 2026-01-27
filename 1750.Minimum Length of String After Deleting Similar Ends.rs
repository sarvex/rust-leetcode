impl Solution {
    /// Two-pointer deletion of matching characters from both ends.
    ///
    /// # Intuition
    /// Repeatedly remove matching prefix and suffix characters. When both ends
    /// share the same character, shrink inward past all copies of that character.
    ///
    /// # Approach
    /// 1. Use two pointers `left` and `right` starting at both ends.
    /// 2. While `left < right` and `s[left] == s[right]`, advance both past all
    ///    copies of the matching character.
    /// 3. Return the remaining length.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut left, mut right) = (0usize, s.len() - 1);
        while left < right && s[left] == s[right] {
            let ch = s[left];
            while left <= right && s[left] == ch {
                left += 1;
            }
            while right >= left && s[right] == ch {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
            if left > right {
                return 0;
            }
        }
        (right - left + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    }

    #[test]
    fn test_example_three() {
        assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    }
}
