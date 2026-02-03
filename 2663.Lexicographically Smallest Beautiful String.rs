impl Solution {
    /// Greedy right-to-left increment with minimal suffix fill.
    ///
    /// # Intuition
    /// A string is beautiful iff it has no equal adjacent letters and no length-3 palindrome, so
    /// each position only depends on the previous two characters. To get the smallest string that
    /// is strictly larger than `s`, we should increase the rightmost possible position and then
    /// fill the suffix with the smallest valid letters.
    ///
    /// # Approach
    /// Convert `s` into indices in `[0, k)`. Scan from right to left:
    /// - Try the next larger letter at position `i` that differs from positions `i-1` and `i-2`.
    /// - If found, fix it and greedily fill positions `i+1..` with the smallest valid letters.
    /// If no position can be increased, return the empty string.
    ///
    /// # Complexity
    /// - Time: O(n * k)
    /// - Space: O(n)
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        let k = k as u8;
        let mut codes: Vec<u8> = s.bytes().map(|b| b - b'a').collect();
        let n = codes.len();

        for i in (0..n).rev() {
            let current = codes[i];
            for next in (current + 1)..k {
                if i >= 1 && next == codes[i - 1] {
                    continue;
                }
                if i >= 2 && next == codes[i - 2] {
                    continue;
                }

                codes[i] = next;
                for j in (i + 1)..n {
                    for cand in 0..k {
                        if j >= 1 && cand == codes[j - 1] {
                            continue;
                        }
                        if j >= 2 && cand == codes[j - 2] {
                            continue;
                        }
                        codes[j] = cand;
                        break;
                    }
                }

                let bytes: Vec<u8> = codes.iter().map(|&c| b'a' + c).collect();
                return String::from_utf8(bytes).expect("valid lowercase ascii");
            }
            codes[i] = current;
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let s = "abcz".to_string();
        assert_eq!(Solution::smallest_beautiful_string(s, 26), "abda");
    }

    #[test]
    fn example_two() {
        let s = "dc".to_string();
        assert_eq!(Solution::smallest_beautiful_string(s, 4), "");
    }

    #[test]
    fn single_char_next() {
        let s = "a".to_string();
        assert_eq!(Solution::smallest_beautiful_string(s, 4), "b");
    }

    #[test]
    fn single_char_none() {
        let s = "d".to_string();
        assert_eq!(Solution::smallest_beautiful_string(s, 4), "");
    }

    #[test]
    fn carry_with_suffix_fill() {
        let s = "az".to_string();
        assert_eq!(Solution::smallest_beautiful_string(s, 26), "ba");
    }
}
