impl Solution {
    /// Finds the k-th lexicographically smallest palindromic permutation.
    ///
    /// # Intuition
    /// A palindrome is determined by its first half. Instead of generating all
    /// permutations, directly compute the k-th permutation using multinomial
    /// counting - for each position, count permutations starting with each char.
    ///
    /// # Approach
    /// 1. Count character frequencies and extract first half multiset
    /// 2. For each position, iterate through characters in order and count
    ///    how many permutations would use that character next
    /// 3. Skip characters whose permutation count is less than remaining k
    /// 4. Build palindrome from computed first half
    ///
    /// # Complexity
    /// - Time: O(n × 26) for direct k-th permutation computation
    /// - Space: O(n) for result storage
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let n = s.len();
        let mut freq = [0i64; 26];
        s.bytes().for_each(|b| freq[(b - b'a') as usize] += 1);

        let mut mid_char: Option<u8> = None;
        let mut half_freq = [0i64; 26];

        // Check for valid palindrome structure and compute half frequencies
        let odd_count = freq.iter().filter(|&&f| f % 2 == 1).count();
        if odd_count > 1 {
            return String::new();
        }

        mid_char = freq
            .iter()
            .enumerate()
            .find(|(_, &f)| f % 2 == 1)
            .map(|(i, _)| b'a' + i as u8);

        (0..26).for_each(|i| half_freq[i] = freq[i] / 2);
        let half_len: i64 = half_freq.iter().sum();

        if n % 2 == 0 && mid_char.is_some() {
            return String::new();
        }
        if n % 2 == 1 && mid_char.is_none() {
            return String::new();
        }

        if half_len == 0 {
            return if k == 1 {
                mid_char.map_or(String::new(), |c| String::from(c as char))
            } else {
                String::new()
            };
        }

        let k_max = k as i64;

        // Find k-th permutation directly
        let mut k = k as i64 - 1; // Convert to 0-indexed
        let mut result = Vec::with_capacity(half_len as usize);

        for _ in 0..half_len {
            let mut found = false;

            for c in 0..26 {
                if half_freq[c] == 0 {
                    continue;
                }

                // Count permutations if we place character c at this position
                half_freq[c] -= 1;
                let count = Self::count_perms(&half_freq, k_max);

                if k < count {
                    // This is the right character for this position
                    result.push(b'a' + c as u8);
                    found = true;
                    break;
                } else {
                    // Skip all permutations starting with this character
                    k -= count;
                    half_freq[c] += 1;
                }
            }

            if !found {
                return String::new(); // k was too large
            }
        }

        Self::build_palindrome(&result, mid_char)
    }

    /// Counts permutations of a multiset using multinomial coefficient.
    /// Returns min(actual_count, max_val + 1) to handle large values.
    fn count_perms(freq: &[i64; 26], max_val: i64) -> i64 {
        let n: i64 = freq.iter().sum();
        if n == 0 {
            return 1;
        }

        // Compute multinomial: n! / (f1! * f2! * ... * f26!)
        // Use the formula: product of C(partial_sum, fi)
        let mut result: i64 = 1;
        let mut remaining = n;
        let cap = max_val + 1;

        for &f in freq.iter() {
            if f > 0 {
                // Multiply by C(remaining, f)
                // Use smaller of f and remaining-f to minimize intermediate values
                let k = f.min(remaining - f);
                result = Self::binomial(remaining, k, result, cap);
                if result >= cap {
                    return cap;
                }
                remaining -= f;
            }
        }

        result
    }

    /// Computes base * C(n, k), capped at cap.
    /// Uses the identity C(n,k) = C(n, n-k) and computes with smaller k.
    fn binomial(n: i64, k: i64, base: i64, cap: i64) -> i64 {
        if k == 0 {
            return base;
        }

        let mut result = base;
        for i in 0..k {
            // result = result * (n - i) / (i + 1)
            // To avoid overflow and ensure exact division, factor out GCDs
            let num = n - i;
            let den = i + 1;

            let g = Self::gcd(result, den);
            result /= g;
            let d = den / g;

            let g2 = Self::gcd(num, d);
            let num_reduced = num / g2;
            let d_reduced = d / g2;

            // Check for overflow before multiplication
            if result > cap / num_reduced {
                return cap;
            }
            result *= num_reduced;
            result /= d_reduced;
        }

        result.min(cap)
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn build_palindrome(half: &[u8], mid: Option<u8>) -> String {
        let mut result = half.to_vec();
        if let Some(m) = mid {
            result.push(m);
        }
        result.extend(half.iter().rev());
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::smallest_palindrome("abba".to_string(), 2), "baab");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::smallest_palindrome("aa".to_string(), 2), "");
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::smallest_palindrome("bacab".to_string(), 1),
            "abcba"
        );
    }

    #[test]
    fn test_single_permutation() {
        assert_eq!(Solution::smallest_palindrome("aa".to_string(), 1), "aa");
    }

    #[test]
    fn test_first_permutation() {
        assert_eq!(Solution::smallest_palindrome("abba".to_string(), 1), "abba");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::smallest_palindrome("o".to_string(), 1), "o");
    }

    #[test]
    fn test_multiple_same_chars() {
        assert_eq!(Solution::smallest_palindrome("aabb".to_string(), 1), "abba");
        assert_eq!(Solution::smallest_palindrome("aabb".to_string(), 2), "baab");
    }
}
