use std::collections::HashMap;

/// Optimized solution with proper simplification handling.
pub struct Solution {}

impl Solution {
    pub fn count_sequences(nums: Vec<i32>, k: i64) -> i32 {
        // Factorize k - return 0 if k has prime factors other than 2, 3, 5
        let Some(target) = factorize(k) else {
            return 0;
        };
        let k2 = target[0];
        let k3 = target[1];
        let k5 = target[2];

        const MAX: i32 = 40;

        // Use i64 for encoding: 8 bits per component (0-255), plenty for 0-40
        let encode = |n2: i32, n3: i32, n5: i32, d2: i32, d3: i32, d5: i32| -> i64 {
            ((n2 + MAX) as i64) << 40
                | ((n3 + MAX) as i64) << 32
                | ((n5 + MAX) as i64) << 24
                | ((d2 + MAX) as i64) << 16
                | ((d3 + MAX) as i64) << 8
                | (d5 + MAX) as i64
        };

        let decode = |key: i64| -> (i32, i32, i32, i32, i32, i32) {
            let n2 = ((key >> 40) & 0xFF) as i32 - MAX;
            let n3 = ((key >> 32) & 0xFF) as i32 - MAX;
            let n5 = ((key >> 24) & 0xFF) as i32 - MAX;
            let d2 = ((key >> 16) & 0xFF) as i32 - MAX;
            let d3 = ((key >> 8) & 0xFF) as i32 - MAX;
            let d5 = (key & 0xFF) as i32 - MAX;
            (n2, n3, n5, d2, d3, d5)
        };

        // Pre-allocate HashMap with reasonable capacity
        let mut dp: HashMap<i64, i64> = HashMap::with_capacity(2048);
        dp.insert(encode(0, 0, 0, 0, 0, 0), 1);

        for &num in &nums {
            let factors = factorize(num as i64).unwrap_or([0, 0, 0]);
            let f2 = factors[0];
            let f3 = factors[1];
            let f5 = factors[2];

            let mut new_dp: HashMap<i64, i64> = HashMap::with_capacity(dp.len() * 3 / 2);

            for (&key, &count) in &dp {
                let (n2, n3, n5, d2, d3, d5) = decode(key);

                // Multiply: add to numerator then simplify
                let (sn2, sn3, sn5, sd2, sd3, sd5) =
                    simplify(n2 + f2, n3 + f3, n5 + f5, d2, d3, d5);
                let mul_key = encode(sn2, sn3, sn5, sd2, sd3, sd5);
                *new_dp.entry(mul_key).or_insert(0) += count;

                // Divide: add to denominator then simplify
                let (sn2, sn3, sn5, sd2, sd3, sd5) =
                    simplify(n2, n3, n5, d2 + f2, d3 + f3, d5 + f5);
                let div_key = encode(sn2, sn3, sn5, sd2, sd3, sd5);
                *new_dp.entry(div_key).or_insert(0) += count;

                // Leave: no change
                *new_dp.entry(key).or_insert(0) += count;
            }

            dp = new_dp;
        }

        // Count states matching target
        let mut result = 0i64;
        for (&key, &count) in &dp {
            let (n2, n3, n5, d2, d3, d5) = decode(key);
            let net2 = n2 - d2;
            let net3 = n3 - d3;
            let net5 = n5 - d5;
            if net2 == k2 && net3 == k3 && net5 == k5 {
                result += count;
            }
        }

        result as i32
    }
}

/// Simplify by canceling common factors
fn simplify(
    n2: i32,
    n3: i32,
    n5: i32,
    d2: i32,
    d3: i32,
    d5: i32,
) -> (i32, i32, i32, i32, i32, i32) {
    let c2 = n2.min(d2);
    let c3 = n3.min(d3);
    let c5 = n5.min(d5);
    (n2 - c2, n3 - c3, n5 - c5, d2 - c2, d3 - c3, d5 - c5)
}

/// Factorize n into exponents of {2, 3, 5}
fn factorize(n: i64) -> Option<[i32; 3]> {
    let mut remaining = n;

    let mut e2 = 0;
    while remaining % 2 == 0 {
        e2 += 1;
        remaining /= 2;
    }
    let mut e3 = 0;
    while remaining % 3 == 0 {
        e3 += 1;
        remaining /= 3;
    }
    let mut e5 = 0;
    while remaining % 5 == 0 {
        e5 += 1;
        remaining /= 5;
    }

    if remaining != 1 {
        return None;
    }

    Some([e2, e3, e5])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 3, 2];
        let k = 6;
        assert_eq!(Solution::count_sequences(nums, k), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 6, 3];
        let k = 2;
        assert_eq!(Solution::count_sequences(nums, k), 2);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 5];
        let k = 1;
        assert_eq!(Solution::count_sequences(nums, k), 3);
    }

    #[test]
    fn test_k_with_other_primes() {
        assert_eq!(Solution::count_sequences(vec![3, 2, 2], 14), 0);
    }

    #[test]
    fn test_cancel_out() {
        assert_eq!(Solution::count_sequences(vec![2, 2], 1), 3);
    }

    #[test]
    fn test_larger() {
        assert_eq!(Solution::count_sequences(vec![2, 2, 2], 1), 7);
    }
}
