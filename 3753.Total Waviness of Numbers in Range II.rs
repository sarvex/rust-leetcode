const MAX_SIZE: usize = 17;

impl Solution {
    /// Total waviness of numbers in range using precomputed DP tables.
    ///
    /// # Intuition
    /// Precompute waviness contributions for all digit positions and values upfront.
    /// This avoids recursion and enables O(D) per query after O(D * 10) preprocessing.
    ///
    /// # Approach
    /// Build two tables:
    /// - unrestricted[len][d]: waviness sum for all len-digit numbers starting with digits 0..=d
    /// - restricted[len][d]: same but excluding leading zeros (valid numbers only)
    ///
    /// For each position, count peaks/valleys by checking if middle digit is greater/less
    /// than both neighbors. Process the input number digit by digit, accumulating
    /// contributions from smaller prefixes.
    ///
    /// # Complexity
    /// - Time: O(D * 10) preprocessing + O(D) per query, where D ~ 15
    /// - Space: O(D * 10) for precomputed tables
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        let (restricted, unrestricted) = Self::build_tables();
        let upper = Self::helper(num2, &restricted, &unrestricted);
        match num1 {
            1 => upper,
            _ => upper - Self::helper(num1 - 1, &restricted, &unrestricted),
        }
    }

    fn build_tables() -> ([[i64; 10]; MAX_SIZE], [[i64; 10]; MAX_SIZE]) {
        let mut restricted = [[0i64; 10]; MAX_SIZE];
        let mut unrestricted = [[0i64; 10]; MAX_SIZE];

        let mut digit_sum = [0i64; 10];
        for d in 0..10 {
            let d_i = d as i64;
            let sum_greater = 45 - (d_i * (d_i + 1)) / 2;
            let sum_smaller = if d == 0 {
                0
            } else {
                9 * d_i - (d_i * (d_i - 1)) / 2
            };
            digit_sum[d] = sum_greater + sum_smaller;
        }

        unrestricted[3][0] = digit_sum[0];
        for d in 1..10 {
            unrestricted[3][d] = unrestricted[3][d - 1] + digit_sum[d];
            restricted[3][d] = restricted[3][d - 1] + digit_sum[d];
        }

        let mut ten: i64 = 10;
        for len in 4..MAX_SIZE {
            let base = unrestricted[len - 1][9];
            unrestricted[len][0] = base + ten * digit_sum[0];
            for d in 1..10 {
                let added = base + ten * digit_sum[d];
                unrestricted[len][d] = unrestricted[len][d - 1] + added;
                restricted[len][d] = restricted[len][d - 1] + added;
            }
            ten *= 10;
        }

        (restricted, unrestricted)
    }

    fn helper(
        n: i64,
        restricted: &[[i64; 10]; MAX_SIZE],
        unrestricted: &[[i64; 10]; MAX_SIZE],
    ) -> i64 {
        if n <= 0 {
            return 0;
        }

        let mut digits = [0i64; 19];
        let mut len = 0usize;
        let mut temp = n;
        while temp > 0 {
            digits[len] = temp % 10;
            len += 1;
            temp /= 10;
        }
        digits[..len].reverse();
        let num = &digits[..len];
        let size = num.len();

        if size < 3 {
            return 0;
        }

        let mut ten = 1i64;
        for _ in 1..size {
            ten *= 10;
        }

        let mut total = restricted[size][num[0] as usize - 1];
        for d in 3..size {
            total += restricted[d][9];
        }

        let mut curr_num = num[0] * ten;
        ten /= 10;

        for i in 1..size - 1 {
            let prev_digit = num[i - 1];
            let curr_digit = num[i];
            let next_digit = num[i + 1];

            curr_num += curr_digit * ten;
            ten /= 10;

            if curr_digit > 0 {
                total += unrestricted[size - i][curr_digit as usize - 1];
            }

            let mut lower_count: i64 = 0;
            for d in 0..curr_digit as usize {
                if d > prev_digit as usize {
                    lower_count += d as i64;
                }
                if d < prev_digit as usize {
                    lower_count += (9 - d) as i64;
                }
            }
            lower_count *= ten;

            let mut equal_count: i64 = 0;
            if curr_digit > prev_digit {
                let stop = curr_digit.min(next_digit);
                equal_count += stop;
                equal_count *= ten;
                if curr_digit > next_digit {
                    let update = curr_num + ten * next_digit;
                    equal_count += n - update + 1;
                }
            }

            if curr_digit < prev_digit && curr_digit < next_digit {
                equal_count += next_digit - curr_digit - 1;
                equal_count *= ten;
                let update = curr_num + ten * next_digit;
                equal_count += n - update + 1;
            }

            total += lower_count + equal_count;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_range_120_130() {
        assert_eq!(Solution::total_waviness(120, 130), 3);
    }

    #[test]
    fn test_range_spanning_200() {
        assert_eq!(Solution::total_waviness(198, 202), 3);
    }

    #[test]
    fn test_single_wavy_number() {
        assert_eq!(Solution::total_waviness(4848, 4848), 2);
    }

    #[test]
    fn test_large_range() {
        assert_eq!(
            Solution::total_waviness(2549294942, 5067104447),
            11661365485
        );
    }
}
