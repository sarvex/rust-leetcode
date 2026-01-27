impl Solution {
    /// Checks if any permutation of s1 is a substring of s2 using a sliding window.
    ///
    /// # Intuition
    /// A permutation match means a window of size |s1| in s2 has the same
    /// character frequencies. Track how many characters have matching counts.
    ///
    /// # Approach
    /// 1. Build a frequency count from s1.
    /// 2. Slide a window of size |s1| over s2, decrementing/incrementing counts.
    /// 3. Track the number of characters with zero balance (fully matched).
    ///
    /// # Complexity
    /// - Time: O(n) where n = s2.len()
    /// - Space: O(1) — fixed 26-element array
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let m = s1.len();
        let n = s2.len();
        if m > n {
            return false;
        }
        let mut cnt = [0i32; 26];
        let mut need = 0;
        for b in s1.bytes() {
            let idx = (b - b'a') as usize;
            if cnt[idx] == 0 {
                need += 1;
            }
            cnt[idx] += 1;
        }
        let s2 = s2.as_bytes();
        for i in 0..n {
            let c = (s2[i] - b'a') as usize;
            cnt[c] -= 1;
            if cnt[c] == 0 {
                need -= 1;
            }
            if i >= m {
                let c = (s2[i - m] - b'a') as usize;
                cnt[c] += 1;
                if cnt[c] == 1 {
                    need += 1;
                }
            }
            if need == 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
    }

    #[test]
    fn test_not_found() {
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ));
    }

    #[test]
    fn test_exact_match() {
        assert!(Solution::check_inclusion(
            "abc".to_string(),
            "bca".to_string()
        ));
    }
}
