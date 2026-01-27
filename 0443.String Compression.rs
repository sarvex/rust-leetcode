impl Solution {
    /// Compresses a character array in-place using run-length encoding.
    ///
    /// # Intuition
    /// Walk through runs of identical characters, writing each character
    /// followed by the count (if > 1) back into the same array.
    ///
    /// # Approach
    /// 1. Use two pointers: `i` for reading runs, `k` for writing.
    /// 2. For each run, write the character and, if the run length > 1,
    ///    write the digits of the count.
    /// 3. Return `k` as the new length.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) in-place
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let (mut i, mut k) = (0, 0);
        while i < n {
            let ch = chars[i];
            let mut j = i + 1;
            while j < n && chars[j] == ch {
                j += 1;
            }
            chars[k] = ch;
            k += 1;
            let run = j - i;
            if run > 1 {
                for digit in run.to_string().chars() {
                    chars[k] = digit;
                    k += 1;
                }
            }
            i = j;
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(&chars[..len], &['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn test_single_char() {
        let mut chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);
        assert_eq!(chars[0], 'a');
    }

    #[test]
    fn test_long_run() {
        let mut chars = vec!['a'; 12];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(&chars[..len], &['a', '1', '2']);
    }
}
