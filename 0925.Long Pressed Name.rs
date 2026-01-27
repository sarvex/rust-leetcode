impl Solution {
    /// Checks if typed string could be the result of long-pressing the name.
    ///
    /// # Intuition
    /// Group consecutive identical characters in both strings. Each group in
    /// typed must match the name's character and have count >= name's count.
    ///
    /// # Approach
    /// Two pointers compare character groups. For each matching character,
    /// count consecutive occurrences in both strings. If typed has fewer
    /// or the characters differ, return false.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(1)
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let (s, t) = (name.as_bytes(), typed.as_bytes());
        let (m, n) = (s.len(), t.len());
        let (mut i, mut j) = (0, 0);

        while i < m && j < n {
            if s[i] != t[j] {
                return false;
            }
            let mut si = i + 1;
            while si < m && s[si] == s[i] {
                si += 1;
            }
            let mut tj = j + 1;
            while tj < n && t[tj] == t[j] {
                tj += 1;
            }
            if si - i > tj - j {
                return false;
            }
            i = si;
            j = tj;
        }

        i == m && j == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_pressed() {
        assert!(Solution::is_long_pressed_name(
            "alex".to_string(),
            "aaleex".to_string()
        ));
    }

    #[test]
    fn test_not_long_pressed() {
        assert!(!Solution::is_long_pressed_name(
            "saeed".to_string(),
            "ssaaedd".to_string()
        ));
    }

    #[test]
    fn test_exact_match() {
        assert!(Solution::is_long_pressed_name(
            "a".to_string(),
            "a".to_string()
        ));
    }
}
