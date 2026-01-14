const MOD: i64 = 1_000_000_007;

impl Solution {
    /// Count Anagrams - calculates distinct anagrams where each word permutes independently
    ///
    /// # Intuition
    /// Each word contributes permutations independently. For a word of length n with character
    /// frequencies c1, c2, ..., ck, the distinct permutations are n! / (c1! * c2! * ... * ck!).
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

        let mut result: i64 = 1;

        for word in s.split(' ') {
            let word_len = word.len();
            let mut char_count = [0usize; 26];

            for ch in word.bytes() {
                char_count[(ch - b'a') as usize] += 1;
            }

            // Numerator: word_len!
            let mut permutations = fact[word_len];

            // Denominator: product of char_count[i]! for each character
            for &count in &char_count {
                if count > 1 {
                    permutations = (permutations * inv_fact[count]) % MOD;
                }
            }

            result = (result * permutations) % MOD;
        }

        result as i32
    }

    /// Precomputes factorials and their modular inverses up to n
    fn precompute_factorials(n: usize) -> (Vec<i64>, Vec<i64>) {
        let mut fact = vec![1i64; n + 1];
        let mut inv_fact = vec![1i64; n + 1];

        for i in 1..=n {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        // Compute inverse of n! using Fermat's little theorem
        inv_fact[n] = Self::mod_pow(fact[n], MOD - 2);

        // Compute remaining inverse factorials: (i-1)!^(-1) = i!^(-1) * i
        for i in (1..n).rev() {
            inv_fact[i] = (inv_fact[i + 1] * (i + 1) as i64) % MOD;
        }

        (fact, inv_fact)
    }

    /// Fast modular exponentiation: base^exp mod MOD
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

    struct Solution;

    #[test]
    fn test_example_1() {
        let s = "too hot".to_string();
        assert_eq!(Solution::count_anagrams(s), 18);
    }

    #[test]
    fn test_example_2() {
        let s = "aa".to_string();
        assert_eq!(Solution::count_anagrams(s), 1);
    }

    #[test]
    fn test_single_char() {
        let s = "a".to_string();
        assert_eq!(Solution::count_anagrams(s), 1);
    }

    #[test]
    fn test_unique_chars() {
        let s = "abc".to_string();
        // 3! = 6 permutations
        assert_eq!(Solution::count_anagrams(s), 6);
    }

    #[test]
    fn test_multiple_words_unique() {
        let s = "ab cd".to_string();
        // 2! * 2! = 4
        assert_eq!(Solution::count_anagrams(s), 4);
    }

    #[test]
    fn test_all_same_chars() {
        let s = "aaa".to_string();
        // 3! / 3! = 1
        assert_eq!(Solution::count_anagrams(s), 1);
    }

    #[test]
    fn test_mixed_frequencies() {
        let s = "aab".to_string();
        // 3! / 2! = 3
        assert_eq!(Solution::count_anagrams(s), 3);
    }
}
