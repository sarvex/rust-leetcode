struct Solution;

impl Solution {
    /// Counts integers in [l, r] with non-decreasing digits in base b.
    ///
    /// # Intuition
    /// Use digit DP on base-b representation. Count(r) - Count(l-1) gives
    /// the range answer. Handle large numbers via string subtraction.
    ///
    /// # Approach
    /// 1. Convert decimal string to base-b digits
    /// 2. Apply digit DP with memoization tracking position, last digit, tight bound, and started flag
    /// 3. Use Count(r) - Count(l-1) formula
    ///
    /// # Complexity
    /// - Time: O(n² × b) where n is the number of base-b digits
    /// - Space: O(n² × b) for memoization
    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn subtract_one(s: &str) -> String {
            let mut chars: Vec<char> = s.chars().collect();
            let mut i = chars.len() - 1;
            loop {
                if chars[i] > '0' {
                    chars[i] = (chars[i] as u8 - 1) as char;
                    break;
                }
                chars[i] = '9';
                i -= 1;
            }
            let result: String = chars.into_iter().collect();
            let trimmed = result.trim_start_matches('0');
            if trimmed.is_empty() {
                "0".to_string()
            } else {
                trimmed.to_string()
            }
        }

        fn to_base(s: &str, base: i32) -> Vec<i32> {
            if s == "0" {
                return vec![0];
            }
            let mut digits: Vec<i32> = s.chars().map(|c| (c as i32) - ('0' as i32)).collect();
            let mut result = Vec::new();
            let base = base as i32;

            while !digits.is_empty() && !(digits.len() == 1 && digits[0] == 0) {
                let mut remainder = 0i32;
                let mut new_digits = Vec::new();
                for &d in &digits {
                    let cur = remainder * 10 + d;
                    let quotient = cur / base;
                    remainder = cur % base;
                    if !new_digits.is_empty() || quotient > 0 {
                        new_digits.push(quotient);
                    }
                }
                result.push(remainder);
                digits = new_digits;
            }
            result.reverse();
            if result.is_empty() {
                vec![0]
            } else {
                result
            }
        }

        fn count(digits: &[i32], base: i32, memo: &mut Vec<Vec<Vec<Vec<i64>>>>) -> i64 {
            let n = digits.len();
            if n == 0 {
                return 0;
            }

            fn dp(
                pos: usize,
                last: usize,
                tight: bool,
                started: bool,
                digits: &[i32],
                base: usize,
                memo: &mut Vec<Vec<Vec<Vec<i64>>>>,
            ) -> i64 {
                const MOD: i64 = 1_000_000_007;
                if pos == digits.len() {
                    return if started { 1 } else { 0 };
                }

                let tight_idx = if tight { 1 } else { 0 };
                let started_idx = if started { 1 } else { 0 };

                if memo[pos][last][tight_idx][started_idx] != -1 {
                    return memo[pos][last][tight_idx][started_idx];
                }

                let limit = if tight {
                    digits[pos] as usize
                } else {
                    base - 1
                };
                let mut result = 0i64;

                for d in 0..=limit {
                    if !started {
                        if d == 0 {
                            result +=
                                dp(pos + 1, 0, tight && d == limit, false, digits, base, memo);
                        } else {
                            result += dp(pos + 1, d, tight && d == limit, true, digits, base, memo);
                        }
                    } else if d >= last {
                        result += dp(pos + 1, d, tight && d == limit, true, digits, base, memo);
                    }
                    result %= MOD;
                }

                memo[pos][last][tight_idx][started_idx] = result;
                result
            }

            let base = base as usize;
            dp(0, 0, true, false, digits, base, memo)
        }

        let r_digits = to_base(&r, b);
        let l_minus_1 = subtract_one(&l);
        let l_digits = to_base(&l_minus_1, b);

        let mut memo_r = vec![vec![vec![vec![-1i64; 2]; 2]; b as usize]; r_digits.len()];
        let count_r = count(&r_digits, b, &mut memo_r);

        let count_l = if l_minus_1 == "0" && l != "0" {
            0
        } else {
            let mut memo_l = vec![vec![vec![vec![-1i64; 2]; 2]; b as usize]; l_digits.len()];
            count(&l_digits, b, &mut memo_l)
        };

        ((count_r - count_l % MOD + MOD) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
