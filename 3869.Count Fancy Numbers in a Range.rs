impl Solution {
    /// Counts fancy numbers in [l, r] via double-bounded digit DP on digit sums
    /// plus BFS enumeration of self-good numbers whose digit sum is not good.
    ///
    /// # Intuition
    /// A number is fancy if it is "good" (strictly monotone digits) or its digit
    /// sum is good. Self-good numbers are finite (~1,524 up to 10^15) since
    /// strictly monotone sequences use distinct digits from 0–9. We enumerate
    /// them via BFS expansion, then count digit-sum-good numbers with a compact
    /// DP that tracks both lower and upper tight constraints simultaneously.
    ///
    /// # Approach
    /// 1. Parse `l` and `r` into digit arrays (reversed, zero-padded to equal length).
    /// 2. Build `vincdec[s]`: whether sum `s` has strictly monotone digits.
    /// 3. BFS-expand all strictly increasing and decreasing numbers; for those in
    ///    [l, r] whose digit sum is NOT good, add 1 to the answer.
    /// 4. Double-bounded digit DP with state `dp[sum][tight]` where `tight` encodes
    ///    two bits: bit 0 = tight to `l`, bit 1 = tight to `r`. Transition scans
    ///    digit range [low, high] per tight state. After processing all positions,
    ///    sum over good digit sums.
    ///
    /// # Complexity
    /// - Time: O(D × S × 10 + 2^10) where D ≤ 16, S ≤ 144
    /// - Space: O(S + 2^10)
    pub fn count_fancy(l: i64, r: i64) -> i64 {
        let rs = r
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .collect::<Vec<_>>();
        let mut ls = l
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .collect::<Vec<_>>();
        while ls.len() < rs.len() {
            ls.push(0);
        }
        let max_sum = 9 * rs.len();
        let mut vincdec = vec![false; max_sum + 1];
        let mut ans = 0;
        for i in 1..=max_sum {
            if i < 10 {
                vincdec[i] = true;
            } else if i < 100 {
                if i % 10 != i / 10 {
                    vincdec[i] = true;
                }
            } else {
                let v1 = i / 100;
                let v2 = (i / 10) % 10;
                let v3 = i % 10;
                if (v1 < v2 && v2 < v3) || (v1 > v2 && v2 > v3) {
                    vincdec[i] = true;
                }
            }
        }
        let mut q = (1..10).collect::<Vec<_>>();
        let mut i = 0;
        while i < q.len() {
            let d = q[i];
            let ld = d % 10;
            for j in ld + 1..10 {
                q.push(d * 10 + j);
            }
            i += 1;
        }
        let mut qincdec = q;
        let mut q = (1..10).rev().collect::<Vec<_>>();
        let mut i = 0;
        while i < q.len() {
            let d = q[i];
            let ld = d % 10;
            for j in 0..ld {
                q.push(d * 10 + j);
            }
            i += 1;
        }
        qincdec.extend(q[9..].iter());
        for mut q in qincdec {
            if q >= l && q <= r {
                let mut s = 0;
                while q > 0 {
                    s += q % 10;
                    q /= 10;
                }
                if !vincdec[s as usize] {
                    ans += 1;
                }
            }
        }
        let mut dp = vec![[0, 0, 0, 0]; max_sum + 1];
        dp[0][0] = 1;
        for i in 0..rs.len() {
            let mut ndp = vec![[0, 0, 0, 0]; max_sum + 1];
            for j in 0..=max_sum {
                for m in 0..4 {
                    if dp[j][m] == 0 {
                        continue;
                    }
                    let low = if m & 1 == 0 { ls[ls.len() - i - 1] } else { 0 };
                    let high = if m & 2 == 0 { rs[rs.len() - i - 1] } else { 9 };
                    for k in low..=high {
                        let mut nt = 3;
                        if m & 1 == 0 && k == ls[ls.len() - i - 1] {
                            nt ^= 1;
                        }
                        if m & 2 == 0 && k == rs[rs.len() - i - 1] {
                            nt ^= 2;
                        }
                        ndp[j + k as usize][nt] += dp[j][m];
                    }
                }
            }
            dp = ndp;
        }
        for i in 0..=max_sum {
            if vincdec[i] {
                ans += dp[i][0] + dp[i][1] + dp[i][2] + dp[i][3];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::count_fancy(8, 10), 3);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::count_fancy(12340, 12341), 1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::count_fancy(123456788, 123456788), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::count_fancy(1, 9), 9);
    }

    #[test]
    fn test_range_1_to_1() {
        assert_eq!(Solution::count_fancy(1, 1), 1);
    }

    #[test]
    fn test_two_digit_numbers() {
        assert_eq!(Solution::count_fancy(10, 12), 3);
    }

    #[test]
    fn test_number_with_good_digit_sum() {
        assert_eq!(Solution::count_fancy(100, 100), 1);
    }

    #[test]
    fn test_large_range() {
        let result = Solution::count_fancy(1, 1_000_000_000_000_000);
        assert!(result > 0);
    }
}
