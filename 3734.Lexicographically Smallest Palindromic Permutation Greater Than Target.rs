impl Solution {
    /// Lexicographically Smallest Palindromic Permutation Greater Than Target
    ///
    /// # Intuition
    /// A palindrome is fully determined by its first half (plus optional middle).
    /// We need the smallest half permutation that yields a palindrome > target.
    ///
    /// # Approach
    /// 1. Count frequencies, extract half chars and optional middle char.
    /// 2. Use recursive search: at each position, try chars in order.
    /// 3. If we place a char > target[pos], fill rest with smallest chars.
    /// 4. If we place target[pos], recurse to find valid continuation.
    /// 5. After recursion fails, try larger chars at current position.
    ///
    /// # Complexity
    /// - Time: O(n * 26)
    /// - Space: O(n)
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let n = s.len();
        let mut freq = [0i32; 26];
        s.bytes().for_each(|b| freq[(b - b'a') as usize] += 1);

        let odd_count = freq.iter().filter(|c| **c % 2 == 1).count();
        match (n % 2, odd_count) {
            (0, 0) | (1, 1) => {}
            _ => return String::new(),
        }

        let mid_char = freq
            .iter()
            .enumerate()
            .find(|(_, c)| *c % 2 == 1)
            .map(|(i, _)| b'a' + i as u8);

        let mut half_freq = [0i32; 26];
        (0..26).for_each(|i| half_freq[i] = freq[i] / 2);

        let half_len = n / 2;
        let target_bytes: Vec<u8> = target.bytes().collect();

        if let Some(half) = Self::find_next_half(&half_freq, half_len, &target_bytes, mid_char) {
            return String::from_utf8(Self::build_palindrome(&half, mid_char)).unwrap();
        }

        String::new()
    }

    fn build_palindrome(half: &[u8], mid: Option<u8>) -> Vec<u8> {
        let mut result = half.to_vec();
        if let Some(m) = mid {
            result.push(m);
        }
        result.extend(half.iter().rev());
        result
    }

    fn find_next_half(
        half_freq: &[i32; 26],
        half_len: usize,
        target: &[u8],
        mid: Option<u8>,
    ) -> Option<Vec<u8>> {
        let mut current = vec![0u8; half_len];
        let mut freq = *half_freq;

        Self::solve(&mut current, 0, &mut freq, target, mid)
    }

    /// Compares palindrome (half + mid + half.rev()) with target without allocating.
    #[inline]
    fn palindrome_gt_target(half: &[u8], mid: Option<u8>, target: &[u8]) -> bool {
        let half_len = half.len();
        for i in 0..half_len {
            if half[i] != target[i] {
                return half[i] > target[i];
            }
        }
        if let Some(m) = mid {
            let t_mid = target[half_len];
            if m != t_mid {
                return m > t_mid;
            }
        }
        let start_second = half_len + mid.is_some() as usize;
        for j in 0..half_len {
            let result_val = half[half_len - 1 - j];
            let target_val = target[start_second + j];
            if result_val != target_val {
                return result_val > target_val;
            }
        }
        false
    }

    fn solve(
        current: &mut Vec<u8>,
        pos: usize,
        freq: &mut [i32; 26],
        target: &[u8],
        mid: Option<u8>,
    ) -> Option<Vec<u8>> {
        let half_len = current.len();

        if pos == half_len {
            return if Self::palindrome_gt_target(current, mid, target) {
                Some(current.clone())
            } else {
                None
            };
        }

        let target_char = target[pos];

        // First, try placing target[pos] and recurse (to find smallest valid)
        let target_idx = (target_char - b'a') as usize;
        if freq[target_idx] > 0 {
            freq[target_idx] -= 1;
            current[pos] = target_char;

            if let Some(result) = Self::solve(current, pos + 1, freq, target, mid) {
                freq[target_idx] += 1;
                return Some(result);
            }

            freq[target_idx] += 1;
        }

        // Place char > target[pos]: result is already > target at pos; fill rest and return
        for c in (target_char + 1)..=b'z' {
            let idx = (c - b'a') as usize;
            if freq[idx] > 0 {
                freq[idx] -= 1;
                current[pos] = c;

                let mut rest_pos = pos + 1;
                for ch_idx in 0..26 {
                    for _ in 0..freq[ch_idx] {
                        current[rest_pos] = b'a' + ch_idx as u8;
                        rest_pos += 1;
                    }
                }

                freq[idx] += 1;
                return Some(current.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string()),
            "baab"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string()),
            ""
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string()),
            ""
        );
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            Solution::lex_palindromic_permutation("aac".to_string(), "abb".to_string()),
            "aca"
        );
    }

    #[test]
    fn test_failing_case() {
        assert_eq!(
            Solution::lex_palindromic_permutation("aabb".to_string(), "baaa".to_string()),
            "baab"
        );
    }
}
