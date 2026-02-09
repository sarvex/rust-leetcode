use std::sync::OnceLock;

const MX: usize = 500_000;

/// Precomputed answer for each index: `ans[i]` = largest prime ≤ i that is a
/// consecutive-prime prefix sum (starting from 2). Built once via `OnceLock`.
static SPRIME_ANS: OnceLock<Vec<i32>> = OnceLock::new();

impl Solution {
    /// Single lookup into a precomputed table of answers for 0..=500_000.
    ///
    /// # Intuition
    /// Valid answers are primes ≤ n that equal a prefix sum of consecutive
    /// primes from 2. Precompute once for all n in range; each query is O(1).
    ///
    /// # Approach
    /// 1. Sieve primes up to MX; build prefix sums of that list.
    /// 2. For each i in 2..=MX, track the largest prime prefix sum ≤ i into `ans[i]`.
    /// 3. Return `ans[n]` (or 0 for n &lt; 2 via table content).
    ///
    /// # Complexity
    /// - Time: O(1) per call after O(MX log log MX) one-time precomputation.
    /// - Space: O(MX) for the static table.
    pub fn largest_prime(n: i32) -> i32 {
        let sprimes = SPRIME_ANS.get_or_init(Self::build_sprime_ans);
        let idx = n as usize;
        if idx > MX {
            return *sprimes.last().unwrap_or(&0);
        }
        sprimes[idx]
    }

    fn build_sprime_ans() -> Vec<i32> {
        let mut is_prime = [true; MX + 1];
        is_prime[..2].fill(false);
        let mut primes = Vec::new();
        for i in 2..=MX {
            if !is_prime[i] {
                continue;
            }
            primes.push(i as i32);
            for j in (i * i..=MX).step_by(i) {
                is_prime[j] = false;
            }
        }
        let mut ans = vec![0i32; MX + 1];
        let (mut sum_p, mut last, mut j) = (0i64, 0i32, 0usize);
        for i in 2..=MX {
            while j < primes.len() && sum_p + i64::from(primes[j]) <= i as i64 {
                sum_p += i64::from(primes[j]);
                j += 1;
                if sum_p <= MX as i64 && is_prime[sum_p as usize] {
                    last = sum_p as i32;
                }
            }
            ans[i] = last;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::largest_prime(20), 17);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::largest_prime(2), 2);
    }

    #[test]
    fn test_n1() {
        assert_eq!(Solution::largest_prime(1), 0);
    }

    #[test]
    fn test_n3() {
        // Primes ≤3: 2,3. Sums: 2, 5. Only 2 ≤ 3 and prime => 2
        assert_eq!(Solution::largest_prime(3), 2);
    }

    #[test]
    fn test_n5() {
        // Sums: 2, 2+3=5. Both prime and ≤5 => 5
        assert_eq!(Solution::largest_prime(5), 5);
    }

    #[test]
    fn test_n10() {
        // Sums: 2, 5, 10. 10 is not prime. Best = 5
        assert_eq!(Solution::largest_prime(10), 5);
    }
}
