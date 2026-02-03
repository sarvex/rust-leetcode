impl Solution {
    /// Digit DP over prefixes with tight and previous digit state.
    ///
    /// # Intuition
    /// A stepping number is fully determined by its digit sequence where each adjacent pair differs
    /// by exactly 1. We can count valid sequences up to a bound by building digits left to right.
    ///
    /// # Approach
    /// Compute `count_up_to(bound)` with a digit DP that tracks the position, previous digit,
    /// whether we are still tight to the bound, and whether we have started the number. Leading
    /// zeros are allowed before the first non-zero digit, which enables counting all lengths up to
    /// the bound. The final answer is `count_up_to(high) - count_up_to(low - 1)` modulo `1e9 + 7`.
    ///
    /// # Complexity
    /// - Time: O(L * 10)
    /// - Space: O(L * 11 * 2 * 2)
    pub fn count_stepping_numbers(low: String, high: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let high_count = Self::count_up_to(&high);
        let low_minus_one = Self::decrement_string(&low);
        let low_count = Self::count_up_to(&low_minus_one);

        let mut result = high_count - low_count;
        if result < 0 {
            result += MOD;
        }
        (result % MOD) as i32
    }

    fn count_up_to(bound: &str) -> i64 {
        const MOD: i64 = 1_000_000_007;

        let digits: Vec<u8> = bound.bytes().map(|byte| byte - b'0').collect();
        let mut memo = vec![vec![vec![vec![None; 2]; 2]; 11]; digits.len() + 1];
        Self::dfs(0, 10, true, false, &digits, &mut memo, MOD)
    }

    fn dfs(
        pos: usize,
        prev_digit: usize,
        tight: bool,
        started: bool,
        digits: &[u8],
        memo: &mut Vec<Vec<Vec<Vec<Option<i64>>>>>,
        modulo: i64,
    ) -> i64 {
        let tight_idx = usize::from(tight);
        let started_idx = usize::from(started);
        if let Some(value) = memo[pos][prev_digit][tight_idx][started_idx] {
            return value;
        }

        if pos == digits.len() {
            let value = if started { 1 } else { 0 };
            memo[pos][prev_digit][tight_idx][started_idx] = Some(value);
            return value;
        }

        let limit = if tight { digits[pos] } else { 9 };
        let mut total = 0i64;

        for digit in 0..=limit {
            let next_tight = tight && digit == limit;
            let digit_usize = digit as usize;

            if !started {
                if digit == 0 {
                    let add = Self::dfs(pos + 1, 10, next_tight, false, digits, memo, modulo);
                    total += add;
                } else {
                    let add =
                        Self::dfs(pos + 1, digit_usize, next_tight, true, digits, memo, modulo);
                    total += add;
                }
            } else if (digit as i32 - prev_digit as i32).abs() == 1 {
                let add = Self::dfs(pos + 1, digit_usize, next_tight, true, digits, memo, modulo);
                total += add;
            }

            if total >= modulo {
                total %= modulo;
            }
        }

        total %= modulo;
        memo[pos][prev_digit][tight_idx][started_idx] = Some(total);
        total
    }

    fn decrement_string(value: &str) -> String {
        if value == "0" {
            return "0".to_string();
        }

        let mut digits: Vec<u8> = value.bytes().map(|byte| byte - b'0').collect();
        let mut idx = digits.len();
        while idx > 0 {
            idx -= 1;
            if digits[idx] > 0 {
                digits[idx] -= 1;
                break;
            }
            digits[idx] = 9;
        }

        let first_non_zero = digits.iter().position(|&digit| digit != 0);
        match first_non_zero {
            Some(start) => digits[start..]
                .iter()
                .map(|digit| (digit + b'0') as char)
                .collect(),
            None => "0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::count_stepping_numbers("1".to_string(), "11".to_string()),
            10
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::count_stepping_numbers("90".to_string(), "101".to_string()),
            2
        );
    }

    #[test]
    fn single_value_stepping() {
        assert_eq!(
            Solution::count_stepping_numbers("10".to_string(), "10".to_string()),
            1
        );
    }

    #[test]
    fn single_value_not_stepping() {
        assert_eq!(
            Solution::count_stepping_numbers("100".to_string(), "100".to_string()),
            0
        );
    }
}
