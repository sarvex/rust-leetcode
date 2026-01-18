use std::sync::OnceLock;

const OFF: usize = 72;
const SZ: usize = 145;

struct Tables {
    w: [[[i64; SZ]; 17]; 2],
    f: [i64; 17],
}

static TABLES: OnceLock<Tables> = OnceLock::new();

fn get_tables() -> &'static Tables {
    TABLES.get_or_init(|| {
        let mut w = [[[0i64; SZ]; 17]; 2];
        w[0][0][OFF] = 1;
        w[1][0][OFF] = 1;

        for r in 0..16 {
            for i in 0..SZ {
                for d in 0..10i32 {
                    if w[0][r][i] > 0 {
                        let delta = if (r + 1) & 1 == 1 { d } else { -d };
                        let j = i as i32 + delta;
                        if (0..SZ as i32).contains(&j) {
                            w[0][r + 1][j as usize] += w[0][r][i];
                        }
                    }
                    if w[1][r][i] > 0 {
                        let delta = if (r + 1) & 1 == 0 { d } else { -d };
                        let j = i as i32 + delta;
                        if (0..SZ as i32).contains(&j) {
                            w[1][r + 1][j as usize] += w[1][r][i];
                        }
                    }
                }
            }
        }

        let mut f = [0i64; 17];
        for len in 2..=16 {
            let mut dp = [0i64; SZ];
            dp[OFF] = 1;
            for p in 0..len {
                let mut np = [0i64; SZ];
                let lo = if p == 0 { 1 } else { 0 };
                for i in 0..SZ {
                    if dp[i] == 0 {
                        continue;
                    }
                    for d in lo..10i32 {
                        let delta = if (p + 1) & 1 == 1 { d } else { -d };
                        let j = i as i32 + delta;
                        if (0..SZ as i32).contains(&j) {
                            np[j as usize] += dp[i];
                        }
                    }
                }
                dp = np;
            }
            f[len] = dp[OFF];
        }

        Tables { w, f }
    })
}

impl Solution {
    /// Digit DP with lazy static precomputation
    ///
    /// # Intuition
    /// Count balanced integers up to a number using digit DP, then use
    /// count(high) - count(low-1) to get the answer for the range.
    ///
    /// # Approach
    /// Use OnceLock for one-time initialization of lookup tables across all calls.
    /// Each query then runs in O(D * 10) time with O(1) table lookups.
    ///
    /// # Complexity
    /// - Time: O(D * 10) per query, O(D^2 * S * 10) one-time precomputation
    /// - Space: O(D * S) for static completion tables
    pub fn count_balanced_permutations(low: i64, high: i64) -> i64 {
        let tables = get_tables();
        let w = &tables.w;
        let f = &tables.f;

        let go = |x: i64| -> i64 {
            if x < 10 {
                return 0;
            }
            let mut a = [0i32; 16];
            let mut v = x;
            let mut n = 0;
            while v > 0 {
                a[n] = (v % 10) as i32;
                v /= 10;
                n += 1;
            }
            a[..n].reverse();

            let mut ans: i64 = (2..n).map(|i| f[i]).sum();
            let mut cur = OFF as i32;

            for p in 0..n {
                let lim = a[p];
                let lo = if p == 0 { 1 } else { 0 };
                let rem = n - p - 1;

                for d in lo..lim {
                    let delta = if (p + 1) & 1 == 1 { d } else { -d };
                    let nxt = cur + delta;
                    if rem == 0 {
                        if nxt == OFF as i32 {
                            ans += 1;
                        }
                    } else {
                        let t = (2 * OFF as i32 - nxt) as usize;
                        if t < SZ {
                            ans += w[((p + 2) & 1) ^ 1][rem][t];
                        }
                    }
                }

                let delta = if (p + 1) & 1 == 1 { lim } else { -lim };
                cur += delta;
                if cur < 0 || cur >= SZ as i32 {
                    return ans;
                }
            }

            if cur == OFF as i32 {
                ans += 1;
            }
            ans
        };

        go(high) - go(low - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_balanced_permutations(1, 100), 9);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_balanced_permutations(120, 129), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_balanced_permutations(1234, 1234), 0);
    }

    #[test]
    fn test_small_range() {
        assert_eq!(Solution::count_balanced_permutations(10, 20), 1);
    }

    #[test]
    fn test_single_balanced() {
        assert_eq!(Solution::count_balanced_permutations(121, 121), 1);
    }

    #[test]
    fn test_medium_ranges() {
        assert_eq!(Solution::count_balanced_permutations(100, 200), 9);
        assert_eq!(Solution::count_balanced_permutations(1000, 2000), 63);
    }

    #[test]
    fn test_large_range() {
        let result = Solution::count_balanced_permutations(1, 1_000_000_000_000_000);
        assert!(result > 0);
    }
}
