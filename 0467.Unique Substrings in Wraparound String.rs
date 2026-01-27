impl Solution {
    /// Counts unique substrings of `s` that appear in the infinite wraparound string.
    ///
    /// # Intuition
    /// A substring is valid if its characters are consecutive in the wraparound
    /// alphabet (a→b→...→z→a). For each ending character, track the longest
    /// valid substring ending there; the total count is the sum of these lengths.
    ///
    /// # Approach
    /// 1. Scan `s`, extending the current run when characters are consecutive.
    /// 2. For each character, update the maximum run length ending at that character.
    /// 3. Sum all 26 maximum lengths.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 26-element array
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut max_len = [0i32; 26];
        let mut run = 0i32;
        for (i, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            if i > 0 && (idx as i32 - (bytes[i - 1] - b'a') as i32 + 26) % 26 == 1 {
                run += 1;
            } else {
                run = 1;
            }
            max_len[idx] = max_len[idx].max(run);
        }
        max_len.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string("a".to_string()),
            1
        );
    }

    #[test]
    fn test_cac() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string("cac".to_string()),
            2
        );
    }

    #[test]
    fn test_zab() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string("zab".to_string()),
            6
        );
    }
}
