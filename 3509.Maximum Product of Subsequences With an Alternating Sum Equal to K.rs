use std::collections::HashMap;

impl Solution {
    /// Dynamic programming with bitset for product tracking of alternating subsequences.
    ///
    /// # Intuition
    /// Use u64 chunks as bitset for efficient product tracking. Iterate only through set bits
    /// using trailing_zeros for better performance.
    ///
    /// # Approach
    /// - `dp[(parity, sum)]` = (bitset chunks, has_zero, has_big)
    /// - Create new_dp for transitions, merge after each number
    /// - Use trailing_zeros to iterate only set bits
    ///
    /// # Complexity
    /// - Time: O(n × S × P) where P is number of set product bits
    /// - Space: O(S × L/64)
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let n = nums.len();
        let max_alt = (n * 12) as i32;

        if k.abs() > max_alt {
            return -1;
        }

        let num_chunks = (limit as usize / 64) + 1;

        type State = (Vec<u64>, bool, bool);
        let mut dp: HashMap<(u8, i32), State> = HashMap::new();

        for &num in &nums {
            let mut new_dp: HashMap<(u8, i32), State> = HashMap::new();

            for (&(parity, sum), (products, has_zero, has_big)) in &dp {
                let has_any = products.iter().any(|&x| x != 0);
                if !has_any && !*has_zero && !*has_big {
                    continue;
                }

                let delta = if parity == 0 { num } else { -num };
                let new_sum = sum + delta;
                let next_parity = 1 - parity;
                let key = (next_parity, new_sum);

                let entry = new_dp
                    .entry(key)
                    .or_insert_with(|| (vec![0u64; num_chunks], false, false));

                if num == 0 {
                    if has_any || *has_zero || *has_big {
                        entry.1 = true;
                    }
                } else {
                    if *has_zero {
                        entry.1 = true;
                    }
                    if *has_big {
                        entry.2 = true;
                    }
                    if has_any {
                        for (chunk_idx, &chunk) in products.iter().enumerate() {
                            let mut remaining = chunk;
                            while remaining != 0 {
                                let bit = remaining.trailing_zeros() as usize;
                                remaining &= remaining - 1; // Clear lowest set bit

                                let prod = (chunk_idx * 64 + bit) as i64;
                                let new_prod = prod * (num as i64);
                                if new_prod <= limit as i64 {
                                    let np = new_prod as usize;
                                    entry.0[np / 64] |= 1u64 << (np % 64);
                                } else {
                                    entry.2 = true;
                                }
                            }
                        }
                    }
                }
            }

            // Merge new_dp into dp
            for (key, (new_prods, new_zero, new_big)) in new_dp {
                let entry = dp
                    .entry(key)
                    .or_insert_with(|| (vec![0u64; num_chunks], false, false));
                entry.1 |= new_zero;
                entry.2 |= new_big;
                entry
                    .0
                    .iter_mut()
                    .zip(new_prods.iter())
                    .for_each(|(dst, &src)| *dst |= src);
            }

            // Start new subsequence
            let key = (1_u8, num);
            let entry = dp
                .entry(key)
                .or_insert_with(|| (vec![0u64; num_chunks], false, false));

            if num == 0 {
                entry.1 = true;
            } else if num <= limit {
                let p = num as usize;
                entry.0[p / 64] |= 1u64 << (p % 64);
            } else {
                entry.2 = true;
            }
        }

        let mut result = -1;

        for parity in 0..2 {
            if let Some((products, has_zero, _)) = dp.get(&(parity, k)) {
                if *has_zero {
                    result = result.max(0);
                }
                for (chunk_idx, &chunk) in products.iter().enumerate().rev() {
                    if chunk != 0 {
                        let high_bit = 63 - chunk.leading_zeros() as usize;
                        let prod = (chunk_idx * 64 + high_bit) as i32;
                        result = result.max(prod);
                        break;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_product(vec![1, 2, 3], 2, 10), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_product(vec![0, 2, 3], -5, 12), -1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_product(vec![2, 2, 3, 3], 0, 9), 9);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_product(vec![5], 5, 10), 5);
    }

    #[test]
    fn test_zero_in_array() {
        assert_eq!(Solution::max_product(vec![0, 1], 0, 5), 0);
    }

    #[test]
    fn test_product_exceeds_limit() {
        assert_eq!(Solution::max_product(vec![10, 10], 0, 5), -1);
    }

    #[test]
    fn test_zero_makes_valid() {
        assert_eq!(Solution::max_product(vec![10, 10, 9, 0], 1, 20), 0);
    }

    #[test]
    fn test_tle_case() {
        assert_eq!(Solution::max_product(vec![9, 4, 6, 3, 12], 20, 20), -1);
    }
}
