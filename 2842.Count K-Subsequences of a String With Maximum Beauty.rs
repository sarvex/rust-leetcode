impl Solution {
    /// Counts maximum-beauty k-subsequences by selecting the highest frequencies.
    ///
    /// # Intuition
    /// The beauty is the sum of chosen character frequencies, so the maximum beauty always uses
    /// the k largest frequencies. Ties at the cutoff create multiple optimal character sets.
    ///
    /// # Approach
    /// - Count each letter frequency and sort descending.
    /// - If there are fewer than k distinct letters, return 0.
    /// - Let `threshold` be the k-th largest frequency.
    /// - All letters with frequency greater than `threshold` are mandatory; choose the remaining
    ///   `need = k - higher` letters from those equal to `threshold`.
    /// - Each chosen letter contributes `f(c)` index choices, so the total is:
    ///   `product(highers) * C(equal, need) * threshold^need (mod 1e9+7)`.
    ///
    /// # Complexity
    /// - Time: O(n + A log A), where A <= 26.
    /// - Space: O(A).
    pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let k = k as usize;
        let mut counts = [0_i64; 26];
        for &byte in s.as_bytes() {
            counts[(byte - b'a') as usize] += 1;
        }

        let mut freqs: Vec<i64> = counts.iter().copied().filter(|count| *count > 0).collect();
        if k > freqs.len() {
            return 0;
        }

        freqs.sort_unstable_by(|a, b| b.cmp(a));
        let threshold = freqs[k - 1];
        let higher = freqs.iter().take_while(|count| **count > threshold).count();
        let equal = freqs.iter().filter(|count| **count == threshold).count();
        let need = k - higher;

        let product_highers = freqs
            .iter()
            .take(higher)
            .fold(1_i64, |acc, count| (acc * (*count % MOD)) % MOD);

        let ways_equal = Self::n_choose_k(equal, need);
        let pow_equal = Self::mod_pow(threshold % MOD, need as i64);
        let result = (((product_highers * ways_equal) % MOD) * pow_equal) % MOD;

        result as i32
    }

    fn n_choose_k(n: usize, k: usize) -> i64 {
        const MOD: i64 = 1_000_000_007;
        if k > n {
            return 0;
        }

        let mut fact = vec![1_i64; n + 1];
        for i in 1..=n {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        let mut inv_fact = vec![1_i64; n + 1];
        inv_fact[n] = Self::mod_pow(fact[n], MOD - 2);
        for i in (1..=n).rev() {
            inv_fact[i - 1] = (inv_fact[i] * i as i64) % MOD;
        }

        (((fact[n] * inv_fact[k]) % MOD) * inv_fact[n - k]) % MOD
    }

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let mut result = 1_i64;
        base %= MOD;

        while exp > 0 {
            if exp & 1 == 1 {
                result = ((result as i128 * base as i128) % MOD as i128) as i64;
            }
            base = ((base as i128 * base as i128) % MOD as i128) as i64;
            exp >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("bcca".to_string(), 2),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("abbcd".to_string(), 4),
            2
        );
    }

    #[test]
    fn test_not_enough_distinct() {
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("aa".to_string(), 2),
            0
        );
    }

    #[test]
    fn test_all_distinct() {
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("abcd".to_string(), 4),
            1
        );
    }

    #[test]
    fn test_ties_for_cutoff() {
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("aabbccdd".to_string(), 3),
            32
        );
    }

    #[test]
    fn test_single_char_max_frequency() {
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("aaabbbccc".to_string(), 1),
            9
        );
    }
}
