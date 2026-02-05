use std::collections::HashMap;

impl Solution {
    /// Prefix sum modular arithmetic with hash map for shortest removable subarray.
    ///
    /// # Intuition
    /// If total sum mod p is 0, no removal needed. Otherwise, find the shortest
    /// subarray whose sum mod p equals the total's remainder. Use prefix sums:
    /// `(prefix[i] - prefix[j]) % p == remainder` means `prefix[j] % p == (prefix[i] - remainder) % p`.
    ///
    /// # Approach
    /// 1. Compute total remainder k = sum % p; if k == 0, return 0
    /// 2. Track last occurrence of each prefix mod value
    /// 3. For each prefix, look up the target mod to find shortest subarray
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the hash map
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut k = 0;
        for &x in &nums {
            k = (k + x) % p;
        }
        if k == 0 {
            return 0;
        }

        let n = nums.len();
        let mut last: HashMap<i32, i32> = HashMap::new();
        last.insert(0, -1);
        let mut result = n as i32;
        let mut prefix = 0;

        for i in 0..n {
            prefix = (prefix + nums[i]) % p;
            let target = (prefix - k + p) % p;
            if let Some(&prev) = last.get(&target) {
                result = result.min(i as i32 - prev);
            }
            last.insert(prefix, i as i32);
        }

        if result >= n as i32 { -1 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_one() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
    }

    #[test]
    fn already_divisible() {
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 0);
    }

    #[test]
    fn impossible() {
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 7), -1);
    }
}
