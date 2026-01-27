impl Solution {
    /// Counts the number of substrings whose mapped digit sum is divisible by their length.
    ///
    /// # Intuition
    /// Each character maps to a digit group (1-9). A substring is "divisible" if the
    /// sum of its mapped digits is divisible by its length.
    ///
    /// # Approach
    /// 1. Build a mapping array from character to digit value based on predefined groups.
    /// 2. Enumerate all substrings, maintaining a running sum.
    /// 3. Check divisibility of the running sum by the substring length.
    ///
    /// # Complexity
    /// - Time: O(n²) for all substring pairs
    /// - Space: O(1) aside from the fixed-size mapping array
    pub fn count_divisible_substrings(word: String) -> i32 {
        let groups = ["ab", "cde", "fgh", "ijk", "lmn", "opq", "rst", "uvw", "xyz"];
        let mut mp = [0i32; 26];
        groups.iter().enumerate().for_each(|(i, s)| {
            s.as_bytes()
                .iter()
                .for_each(|&b| mp[(b - b'a') as usize] = (i + 1) as i32);
        });

        let bytes = word.as_bytes();
        let n = bytes.len();
        let mut ans = 0;

        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += mp[(bytes[j] - b'a') as usize];
                if sum % (j - i + 1) as i32 == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_divisible() {
        assert_eq!(Solution::count_divisible_substrings("asdf".to_string()), 6);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::count_divisible_substrings("a".to_string()), 1);
    }
}
