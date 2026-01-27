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
        match num1 {
            1 => Self::helper(num2),
            _ => Self::helper(num2) - Self::helper(num1 - 1),
        }
    }

    fn helper(n: i64) -> i64 {
        if n <= 0 {
            return 0;
        }

        const MAX_SIZE: usize = 17;
        let mut restricted = vec![vec![0i64; 10]; MAX_SIZE];
        let mut unrestricted = vec![vec![0i64; 10]; MAX_SIZE];

        unrestricted[3][0] = 45;

        for d in 1..10 {
            unrestricted[3][d] = unrestricted[3][d - 1];
            restricted[3][d] = restricted[3][d - 1];
            for j in d + 1..10 {
                unrestricted[3][d] += j as i64;
                restricted[3][d] += j as i64;
            }
            for j in 0..d {
                unrestricted[3][d] += (9 - j) as i64;
                restricted[3][d] += (9 - j) as i64;
            }
        }

        let mut ten: i64 = 10;
        for len in 4..MAX_SIZE {
            for d in 0..10 {
                let mut unrestricted_count: i64 = 0;
                let mut restricted_count: i64 = 0;

                match d {
                    0 => {
                        unrestricted[len][d] = unrestricted[len - 1][9];
                        for j in 1..10 {
                            unrestricted_count += j as i64;
                        }
                    }
                    _ => {
                        unrestricted[len][d] = unrestricted[len][d - 1] + unrestricted[len - 1][9];
                        restricted[len][d] = restricted[len][d - 1] + unrestricted[len - 1][9];
                        for j in d + 1..10 {
                            unrestricted_count += j as i64;
                            restricted_count += j as i64;
                        }
                        for j in 0..d {
                            unrestricted_count += (9 - j) as i64;
                            restricted_count += (9 - j) as i64;
                        }
                    }
                }

                unrestricted[len][d] += ten * unrestricted_count;
                restricted[len][d] += ten * restricted_count;
            }
            ten *= 10;
        }

        let mut num = Vec::with_capacity(16);
        let mut temp = n;
        while temp > 0 {
            num.push(temp % 10);
            temp /= 10;
        }
        num.reverse();
        let size = num.len();

        if size < 3 {
            return 0;
        }

        ten = 10_i64.pow(size as u32 - 1);

        let mut total = restricted[size][num[0] as usize - 1];
        for d in 3..size {
            total += restricted[d][9];
        }

        let mut curr_num = num[0] * ten;
        ten /= 10;

        for i in 1..size - 1 {
            let curr_digit = num[i];
            curr_num += curr_digit * ten;
            ten /= 10;

            if curr_digit > 0 {
                total += unrestricted[size - i][curr_digit as usize - 1];
            }

            let mut lower_count: i64 = 0;
            for d in 0..curr_digit as usize {
                if d > num[i - 1] as usize {
                    lower_count += d as i64;
                }
                if d < num[i - 1] as usize {
                    lower_count += (9 - d) as i64;
                }
            }
            lower_count *= ten;

            let mut equal_count: i64 = 0;
            if curr_digit > num[i - 1] {
                let stop = curr_digit.min(num[i + 1]);
                equal_count += stop;
                equal_count *= ten;
                if curr_digit > num[i + 1] {
                    let update = curr_num + ten * num[i + 1];
                    equal_count += n - update + 1;
                }
            }

            if curr_digit < num[i - 1] && curr_digit < num[i + 1] {
                equal_count += num[i + 1] - curr_digit - 1;
                equal_count *= ten;
                let update = curr_num + ten * num[i + 1];
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
