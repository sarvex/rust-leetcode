pub struct Solution;

impl Solution {
    /// Reverses a string in-place using two pointers.
    ///
    /// # Intuition
    /// Swap characters from both ends converging toward the center.
    ///
    /// # Approach
    /// Use the built-in `reverse()` method for idiomatic in-place reversal.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn single_char() {
        let mut s = vec!['a'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['a']);
    }

    #[test]
    fn empty_string() {
        let mut s: Vec<char> = vec![];
        Solution::reverse_string(&mut s);
        assert_eq!(s, Vec::<char>::new());
    }

    #[test]
    fn two_chars() {
        let mut s = vec!['a', 'b'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['b', 'a']);
    }

    #[test]
    fn palindrome() {
        let mut s = vec!['r', 'a', 'c', 'e', 'c', 'a', 'r'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['r', 'a', 'c', 'e', 'c', 'a', 'r']);
    }

    #[test]
    fn with_spaces() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
        Solution::reverse_string(&mut s);
        assert_eq!(
            s,
            vec!['d', 'l', 'r', 'o', 'w', ' ', 'o', 'l', 'l', 'e', 'h']
        );
    }
}
