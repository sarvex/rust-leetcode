impl Solution {
    /// Count numbers in [num1, num2] whose digit sum is within bounds.
    ///
    /// # Intuition
    /// We need fast counting across a large numeric range. Digit DP counts how
    /// many numbers up to a bound have a given digit-sum constraint.
    ///
    /// # Approach
    /// 1. Implement `count_up_to(bound)` using digit DP with (position, sum, tight).
    /// 2. Count numbers in [0, bound] whose digit sum is within [min_sum, max_sum].
    /// 3. Answer = count_up_to(num2) - count_up_to(num1 - 1) modulo 1e9+7.
    ///
    /// # Complexity
    /// - Time: O(len * max_sum * 10)
    /// - Space: O(len * max_sum)
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn decrement_decimal(num: &str) -> String {
            let mut bytes = num.as_bytes().to_vec();
            let mut idx = bytes.len();
            while idx > 0 && bytes[idx - 1] == b'0' {
                bytes[idx - 1] = b'9';
                idx -= 1;
            }
            if idx == 0 {
                return "0".to_string();
            }
            bytes[idx - 1] -= 1;
            let mut start = 0usize;
            while start + 1 < bytes.len() && bytes[start] == b'0' {
                start += 1;
            }
            String::from_utf8(bytes[start..].to_vec()).unwrap()
        }

        fn count_up_to(bound: &str, min_sum: usize, max_sum: usize) -> i32 {
            const MOD: i32 = 1_000_000_007;
            let digits: Vec<u8> = bound.bytes().map(|b| b - b'0').collect();
            let max_possible = 9 * digits.len();
            let max_sum = max_sum.min(max_possible);
            if min_sum > max_sum {
                return 0;
            }
            let mut memo = vec![vec![[-1; 2]; max_sum + 1]; digits.len() + 1];

            fn dfs(
                pos: usize,
                sum: usize,
                tight: bool,
                digits: &[u8],
                min_sum: usize,
                max_sum: usize,
                memo: &mut Vec<Vec<[i32; 2]>>,
            ) -> i32 {
                if pos == digits.len() {
                    return if sum >= min_sum { 1 } else { 0 };
                }
                let tight_idx = if tight { 1 } else { 0 };
                if memo[pos][sum][tight_idx] != -1 {
                    return memo[pos][sum][tight_idx];
                }

                let limit = if tight { digits[pos] } else { 9 };
                let mut total: i64 = 0;
                for digit in 0..=limit {
                    let next_sum = sum + digit as usize;
                    if next_sum > max_sum {
                        break;
                    }
                    let next_tight = tight && digit == limit;
                    total += dfs(
                        pos + 1,
                        next_sum,
                        next_tight,
                        digits,
                        min_sum,
                        max_sum,
                        memo,
                    ) as i64;
                    if total >= MOD as i64 {
                        total %= MOD as i64;
                    }
                }
                let res = (total % MOD as i64) as i32;
                memo[pos][sum][tight_idx] = res;
                res
            }

            dfs(0, 0, true, &digits, min_sum, max_sum, &mut memo)
        }

        let min_sum = min_sum.max(0) as usize;
        let max_sum = max_sum.max(0) as usize;
        let lower = decrement_decimal(&num1);
        let mut result =
            count_up_to(&num2, min_sum, max_sum) - count_up_to(&lower, min_sum, max_sum);
        result %= MOD;
        if result < 0 {
            result += MOD;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let result = Solution::count("1".to_string(), "12".to_string(), 1, 8);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_two() {
        let result = Solution::count("1".to_string(), "5".to_string(), 1, 5);
        assert_eq!(result, 5);
    }

    #[test]
    fn single_value_valid() {
        let result = Solution::count("100".to_string(), "100".to_string(), 1, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn impossible_sum_range() {
        let result = Solution::count("1".to_string(), "10".to_string(), 30, 40);
        assert_eq!(result, 0);
    }
}
