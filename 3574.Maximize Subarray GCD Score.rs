impl Solution {
    /// Maximize subarray GCD score with optional doubling.
    ///
    /// # Intuition
    /// Use distinct GCDs technique: for each right endpoint, only O(log M) distinct GCD values exist.
    /// Store positions for each pow2 value and use binary search for range counting.
    ///
    /// # Approach
    /// 1. Extract power-of-2 and odd parts, store positions by pow2 value
    /// 2. For each right endpoint r, maintain compressed (gcd, min_pow, range) entries
    /// 3. For each entry, binary search for threshold where doubling count ≤ k
    /// 4. Compute best scores with and without the boost
    ///
    /// # Complexity
    /// - Time: O(n log M log n) - n positions, O(log M) entries, O(log n) counting/search
    /// - Space: O(n) for position lists
    pub fn max_gcd_score(mut nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let pow2: Vec<u8> = nums
            .iter_mut()
            .map(|x| {
                let p = x.trailing_zeros() as u8;
                *x >>= p;
                p
            })
            .collect();

        let max_p = *pow2.iter().max().unwrap() as usize;
        let pos: Vec<Vec<usize>> =
            pow2.iter()
                .enumerate()
                .fold(vec![vec![]; max_p + 1], |mut pos, (i, &p)| {
                    pos[p as usize].push(i);
                    pos
                });

        #[inline]
        fn count(pos: &[usize], l: usize, r: usize) -> i32 {
            let lo = pos.partition_point(|&x| x < l);
            let hi = pos.partition_point(|&x| x <= r);
            (hi - lo) as i32
        }

        let mut ans = 0i64;
        let mut entries: Vec<(i32, u8, usize, usize)> = vec![];

        for r in 0..n {
            let capacity = entries.len() + 1;
            let mut new_entries = entries.into_iter().fold(
                Vec::with_capacity(capacity),
                |mut acc, (g, mp, ls, le)| {
                    let new_g = Self::gcd(g, nums[r]);
                    let new_mp = mp.min(pow2[r]);

                    match acc.last_mut() {
                        Some(last) if last.0 == new_g && last.1 == new_mp => last.3 = le,
                        _ => acc.push((new_g, new_mp, ls, le)),
                    }
                    acc
                },
            );

            match new_entries.last_mut() {
                Some(last) if last.0 == nums[r] && last.1 == pow2[r] => last.3 = r,
                _ => new_entries.push((nums[r], pow2[r], r, r)),
            }

            entries = new_entries;

            for &(g, mp, ls, le) in &entries {
                let positions = &pos[mp as usize];
                let mut lo = ls;
                let mut hi = le + 1;
                while lo < hi {
                    let mid = lo + (hi - lo) / 2;
                    if count(positions, mid, r) <= k {
                        hi = mid;
                    } else {
                        lo = mid + 1;
                    }
                }
                let l_star = lo;

                if ls < l_star {
                    let score = ((r - ls + 1) as i64) * ((g as i64) << mp);
                    if score > ans {
                        ans = score;
                    }
                }

                if l_star <= le {
                    let score = ((r - l_star + 1) as i64) * ((g as i64) << (mp + 1));
                    if score > ans {
                        ans = score;
                    }
                }
            }
        }

        ans
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_to_match() {
        assert_eq!(Solution::max_gcd_score(vec![2, 4], 1), 8);
    }

    #[test]
    fn test_single_element_doubled() {
        assert_eq!(Solution::max_gcd_score(vec![3, 5, 7], 2), 14);
    }

    #[test]
    fn test_no_doubling_optimal() {
        assert_eq!(Solution::max_gcd_score(vec![5, 5, 5], 1), 15);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_gcd_score(vec![3], 1), 6);
    }

    #[test]
    fn test_all_powers_of_two() {
        assert_eq!(Solution::max_gcd_score(vec![2, 4, 8], 3), 16);
    }

    #[test]
    fn test_large_k_small_array() {
        assert_eq!(Solution::max_gcd_score(vec![6, 12], 2), 24);
    }

    #[test]
    fn test_mixed_values() {
        assert_eq!(Solution::max_gcd_score(vec![6, 3, 12], 2), 24);
    }

    #[test]
    fn test_full_array_optimal() {
        assert_eq!(Solution::max_gcd_score(vec![6, 6, 6], 2), 24);
    }
}
