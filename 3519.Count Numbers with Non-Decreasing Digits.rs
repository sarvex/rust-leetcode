impl Solution {
    /// Counts integers in [l, r] with non-decreasing digits in base b.
    ///
    /// # Intuition
    /// Use digit DP on base-b representation. Count(r) - Count(l-1) gives
    /// the range answer. Precompute counts for non-tight states.
    ///
    /// # Approach
    /// 1. Convert decimal string to base-b digits via repeated division
    /// 2. Precompute dp[len][last_digit] = count of len-digit non-decreasing numbers starting with last_digit
    /// 3. For tight bound, iterate digits and accumulate valid counts
    ///
    /// # Complexity
    /// - Time: O(n × b²) where n is the number of base-b digits
    /// - Space: O(n × b)
    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let b = b as usize;

        fn subtract_one(s: &[u8]) -> Vec<u8> {
            let mut result: Vec<u8> = s.to_vec();
            let mut i = result.len() - 1;
            while result[i] == 0 {
                result[i] = 9;
                i -= 1;
            }
            result[i] -= 1;
            let start = result
                .iter()
                .position(|&x| x != 0)
                .unwrap_or(result.len() - 1);
            result[start..].to_vec()
        }

        fn to_base(s: &[u8], base: usize) -> Vec<usize> {
            let mut digits = s.to_vec();
            let mut result = Vec::new();
            while !digits.is_empty() {
                let mut rem = 0u32;
                let mut next = Vec::new();
                for &d in &digits {
                    let cur = rem * 10 + d as u32;
                    let q = cur / base as u32;
                    rem = cur % base as u32;
                    if !next.is_empty() || q > 0 {
                        next.push(q as u8);
                    }
                }
                result.push(rem as usize);
                digits = next;
            }
            result.reverse();
            if result.is_empty() { vec![0] } else { result }
        }

        // dp[i][d] = count of i-digit non-decreasing numbers where first digit is d
        fn build_dp(max_len: usize, base: usize) -> Vec<Vec<u64>> {
            let mut dp = vec![vec![0u64; base + 1]; max_len + 1];
            (0..=base).for_each(|d| dp[0][d] = 1);
            for len in 1..=max_len {
                dp[len][base] = 0;
                for d in (0..base).rev() {
                    dp[len][d] = (dp[len][d + 1] + dp[len - 1][d]) % MOD;
                }
            }
            dp
        }

        // Count non-decreasing numbers <= digits (in base b)
        fn count(digits: &[usize], base: usize, dp: &[Vec<u64>]) -> u64 {
            let n = digits.len();
            if n == 0 {
                return 0;
            }
            let mut result = 0u64;

            // Count numbers with fewer digits (1 to n-1 digits)
            result = (1..n)
                .flat_map(|len| (1..base).map(move |d| dp[len - 1][d]))
                .fold(result, |acc, x| (acc + x) % MOD);

            // Count n-digit numbers <= digits
            let mut last = 0;
            for (i, &d) in digits.iter().enumerate() {
                let remaining = n - 1 - i;
                let start = if i == 0 { 1.max(last) } else { last };
                for digit in start..d {
                    result = (result + dp[remaining][digit]) % MOD;
                }
                if d < last {
                    return result;
                }
                last = d;
            }
            result + 1 // Include the number itself
        }

        let l_bytes: Vec<u8> = l.bytes().map(|c| c - b'0').collect();
        let r_bytes: Vec<u8> = r.bytes().map(|c| c - b'0').collect();

        let r_digits = to_base(&r_bytes, b);
        let l_minus_1 = subtract_one(&l_bytes);
        let l_digits = to_base(&l_minus_1, b);

        let max_len = r_digits.len();
        let dp = build_dp(max_len, b);

        let count_r = count(&r_digits, b, &dp);
        let count_l = if l_minus_1.is_empty() || (l_minus_1.len() == 1 && l_minus_1[0] == 0) {
            0
        } else {
            count(&l_digits, b, &dp)
        };

        ((count_r + MOD - count_l) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::count_numbers("23".to_string(), "28".to_string(), 8),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::count_numbers("2".to_string(), "7".to_string(), 2),
            2
        );
    }

    #[test]
    fn test_single_number() {
        assert_eq!(
            Solution::count_numbers("1".to_string(), "1".to_string(), 10),
            1
        );
    }

    #[test]
    fn test_base_10() {
        assert_eq!(
            Solution::count_numbers("1".to_string(), "100".to_string(), 10),
            54
        );
    }

    #[test]
    fn test_large_range() {
        assert_eq!(
            Solution::count_numbers("1".to_string(), "1000".to_string(), 10),
            219
        );
    }
}
