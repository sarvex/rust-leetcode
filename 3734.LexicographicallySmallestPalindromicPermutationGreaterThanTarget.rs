/// Lexicographically Smallest Palindromic Permutation Greater Than Target
///
/// # Intuition
/// A palindrome is determined by its first half. We need to find the smallest
/// arrangement of the first half such that the resulting palindrome exceeds target.
///
/// # Approach
/// 1. Count character frequencies and check if palindrome is possible
/// 2. Extract the "half counts" (each count divided by 2) and middle char if odd length
/// 3. Try to build smallest first-half that makes palindrome > target:
///    - First try: match target's first half as closely as possible, then increment
///    - Use a greedy approach with backtracking when needed
///
/// # Complexity
/// - Time: O(n * 26) for building permutations
/// - Space: O(n) for storing the result
impl Solution {
    pub fn smallest_palindrome(s: String, target: String) -> String {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let target_bytes = target.as_bytes();

        // Count character frequencies
        let mut freq = [0i32; 26];
        for &b in s_bytes {
            freq[(b - b'a') as usize] += 1;
        }

        // Check if palindrome is possible and find middle char
        let mut odd_char: Option<u8> = None;
        let mut odd_count = 0;
        for i in 0..26 {
            if freq[i] % 2 == 1 {
                odd_count += 1;
                odd_char = Some(b'a' + i as u8);
            }
        }

        // Palindrome impossible if more than one odd frequency
        // or if odd frequency exists but length is even
        if odd_count > 1 || (odd_count == 1 && n % 2 == 0) {
            return String::new();
        }

        // For even length with no odd chars, or odd length with exactly one odd char
        if n % 2 == 1 && odd_char.is_none() {
            return String::new();
        }

        // Half counts for building first half of palindrome
        let mut half_freq = [0i32; 26];
        for i in 0..26 {
            half_freq[i] = freq[i] / 2;
        }

        let half_len = n / 2;
        let mut first_half = vec![0u8; half_len];

        // Build palindrome from half
        let build_palindrome = |half: &[u8], mid: Option<u8>| -> Vec<u8> {
            let mut result = half.to_vec();
            if let Some(m) = mid {
                result.push(m);
            }
            result.extend(half.iter().rev());
            result
        };

        // Try to build first_half that results in palindrome > target
        if Self::find_next_half(
            &mut first_half,
            &half_freq,
            &target_bytes,
            half_len,
            odd_char,
        ) {
            let result = build_palindrome(&first_half, odd_char);
            return String::from_utf8(result).unwrap();
        }

        String::new()
    }

    fn find_next_half(
        half: &mut [u8],
        freq: &[i32; 26],
        target: &[u8],
        half_len: usize,
        mid: Option<u8>,
    ) -> bool {
        // Try each position from left to right
        // At position i, we either:
        // 1. Match target[i] exactly and continue
        // 2. Place something > target[i] and fill rest with smallest

        fn solve(
            pos: usize,
            half: &mut [u8],
            avail: &mut [i32; 26],
            target: &[u8],
            half_len: usize,
            mid: Option<u8>,
            must_be_greater: bool,
        ) -> bool {
            if pos == half_len {
                // Build full palindrome and compare
                if must_be_greater {
                    return true;
                }
                // Check middle character
                if let Some(m) = mid {
                    if m > target[half_len] {
                        return true;
                    } else if m < target[half_len] {
                        return false;
                    }
                }
                // Check second half (which mirrors first half)
                // second_half[i] = half[half_len - 1 - i]
                // corresponds to target[half_len + 1 + i] (if mid exists) or target[half_len + i]
                let offset = if mid.is_some() {
                    half_len + 1
                } else {
                    half_len
                };
                for i in 0..half_len {
                    let palindrome_char = half[half_len - 1 - i];
                    let target_char = target[offset + i];
                    if palindrome_char > target_char {
                        return true;
                    } else if palindrome_char < target_char {
                        return false;
                    }
                }
                false // Equal, not strictly greater
            } else if must_be_greater {
                // Fill with smallest available
                for c in 0..26 {
                    if avail[c] > 0 {
                        half[pos] = b'a' + c as u8;
                        avail[c] -= 1;
                        if solve(pos + 1, half, avail, target, half_len, mid, true) {
                            return true;
                        }
                        avail[c] += 1;
                    }
                }
                false
            } else {
                // Try chars >= target[pos]
                let min_char = (target[pos] - b'a') as usize;

                // First try exact match
                if avail[min_char] > 0 {
                    half[pos] = target[pos];
                    avail[min_char] -= 1;
                    if solve(pos + 1, half, avail, target, half_len, mid, false) {
                        return true;
                    }
                    avail[min_char] += 1;
                }

                // Then try larger chars
                for c in (min_char + 1)..26 {
                    if avail[c] > 0 {
                        half[pos] = b'a' + c as u8;
                        avail[c] -= 1;
                        if solve(pos + 1, half, avail, target, half_len, mid, true) {
                            return true;
                        }
                        avail[c] += 1;
                    }
                }
                false
            }
        }

        let mut avail = *freq;
        solve(0, half, &mut avail, target, half_len, mid, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::smallest_palindrome("baba".to_string(), "abba".to_string()),
            "baab"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::smallest_palindrome("baba".to_string(), "bbaa".to_string()),
            ""
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::smallest_palindrome("abc".to_string(), "abb".to_string()),
            ""
        );
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            Solution::smallest_palindrome("aac".to_string(), "abb".to_string()),
            "aca"
        );
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::smallest_palindrome("baba".to_string(), "abba".to_string())
    );
}
