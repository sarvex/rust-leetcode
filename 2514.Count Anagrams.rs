const MOD: i64 = 1_000_000_007;

impl Solution {
    /// Calculates distinct anagrams where each word permutes independently.
    ///
    /// # Intuition
    /// Each word contributes permutations independently. For a word of length n with character
    /// frequencies c1, c2, ..., ck, the distinct permutations are n! / (c1! × c2! × ... × ck!).
    /// The total is the product of permutations across all words.
    ///
    /// # Approach
    /// 1. Precompute factorials and inverse factorials up to max string length
    /// 2. For each word, compute permutations using the multinomial coefficient formula
    /// 3. Multiply results across all words using modular arithmetic
    /// 4. Use Fermat's little theorem for modular inverse: a^(-1) ≡ a^(p-2) (mod p)
    ///
    /// # Complexity
    /// - Time: O(n) where n is the string length
    /// - Space: O(n) for factorial precomputation
    pub fn count_anagrams(s: String) -> i32 {
        let n = s.len();
        let (fact, inv_fact) = Self::precompute_factorials(n);

        let result = s.split(' ').fold(1i64, |result, word| {
            let mut char_count = [0usize; 26];
            for b in word.bytes() {
                char_count[(b - b'a') as usize] += 1;
            }

            let permutations = char_count
                .iter()
                .filter(|&&c| c > 1)
                .fold(fact[word.len()], |perm, &count| {
                    (perm * inv_fact[count]) % MOD
                });

            (result * permutations) % MOD
        });

        result as i32
    }

    fn precompute_factorials(n: usize) -> (Vec<i64>, Vec<i64>) {
        let mut fact = vec![1i64; n + 1];
        let mut inv_fact = vec![1i64; n + 1];

        for i in 1..=n {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        inv_fact[n] = Self::mod_pow(fact[n], MOD - 2);
        for i in (1..n).rev() {
            inv_fact[i] = (inv_fact[i + 1] * (i + 1) as i64) % MOD;
        }

        (fact, inv_fact)
    }

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut result = 1i64;
        base %= MOD;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % MOD;
            }
            exp >>= 1;
            base = (base * base) % MOD;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_anagrams("too hot".to_string()), 18);
    }

    #[test]
    fn test_single_repeated_char() {
        assert_eq!(Solution::count_anagrams("aa".to_string()), 1);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::count_anagrams("a".to_string()), 1);
    }

    #[test]
    fn test_unique_chars() {
        assert_eq!(Solution::count_anagrams("abc".to_string()), 6);
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(Solution::count_anagrams("ab cd".to_string()), 4);
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(Solution::count_anagrams("aaa".to_string()), 1);
    }

    #[test]
    fn test_mixed_frequencies() {
        assert_eq!(Solution::count_anagrams("aab".to_string()), 3);
    }
}
