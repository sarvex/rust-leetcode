impl Solution {
    /// Check if concatenation of n, 2n, and 3n contains digits 1-9 exactly once.
    ///
    /// # Intuition
    /// A fascinating number produces a 9-digit string with each digit 1-9
    /// appearing exactly once and no zeros.
    ///
    /// # Approach
    /// 1. Concatenate `n`, `2n`, and `3n` into a single string.
    /// 2. Verify total length is 9.
    /// 3. Count digit frequencies; ensure every digit 1-9 appears once and 0 never.
    ///
    /// # Complexity
    /// - Time: O(1) — the concatenated string has at most 9 digits
    /// - Space: O(1)
    pub fn is_fascinating(n: i32) -> bool {
        let s = format!("{}{}{}", n, n * 2, n * 3);
        if s.len() != 9 {
            return false;
        }
        let mut cnt = [0u8; 10];
        for b in s.as_bytes() {
            let d = (b - b'0') as usize;
            cnt[d] += 1;
            if cnt[d] > 1 {
                return false;
            }
        }
        cnt[0] == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fascinating_number() {
        assert!(Solution::is_fascinating(192));
    }

    #[test]
    fn not_fascinating_due_to_repeats() {
        assert!(!Solution::is_fascinating(100));
    }

    #[test]
    fn not_fascinating_wrong_length() {
        assert!(!Solution::is_fascinating(99));
    }
}
