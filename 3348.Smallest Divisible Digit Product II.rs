impl Solution {
    /// Finds the smallest zero-free number >= num with digit product divisible by t.
    ///
    /// # Intuition
    /// The digit product can only contain prime factors 2, 3, 5, 7 (from digits 2-9).
    /// If t has any prime factor other than these, no solution exists. We need to find
    /// the smallest number >= num where the digit product covers all required prime factors.
    ///
    /// # Approach
    /// 1. Factorize t into powers of 2, 3, 5, 7. If t has other factors, return "-1".
    /// 2. If num is already zero-free and its digit product is divisible by t, return num.
    /// 3. Otherwise, try to increment num at each position from right to left:
    ///    - Increment digit at position i to a non-zero value
    ///    - Fill remaining positions with digits that satisfy remaining factor requirements
    /// 4. If no valid number of same length exists, construct a new number with length + 1.
    ///
    /// # Complexity
    /// - Time: O(n * 10) where n is the length of num
    /// - Space: O(n) for storing the result
    pub fn smallest_number(num: String, t: i64) -> String {
        // Factorize t into powers of 2, 3, 5, 7
        let (mut c2, mut c3, mut c5, mut c7) = (0i32, 0i32, 0i32, 0i32);
        let mut rem = t;

        while rem % 2 == 0 {
            rem /= 2;
            c2 += 1;
        }
        while rem % 3 == 0 {
            rem /= 3;
            c3 += 1;
        }
        while rem % 5 == 0 {
            rem /= 5;
            c5 += 1;
        }
        while rem % 7 == 0 {
            rem /= 7;
            c7 += 1;
        }

        // If t has prime factors other than 2, 3, 5, 7, no solution exists
        if rem > 1 {
            return "-1".to_string();
        }

        let digits: Vec<u8> = num.bytes().map(|b| b - b'0').collect();
        let n = digits.len();

        // Precompute factor contribution of each digit (2-9)
        // digit_factors[d] = (count_2, count_3, count_5, count_7)
        let digit_factors: [(i32, i32, i32, i32); 10] = [
            (0, 0, 0, 0), // 0 - invalid
            (0, 0, 0, 0), // 1 - contributes nothing
            (1, 0, 0, 0), // 2 = 2
            (0, 1, 0, 0), // 3 = 3
            (2, 0, 0, 0), // 4 = 2^2
            (0, 0, 1, 0), // 5 = 5
            (1, 1, 0, 0), // 6 = 2 * 3
            (0, 0, 0, 1), // 7 = 7
            (3, 0, 0, 0), // 8 = 2^3
            (0, 2, 0, 0), // 9 = 3^2
        ];

        // Calculate minimum digits needed to provide remaining factors
        // Returns (min_digits, optimal_suffix)
        let build_suffix = |mut r2: i32, mut r3: i32, mut r5: i32, mut r7: i32| -> Vec<u8> {
            let mut suffix = Vec::new();

            // Greedily use 7s for remaining 7 factors
            while r7 > 0 {
                suffix.push(7);
                r7 -= 1;
            }

            // Greedily use 5s for remaining 5 factors
            while r5 > 0 {
                suffix.push(5);
                r5 -= 1;
            }

            // Handle 3s: use 9 (3^2) when we have >= 2 threes remaining
            while r3 >= 2 {
                suffix.push(9);
                r3 -= 2;
            }

            // Handle 2s: use 8 (2^3) when we have >= 3 twos remaining
            while r2 >= 3 {
                suffix.push(8);
                r2 -= 3;
            }

            // Handle remaining 2s and 3s
            if r3 == 1 && r2 >= 1 {
                // Use 6 = 2 * 3
                suffix.push(6);
                r3 -= 1;
                r2 -= 1;
            }

            if r3 == 1 {
                suffix.push(3);
                r3 -= 1;
            }

            // Handle remaining 2s
            if r2 == 2 {
                suffix.push(4);
            } else if r2 == 1 {
                suffix.push(2);
            }

            // Sort to get smallest lexicographic order
            suffix.sort_unstable();
            suffix
        };

        // Try to find valid number by modifying num
        // prefix_factors[i] = accumulated factors from digits[0..i]
        let mut prefix_2 = vec![0i32; n + 1];
        let mut prefix_3 = vec![0i32; n + 1];
        let mut prefix_5 = vec![0i32; n + 1];
        let mut prefix_7 = vec![0i32; n + 1];
        let mut has_zero = vec![false; n + 1];

        for i in 0..n {
            let d = digits[i] as usize;
            prefix_2[i + 1] = prefix_2[i] + digit_factors[d].0;
            prefix_3[i + 1] = prefix_3[i] + digit_factors[d].1;
            prefix_5[i + 1] = prefix_5[i] + digit_factors[d].2;
            prefix_7[i + 1] = prefix_7[i] + digit_factors[d].3;
            has_zero[i + 1] = has_zero[i] || d == 0;
        }

        // Check if original number is already valid
        if !has_zero[n]
            && prefix_2[n] >= c2
            && prefix_3[n] >= c3
            && prefix_5[n] >= c5
            && prefix_7[n] >= c7
        {
            return num;
        }

        // Try incrementing at each position from right to left
        for i in (0..n).rev() {
            let start_digit = if i == 0 && n > 1 {
                (digits[i] + 1).max(1)
            } else {
                digits[i] + 1
            };

            for new_d in start_digit..=9 {
                let new_d_usize = new_d as usize;

                // Calculate remaining factors needed after position i
                let r2 = (c2 - prefix_2[i] - digit_factors[new_d_usize].0).max(0);
                let r3 = (c3 - prefix_3[i] - digit_factors[new_d_usize].1).max(0);
                let r5 = (c5 - prefix_5[i] - digit_factors[new_d_usize].2).max(0);
                let r7 = (c7 - prefix_7[i] - digit_factors[new_d_usize].3).max(0);

                let suffix = build_suffix(r2, r3, r5, r7);
                let remaining_len = n - i - 1;

                if suffix.len() <= remaining_len {
                    // Build result: prefix + new_digit + 1s padding + suffix
                    let mut result: Vec<u8> = digits[..i].to_vec();
                    result.push(new_d);

                    // Fill with 1s
                    for _ in 0..(remaining_len - suffix.len()) {
                        result.push(1);
                    }

                    result.extend(suffix);

                    return result.iter().map(|d| (d + b'0') as char).collect();
                }
            }
        }

        // No valid number with same length, try length + 1
        let suffix = build_suffix(c2, c3, c5, c7);
        if suffix.len() <= n + 1 {
            let mut result = vec![1u8; n + 1 - suffix.len()];
            result.extend(suffix);
            return result.iter().map(|d| (d + b'0') as char).collect();
        }

        "-1".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 256),
            "1488".to_string()
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::smallest_number("12355".to_string(), 50),
            "12355".to_string()
        );
    }

    #[test]
    fn test_example3() {
        // 26 = 2 * 13, and 13 is not achievable with digit products
        assert_eq!(
            Solution::smallest_number("11111".to_string(), 26),
            "-1".to_string()
        );
    }

    #[test]
    fn test_with_zero_in_num() {
        // "102" has a zero, need to find next zero-free number
        assert_eq!(
            Solution::smallest_number("102".to_string(), 2),
            "112".to_string()
        );
    }

    #[test]
    fn test_t_is_one() {
        // t = 1, any zero-free number works
        assert_eq!(
            Solution::smallest_number("123".to_string(), 1),
            "123".to_string()
        );
    }

    #[test]
    fn test_needs_longer_number() {
        // Small number but large t requires more digits
        assert_eq!(
            Solution::smallest_number("11".to_string(), 256),
            "188".to_string()
        );
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(
            Solution::smallest_number("5".to_string(), 5),
            "5".to_string()
        );
    }
}
