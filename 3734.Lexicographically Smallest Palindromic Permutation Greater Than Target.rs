impl Solution {
    /// Lexicographically Smallest Palindromic Permutation Greater Than Target
    ///
    /// # Intuition
    /// A palindrome is determined by its first half. We need to find the smallest
    /// palindromic permutation of s that is strictly greater than target. We work
    /// with the first half of the palindrome and find the next permutation.
    ///
    /// # Approach
    /// 1. Count character frequencies in s. For a palindrome, at most one char can
    ///    have odd frequency (for odd-length strings).
    /// 2. If more than one char has odd frequency, no palindrome exists.
    /// 3. Build the first half using half of each character's count (sorted).
    /// 4. Extract the first half of target.
    /// 5. Find the next permutation of our half that is greater than target's half.
    /// 6. Reconstruct the full palindrome from the resulting half.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let n = s.len();
        let mut freq = [0u32; 26];

        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let odd_count = freq.iter().filter(|&&c| c % 2 == 1).count();
        if (n % 2 == 0 && odd_count > 0) || (n % 2 == 1 && odd_count != 1) {
            return String::new();
        }

        let mut mid_char: Option<u8> = None;
        let mut half: Vec<u8> = Vec::with_capacity(n / 2);

        for (i, &count) in freq.iter().enumerate() {
            let ch = b'a' + i as u8;
            if count % 2 == 1 {
                mid_char = Some(ch);
            }
            for _ in 0..count / 2 {
                half.push(ch);
            }
        }

        let target_bytes: Vec<u8> = target.bytes().collect();

        if let Some(result) = Self::find_next(&half, mid_char, &target_bytes) {
            return String::from_utf8(result).unwrap();
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

    fn find_next(half: &[u8], mid: Option<u8>, target: &[u8]) -> Option<Vec<u8>> {
        let len = half.len();

        // Build smallest palindrome and check if it's already greater
        let smallest = Self::build_palindrome(half, mid);
        if smallest.as_slice() > target {
            return Some(smallest);
        }

        // Try to find next permutation of half that makes palindrome > target
        let mut current = half.to_vec();

        // For odd length, check if matching first half but with mid char makes it greater
        if len == 0 {
            // Single character palindrome - only option is the mid char
            return None; // Already checked above with smallest
        }

        let target_half = &target[..len];

        // Try positions from right to left
        for i in (0..len).rev() {
            // Collect available chars from position i onwards
            let mut avail = [0u32; 26];
            for j in i..len {
                avail[(current[j] - b'a') as usize] += 1;
            }

            let target_ch = target_half[i];

            // Try each char greater than target[i]
            for c in (target_ch + 1)..=b'z' {
                let idx = (c - b'a') as usize;
                if avail[idx] > 0 {
                    avail[idx] -= 1;
                    current[i] = c;

                    // Fill remaining with smallest available
                    let mut pos = i + 1;
                    for ch_idx in 0..26 {
                        let ch = b'a' + ch_idx as u8;
                        for _ in 0..avail[ch_idx] {
                            current[pos] = ch;
                            pos += 1;
                        }
                    }

                    return Some(Self::build_palindrome(&current, mid));
                }
            }

            // Try placing target[i] and continue deeper
            let idx = (target_ch - b'a') as usize;
            if avail[idx] > 0 {
                avail[idx] -= 1;
                current[i] = target_ch;

                // Build smallest suffix
                let mut suffix = Vec::with_capacity(len - i - 1);
                for ch_idx in 0..26 {
                    let ch = b'a' + ch_idx as u8;
                    for _ in 0..avail[ch_idx] {
                        suffix.push(ch);
                    }
                }

                // Check if this suffix makes half > target_half
                if suffix.as_slice() > &target_half[i + 1..] {
                    for (j, &ch) in suffix.iter().enumerate() {
                        current[i + 1 + j] = ch;
                    }
                    return Some(Self::build_palindrome(&current, mid));
                }

                // Check if suffix equals target_half suffix - then mid char decides
                if suffix.as_slice() == &target_half[i + 1..] {
                    for (j, &ch) in suffix.iter().enumerate() {
                        current[i + 1 + j] = ch;
                    }
                    // Now check if the palindrome > target considering mid and second half
                    let candidate = Self::build_palindrome(&current, mid);
                    if candidate.as_slice() > target {
                        return Some(candidate);
                    }
                }
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
}
