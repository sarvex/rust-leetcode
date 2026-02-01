use std::collections::BinaryHeap;

const MOD: u64 = 1_000_000_007;

impl Solution {
    /// Fast-forward after the minimum can leap past the maximum.
    ///
    /// # Intuition
    /// If the smallest value multiplied by `multiplier` is still <= the current maximum, the
    /// maximum never changes, so we can simulate those steps. Once the smallest value would exceed
    /// the maximum, each subsequent operation becomes a stable round-robin on the sorted order.
    ///
    /// # Approach
    /// - If `multiplier == 1`, return the original array.
    /// - Phase 1: use a packed min-heap to simulate while `min * multiplier <= max`.
    /// - Phase 2: distribute the remaining operations as full cycles plus a prefix.
    /// - Apply modular exponentiation to compute final values.
    ///
    /// # Complexity
    /// - Time: O(n log n + t log n + n), with t operations in phase 1.
    /// - Space: O(n)
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let n = nums.len();
        if k == 0 || multiplier == 1 {
            return nums;
        }

        let multiplier_u64 = multiplier as u64;
        let mut max_value: u64 = 0;
        for &value in nums.iter() {
            let value_u64 = value as u64;
            if value_u64 > max_value {
                max_value = value_u64;
            }
        }

        let max_div = max_value / multiplier_u64;

        let mut packed_keys: Vec<u64> = Vec::with_capacity(n);
        for (index, &value) in nums.iter().enumerate() {
            let packed = ((value as u64) << 32) | (index as u64);
            packed_keys.push(!packed);
        }
        let mut heap: BinaryHeap<u64> = BinaryHeap::from(packed_keys);

        let mut remaining = k as i64;
        while remaining > 0 {
            let key = *heap.peek().expect("heap always contains all elements");
            let packed = !key;
            let value = packed >> 32;

            if value > max_div {
                break;
            }

            let index = (packed as u32) as usize;
            heap.pop();

            let multiplied = value * multiplier_u64;
            let new_packed = (multiplied << 32) | (index as u64);
            heap.push(!new_packed);
            remaining -= 1;
        }

        if remaining == 0 {
            for key in heap.into_vec() {
                let packed = !key;
                let value = (packed >> 32) as i32;
                let index = (packed as u32) as usize;
                nums[index] = value;
            }
            return nums;
        }

        let n_i64 = n as i64;
        let full_cycles = (remaining / n_i64) as u64;
        let prefix = (remaining % n_i64) as usize;

        let base = mod_pow(multiplier_u64, full_cycles);
        let extra = (base * multiplier_u64) % MOD;

        let mut keys = heap.into_vec();
        if prefix > 0 {
            keys.select_nth_unstable_by(prefix, |a, b| b.cmp(a));
            for &key in &keys[..prefix] {
                let packed = !key;
                let value = packed >> 32;
                let index = (packed as u32) as usize;
                nums[index] = ((value * extra) % MOD) as i32;
            }
            for &key in &keys[prefix..] {
                let packed = !key;
                let value = packed >> 32;
                let index = (packed as u32) as usize;
                nums[index] = ((value * base) % MOD) as i32;
            }
        } else {
            for key in keys {
                let packed = !key;
                let value = packed >> 32;
                let index = (packed as u32) as usize;
                nums[index] = ((value * base) % MOD) as i32;
            }
        }

        nums
    }
}

#[inline]
fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result: u64 = 1;
    while exp > 0 {
        if (exp & 1) == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 1, 3, 5, 6];
        assert_eq!(Solution::get_final_state(nums, 5, 2), vec![8, 4, 6, 5, 6]);
    }

    #[test]
    fn example_two() {
        let nums = vec![100000, 2000];
        assert_eq!(
            Solution::get_final_state(nums, 2, 1_000_000),
            vec![999_999_307, 999_999_993]
        );
    }

    #[test]
    fn multiplier_one_keeps_values() {
        let nums = vec![5, 1, 7];
        assert_eq!(Solution::get_final_state(nums, 100, 1), vec![5, 1, 7]);
    }

    #[test]
    fn equal_values_round_robin() {
        let nums = vec![2, 2, 2];
        assert_eq!(Solution::get_final_state(nums, 4, 3), vec![18, 6, 6]);
    }

    #[test]
    fn initial_phase_then_cycle() {
        let nums = vec![1, 3];
        assert_eq!(Solution::get_final_state(nums, 2, 2), vec![4, 3]);
    }
}
