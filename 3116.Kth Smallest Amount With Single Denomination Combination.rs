impl Solution {
    /// Binary search over divisibility union.
    ///
    /// # Intuition
    /// Every amount is a multiple of a single coin, so we only need the union of all coin multiples.
    /// Counting how many values `<= x` are divisible by any coin lets us binary search the smallest
    /// `x` with count >= `k`.
    ///
    /// # Approach
    /// - Remove redundant coins that are multiples of smaller coins.
    /// - Precompute all subset LCMs and their inclusion-exclusion signs.
    /// - Binary search in `[1, min_coin * k]` using the subset counts.
    ///
    /// # Complexity
    /// - Time: O(2^n + 2^n log M), where n <= 15 and M is the answer.
    /// - Space: O(2^n) for subset LCMs.
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let mut coins: Vec<i64> = coins.into_iter().map(i64::from).collect();
        coins.sort_unstable();
        coins.dedup();

        let mut filtered: Vec<i64> = Vec::with_capacity(coins.len());
        for coin in coins {
            if filtered.iter().all(|prev| coin % *prev != 0) {
                filtered.push(coin);
            }
        }

        let min_coin = filtered[0];
        let upper = min_coin * i64::from(k);
        let inclusion = Self::build_inclusion_list(&filtered, upper);

        let mut low = 1i64;
        let mut high = upper;
        while low < high {
            let mid = low + (high - low) / 2;
            let count = Self::count_at_most(mid, &inclusion);
            if count >= i64::from(k) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }

    fn count_at_most(limit: i64, inclusion: &[(i64, i64)]) -> i64 {
        inclusion
            .iter()
            .map(|(lcm, sign)| sign * (limit / *lcm))
            .sum()
    }

    fn build_inclusion_list(coins: &[i64], limit: i64) -> Vec<(i64, i64)> {
        let n = coins.len();
        let subset_count = 1usize << n;
        let mut list = Vec::new();

        for mask in 1..subset_count {
            let mut lcm = 1i64;
            let mut valid = true;
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    lcm = Self::lcm_limited(lcm, coins[i], limit);
                    if lcm > limit {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                continue;
            }
            let sign = if mask.count_ones() % 2 == 1 { 1 } else { -1 };
            list.push((lcm, sign));
        }

        list
    }

    fn lcm_limited(a: i64, b: i64, limit: i64) -> i64 {
        let g = Self::gcd(a, b);
        let lcm = (a / g) as i128 * b as i128;
        if lcm > limit as i128 {
            limit.saturating_add(1)
        } else {
            lcm as i64
        }
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let rem = a % b;
            a = b;
            b = rem;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = Solution::find_kth_smallest(vec![3, 6, 9], 3);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::find_kth_smallest(vec![5, 2], 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_redundant_coins() {
        let result = Solution::find_kth_smallest(vec![2, 4, 8], 5);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_single_coin() {
        let result = Solution::find_kth_smallest(vec![7], 4);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_mixed_primes() {
        let result = Solution::find_kth_smallest(vec![5, 7], 2);
        assert_eq!(result, 7);
    }
}
