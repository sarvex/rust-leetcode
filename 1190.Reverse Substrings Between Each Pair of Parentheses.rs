impl Solution {
    /// Emits letters in reverse-parentheses order by jumping between pairs.
    ///
    /// # Intuition
    /// Reversing a parenthesized span is the same as reading it backwards.
    /// Each matching pair is a portal: landing on one parenthesis sends the
    /// scan to its partner and flips direction. Nested reversals then fall out
    /// of a single walk, because an inner pair flips direction again.
    ///
    /// # Approach
    /// 1. Record the partner index of every parenthesis with a stack.
    /// 2. Start at index 0 moving forward.
    /// 3. Append each letter.
    /// 4. On a parenthesis, jump to its partner, negate the direction, and
    ///    continue one step in the new direction.
    ///
    /// The input is guaranteed to be balanced, so every closing parenthesis
    /// has an opening partner on the stack.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    ///
    /// # Panics
    /// Panics if `s` contains an unmatched closing parenthesis.
    pub fn reverse_parentheses(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut partner = vec![0; n];
        let mut open = Vec::with_capacity(n / 2);

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'(' {
                open.push(i);
            } else if byte == b')' {
                let start = open.pop().expect("parentheses are balanced");
                partner[start] = i;
                partner[i] = start;
            }
        }

        let mut result = String::with_capacity(n);
        let mut index = 0isize;
        let mut direction = 1isize;
        let limit = n as isize;

        while index < limit {
            let cursor = index as usize;
            if bytes[cursor] == b'(' || bytes[cursor] == b')' {
                index = partner[cursor] as isize;
                direction = -direction;
            } else {
                result.push(char::from(bytes[cursor]));
            }
            index += direction;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::reverse_parentheses("(abcd)".to_string()), "dcba");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_string()),
            "iloveu"
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
            "leetcode"
        );
    }

    #[test]
    fn test_no_parentheses() {
        assert_eq!(Solution::reverse_parentheses("abcd".to_string()), "abcd");
    }

    #[test]
    fn test_single_letter() {
        assert_eq!(Solution::reverse_parentheses("(a)".to_string()), "a");
    }

    #[test]
    fn test_empty_pair() {
        assert_eq!(Solution::reverse_parentheses("()".to_string()), "");
    }

    #[test]
    fn test_adjacent_pairs() {
        assert_eq!(
            Solution::reverse_parentheses("(ab)(cd)".to_string()),
            "badc"
        );
    }

    #[test]
    fn test_double_reverse_cancels() {
        assert_eq!(Solution::reverse_parentheses("((ab))".to_string()), "ab");
    }

    #[test]
    fn test_letters_outside_parentheses() {
        assert_eq!(Solution::reverse_parentheses("a(bc)d".to_string()), "acbd");
    }

    #[test]
    fn test_nested_with_outer_letters() {
        assert_eq!(
            Solution::reverse_parentheses("a(bcdefghijkl(mno)p)q".to_string()),
            "apmnolkjihgfedcbq"
        );
    }
}
