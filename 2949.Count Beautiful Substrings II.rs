use std::collections::HashMap;

impl Solution {
    /// Counts substrings with equal vowels and consonants whose product is divisible by `k`.
    ///
    /// # Intuition
    /// A beautiful substring must have the same number of vowels and consonants. If that count is
    /// `v`, then the length is `2v` and the divisibility condition becomes `v^2 % k == 0`, which
    /// means `v` must be a multiple of the smallest integer `m` with `m^2` divisible by `k`.
    ///
    /// # Approach
    /// - Factorize `k` to compute the minimal `m` such that `k | m^2`.
    /// - Let `step = 2m`. Any valid substring has length divisible by `step`.
    /// - Track the prefix balance `vowels - consonants` and the prefix index modulo `step`.
    /// - For each prefix, count prior prefixes with the same balance and modulo; each pair forms a
    ///   beautiful substring.
    ///
    /// # Complexity
    /// - Time: O(n + sqrt(k)), where `n = s.len()`
    /// - Space: O(n)
    pub fn beautiful_substrings(s: String, k: i32) -> i64 {
        let min_vowels = Self::min_square_factor(k);
        let step = (min_vowels * 2) as usize;

        let mut counts: HashMap<(i32, usize), i64> = HashMap::with_capacity(s.len() + 1);
        counts.insert((0, 0), 1);

        let mut balance = 0_i32;
        let mut total = 0_i64;

        for (idx, ch) in s.as_bytes().iter().enumerate() {
            if Self::is_vowel(*ch) {
                balance += 1;
            } else {
                balance -= 1;
            }
            let mod_class = (idx + 1) % step;
            let key = (balance, mod_class);
            if let Some(prev) = counts.get(&key) {
                total += *prev;
            }
            *counts.entry(key).or_insert(0) += 1;
        }

        total
    }

    fn is_vowel(ch: u8) -> bool {
        matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    fn min_square_factor(k: i32) -> i32 {
        let mut remaining = k;
        let mut factor = 1_i32;
        let mut p = 2_i32;

        while p * p <= remaining {
            if remaining % p == 0 {
                let mut exp = 0_i32;
                while remaining % p == 0 {
                    remaining /= p;
                    exp += 1;
                }
                let need = (exp + 1) / 2;
                factor *= p.pow(need as u32);
            }
            p += 1;
        }

        if remaining > 1 {
            factor *= remaining;
        }

        factor
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let s = "baeyh".to_string();
        let k = 2;
        assert_eq!(Solution::beautiful_substrings(s, k), 2);
    }

    #[test]
    fn test_example_2() {
        let s = "abba".to_string();
        let k = 1;
        assert_eq!(Solution::beautiful_substrings(s, k), 3);
    }

    #[test]
    fn test_example_3() {
        let s = "bcdf".to_string();
        let k = 1;
        assert_eq!(Solution::beautiful_substrings(s, k), 0);
    }

    #[test]
    fn test_requires_square_multiple() {
        let s = "ab".to_string();
        let k = 2;
        assert_eq!(Solution::beautiful_substrings(s, k), 0);
    }

    #[test]
    fn test_length_multiple_of_four() {
        let s = "aabb".to_string();
        let k = 2;
        assert_eq!(Solution::beautiful_substrings(s, k), 1);
    }

    #[test]
    fn test_large_factor() {
        let s = "aaaaaabbbbbb".to_string();
        let k = 12;
        assert_eq!(Solution::beautiful_substrings(s, k), 1);
    }
}
