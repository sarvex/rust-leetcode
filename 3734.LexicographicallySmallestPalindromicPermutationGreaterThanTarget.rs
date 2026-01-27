impl Solution {
    /// Finds lexicographically smallest palindromic permutation greater than target.
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
    pub fn smallest_palindrome(s: String, target: String) -> String {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let target_bytes = target.as_bytes();

        let mut freq = [0i32; 26];
        for &b in s_bytes {
            freq[(b - b'a') as usize] += 1;
        }

        let odd_indices: Vec<usize> = freq
            .iter()
            .enumerate()
            .filter(|(_, &c)| c % 2 == 1)
            .map(|(i, _)| i)
            .collect();

        match (n % 2, odd_indices.len()) {
            (0, 0) | (1, 1) => {}
            _ => return String::new(),
        }

        let odd_char = odd_indices.first().map(|&i| b'a' + i as u8);

        let mut half_freq = [0i32; 26];
        (0..26).for_each(|i| half_freq[i] = freq[i] / 2);

        let half_len = n / 2;
        let mut first_half = vec![0u8; half_len];

        let build_palindrome = |half: &[u8], mid: Option<u8>| -> Vec<u8> {
            let mut result = half.to_vec();
            if let Some(m) = mid {
                result.push(m);
            }
            result.extend(half.iter().rev());
            result
        };

        if Self::find_next_half(
            &mut first_half,
            &half_freq,
            target_bytes,
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
        fn solve(
            pos: usize,
            half: &mut [u8],
            avail: &mut [i32; 26],
            target: &[u8],
            half_len: usize,
            mid: Option<u8>,
            must_be_greater: bool,
        ) -> bool {
            match (pos == half_len, must_be_greater) {
                (true, true) => true,
                (true, false) => {
                    if let Some(m) = mid {
                        match m.cmp(&target[half_len]) {
                            std::cmp::Ordering::Greater => return true,
                            std::cmp::Ordering::Less => return false,
                            std::cmp::Ordering::Equal => {}
                        }
                    }
                    let offset = match mid {
                        Some(_) => half_len + 1,
                        None => half_len,
                    };
                    for i in 0..half_len {
                        match half[half_len - 1 - i].cmp(&target[offset + i]) {
                            std::cmp::Ordering::Greater => return true,
                            std::cmp::Ordering::Less => return false,
                            std::cmp::Ordering::Equal => {}
                        }
                    }
                    false
                }
                (false, true) => (0..26).any(|c| {
                    if avail[c] > 0 {
                        half[pos] = b'a' + c as u8;
                        avail[c] -= 1;
                        let found = solve(pos + 1, half, avail, target, half_len, mid, true);
                        avail[c] += 1;
                        found
                    } else {
                        false
                    }
                }),
                (false, false) => {
                    let min_char = (target[pos] - b'a') as usize;

                    if avail[min_char] > 0 {
                        half[pos] = target[pos];
                        avail[min_char] -= 1;
                        if solve(pos + 1, half, avail, target, half_len, mid, false) {
                            avail[min_char] += 1;
                            return true;
                        }
                        avail[min_char] += 1;
                    }

                    ((min_char + 1)..26).any(|c| {
                        if avail[c] > 0 {
                            half[pos] = b'a' + c as u8;
                            avail[c] -= 1;
                            let found = solve(pos + 1, half, avail, target, half_len, mid, true);
                            avail[c] += 1;
                            found
                        } else {
                            false
                        }
                    })
                }
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
    fn test_basic_palindrome_increment() {
        assert_eq!(
            Solution::smallest_palindrome("baba".to_string(), "abba".to_string()),
            "baab"
        );
    }

    #[test]
    fn test_no_larger_palindrome_exists() {
        assert_eq!(
            Solution::smallest_palindrome("baba".to_string(), "bbaa".to_string()),
            ""
        );
    }

    #[test]
    fn test_impossible_odd_frequency() {
        assert_eq!(
            Solution::smallest_palindrome("abc".to_string(), "abb".to_string()),
            ""
        );
    }

    #[test]
    fn test_odd_length_palindrome() {
        assert_eq!(
            Solution::smallest_palindrome("aac".to_string(), "abb".to_string()),
            "aca"
        );
    }
}
