impl Solution {
    /// Finds all start indices of anagrams of `p` in `s` using a sliding window.
    ///
    /// # Intuition
    /// Maintain a frequency difference array between the current window and `p`.
    /// When all counts are zero, the window is an anagram.
    ///
    /// # Approach
    /// 1. Initialize a 26-element count array from the first window.
    /// 2. Slide the window by one character, updating counts.
    /// 3. Track the number of zero-count entries to check anagram in O(1).
    ///
    /// # Complexity
    /// - Time: O(m) where m = s.len()
    /// - Space: O(1) — fixed 26-element array
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (m, n) = (s.len(), p.len());
        if m < n {
            return vec![];
        }
        let mut cnt = [0i32; 26];
        for i in 0..n {
            cnt[(p[i] - b'a') as usize] += 1;
            cnt[(s[i] - b'a') as usize] -= 1;
        }
        let mut result = Vec::new();
        if cnt.iter().all(|&v| v == 0) {
            result.push(0);
        }
        for i in n..m {
            cnt[(s[i] - b'a') as usize] -= 1;
            cnt[(s[i - n] - b'a') as usize] += 1;
            if cnt.iter().all(|&v| v == 0) {
                result.push((i - n + 1) as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
    }

    #[test]
    fn test_all_anagrams() {
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            Solution::find_anagrams("hello".to_string(), "xyz".to_string()),
            Vec::<i32>::new()
        );
    }
}
