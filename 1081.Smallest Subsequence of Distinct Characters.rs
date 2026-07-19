impl Solution {
    /// Monotonic stack to find lexicographically smallest subsequence with all distinct characters.
    ///
    /// # Intuition
    /// Use a greedy monotonic stack: when we see a smaller character, pop larger ones from the
    /// stack — but only if they appear again later in the string (so we won't lose them).
    ///
    /// # Approach
    /// 1. Count the last occurrence index of each character.
    /// 2. Iterate through the string maintaining a stack (the result).
    /// 3. For each character:
    ///    - Skip it if already in the stack (it's been placed optimally).
    ///    - Otherwise, pop stack characters that are larger AND still appear later.
    ///    - Push the current character.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — stack and tracking arrays are bounded by 26 letters
    pub fn smallest_subsequence(s: String) -> String {
        let s = s.as_bytes();

        // Last occurrence index for each character
        let mut last = [0usize; 26];
        for (i, &b) in s.iter().enumerate() {
            last[(b - b'a') as usize] = i;
        }

        let mut stack: Vec<u8> = Vec::with_capacity(26);
        let mut in_stack = [false; 26];

        for (i, &b) in s.iter().enumerate() {
            let c = (b - b'a') as usize;

            if in_stack[c] {
                continue;
            }

            // Pop larger characters that still appear later
            while let Some(&top) = stack.last() {
                let t = (top - b'a') as usize;
                if top > b && last[t] > i {
                    stack.pop();
                    in_stack[t] = false;
                } else {
                    break;
                }
            }

            stack.push(b);
            in_stack[c] = true;
        }

        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::smallest_subsequence("bcabc".to_string()), "abc");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::smallest_subsequence("cbacdcbc".to_string()),
            "acdb"
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::smallest_subsequence("a".to_string()), "a");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::smallest_subsequence("aaaa".to_string()), "a");
    }

    #[test]
    fn test_already_sorted_distinct() {
        assert_eq!(Solution::smallest_subsequence("abc".to_string()), "abc");
    }

    #[test]
    fn test_reverse_sorted_distinct() {
        // All distinct, no pops possible — keeps original order
        assert_eq!(Solution::smallest_subsequence("cba".to_string()), "cba");
    }

    #[test]
    fn test_two_chars() {
        assert_eq!(Solution::smallest_subsequence("abab".to_string()), "ab");
        assert_eq!(Solution::smallest_subsequence("baba".to_string()), "ab");
    }
}
