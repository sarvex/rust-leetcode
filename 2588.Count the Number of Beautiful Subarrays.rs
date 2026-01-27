use std::collections::HashMap;

impl Solution {
    /// Count beautiful subarrays using XOR prefix and frequency map.
    ///
    /// # Intuition
    /// A subarray is beautiful if all bits can be zeroed out, which means the XOR
    /// of the subarray equals 0. Two equal prefix XOR values define such a subarray.
    ///
    /// # Approach
    /// 1. Maintain a running XOR prefix
    /// 2. Use a HashMap to count occurrences of each prefix XOR
    /// 3. For each new prefix, add the count of previous equal prefixes
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut cnt: HashMap<i32, i64> = HashMap::new();
        cnt.insert(0, 1);
        let mut ans: i64 = 0;
        let mut mask = 0;

        for &x in &nums {
            mask ^= x;
            let count = cnt.entry(mask).or_insert(0);
            ans += *count;
            *count += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::beautiful_subarrays(vec![4, 3, 1, 2, 4]), 2);
    }

    #[test]
    fn test_single_zero() {
        assert_eq!(Solution::beautiful_subarrays(vec![0]), 1);
    }

    #[test]
    fn test_no_beautiful() {
        assert_eq!(Solution::beautiful_subarrays(vec![1, 2, 3]), 0);
    }
}
