impl Solution {
    /// Counts the number of length-`k` rotations that turn `s` into `t`.
    ///
    /// # Intuition
    /// Each operation rotates the string by a non-zero amount, so after `k` steps the result is
    /// determined solely by the net shift. All non-zero shifts are symmetric.
    ///
    /// # Approach
    /// - Find every rotation offset where `s` equals `t` by running KMP on `s + s`.
    /// - Track two counts: ways to end at shift `0` (`same_count`) and at any specific non-zero
    ///   shift (`other_count`).
    /// - These counts follow a 2-state recurrence, solved with 2x2 matrix exponentiation.
    /// - Sum `same_count` for the zero shift and `other_count` for each non-zero matching shift.
    ///
    /// # Complexity
    /// - Time: O(n + log k)
    /// - Space: O(n)
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = s.len();
        if n == 0 {
            return 0;
        }

        let (match_zero, match_non_zero) = Self::rotation_match_counts(&s, &t);
        let (same_count, other_count) = Self::shift_counts(n as i64, k);
        let total = (match_zero * same_count + match_non_zero * other_count) % MOD;

        total as i32
    }

    fn rotation_match_counts(s: &str, t: &str) -> (i64, i64) {
        let n = s.len();
        if n == 0 {
            return (0, 0);
        }

        let pattern = t.as_bytes();
        let prefix = Self::prefix_function(pattern);

        let mut doubled = Vec::with_capacity(n * 2);
        doubled.extend_from_slice(s.as_bytes());
        doubled.extend_from_slice(s.as_bytes());

        let mut matched = 0usize;
        let mut match_zero = 0i64;
        let mut match_non_zero = 0i64;

        for (idx, &byte) in doubled.iter().enumerate() {
            while matched > 0 && byte != pattern[matched] {
                matched = prefix[matched - 1];
            }
            if byte == pattern[matched] {
                matched += 1;
            }
            if matched == n {
                let start = idx + 1 - n;
                if start < n {
                    if start == 0 {
                        match_zero += 1;
                    } else {
                        match_non_zero += 1;
                    }
                }
                matched = prefix[matched - 1];
            }
        }

        (match_zero, match_non_zero)
    }

    fn prefix_function(pattern: &[u8]) -> Vec<usize> {
        let n = pattern.len();
        let mut prefix = vec![0usize; n];
        let mut j = 0usize;

        for i in 1..n {
            while j > 0 && pattern[i] != pattern[j] {
                j = prefix[j - 1];
            }
            if pattern[i] == pattern[j] {
                j += 1;
            }
            prefix[i] = j;
        }

        prefix
    }

    fn shift_counts(n: i64, k: i64) -> (i64, i64) {
        const MOD: i64 = 1_000_000_007;

        let base = [[0_i64, (n - 1) % MOD], [1_i64, (n - 2).rem_euclid(MOD)]];
        let power = Self::matrix_pow(base, k);

        (power[0][0], power[1][0])
    }

    fn matrix_pow(mut base: [[i64; 2]; 2], mut exp: i64) -> [[i64; 2]; 2] {
        let mut result = [[1_i64, 0_i64], [0_i64, 1_i64]];

        while exp > 0 {
            if exp & 1 == 1 {
                result = Self::matrix_mul(result, base);
            }
            base = Self::matrix_mul(base, base);
            exp >>= 1;
        }

        result
    }

    fn matrix_mul(a: [[i64; 2]; 2], b: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
        const MOD: i64 = 1_000_000_007;
        let mut result = [[0_i64; 2]; 2];

        for i in 0..2 {
            for j in 0..2 {
                let sum = a[i][0] as i128 * b[0][j] as i128 + a[i][1] as i128 * b[1][j] as i128;
                result[i][j] = (sum % MOD as i128) as i64;
            }
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
            Solution::number_of_ways("abcd".to_string(), "cdab".to_string(), 2),
            2
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::number_of_ways("ababab".to_string(), "ababab".to_string(), 1),
            2
        );
    }

    #[test]
    fn test_all_rotations_match() {
        assert_eq!(
            Solution::number_of_ways("aaaa".to_string(), "aaaa".to_string(), 2),
            9
        );
    }

    #[test]
    fn test_no_rotation_match() {
        assert_eq!(
            Solution::number_of_ways("abcd".to_string(), "abdc".to_string(), 5),
            0
        );
    }

    #[test]
    fn test_two_letters() {
        assert_eq!(
            Solution::number_of_ways("ab".to_string(), "ba".to_string(), 1),
            1
        );
    }
}
