impl Solution {
    /// Build the lexicographically largest n-digit palindrome divisible by `k`.
    ///
    /// # Intuition
    /// A palindrome is determined by its left half (and middle digit for odd length), and each
    /// chosen digit contributes a fixed modular weight to the whole number.
    ///
    /// # Approach
    /// - Precompute `10^i mod k` to derive each position's weight in the palindrome.
    /// - Run a suffix DP over positions and remainders to know which remainders are achievable
    ///   with the remaining digits (respecting the no-leading-zero rule).
    /// - Greedily pick the largest digit at each position that can still reach remainder `0`.
    /// - Mirror the left half to build the full palindrome string.
    ///
    /// # Complexity
    /// - Time: O(n * k * 10)
    /// - Space: O(n * k)
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;

        if n == 0 {
            return String::new();
        }

        let half = n / 2;
        let choose_len = half + (n % 2);

        let mut pow10 = vec![0_usize; n];
        pow10[0] = 1 % k;
        for i in 1..n {
            pow10[i] = (pow10[i - 1] * 10) % k;
        }

        let mut weights = Vec::with_capacity(choose_len);
        for i in 0..choose_len {
            let weight = if n % 2 == 1 && i == half {
                pow10[i] % k
            } else {
                (pow10[i] + pow10[n - 1 - i]) % k
            };
            weights.push(weight);
        }

        let mut dp = vec![vec![false; k]; choose_len + 1];
        dp[choose_len][0] = true;

        for pos in (0..choose_len).rev() {
            let min_digit = if pos == 0 { 1 } else { 0 };
            for rem in 0..k {
                let mut ok = false;
                for d in (min_digit..=9).rev() {
                    let contrib = (d * weights[pos]) % k;
                    let next_rem = (rem + k - contrib) % k;
                    if dp[pos + 1][next_rem] {
                        ok = true;
                        break;
                    }
                }
                dp[pos][rem] = ok;
            }
        }

        let mut left_digits: Vec<u8> = Vec::with_capacity(choose_len);
        let mut prefix_rem = 0_usize;

        for pos in 0..choose_len {
            let min_digit = if pos == 0 { 1 } else { 0 };
            let mut chosen = None;
            for d in (min_digit..=9).rev() {
                let contrib = (d * weights[pos]) % k;
                let new_prefix = (prefix_rem + contrib) % k;
                let target = if new_prefix == 0 { 0 } else { k - new_prefix };
                if dp[pos + 1][target] {
                    chosen = Some(d);
                    prefix_rem = new_prefix;
                    left_digits.push(d as u8);
                    break;
                }
            }
            if chosen.is_none() {
                return String::new();
            }
        }

        let mut bytes = Vec::with_capacity(n);
        for &d in &left_digits[..half] {
            bytes.push(b'0' + d);
        }
        if n % 2 == 1 {
            bytes.push(b'0' + left_digits[half]);
        }
        for &d in left_digits[..half].iter().rev() {
            bytes.push(b'0' + d);
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::largest_palindrome(3, 5), "595");
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::largest_palindrome(1, 4), "8");
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::largest_palindrome(5, 6), "89898");
    }

    #[test]
    fn k_one_all_nines() {
        assert_eq!(Solution::largest_palindrome(4, 1), "9999");
    }

    #[test]
    fn even_length_k_five() {
        assert_eq!(Solution::largest_palindrome(2, 5), "55");
    }
}
