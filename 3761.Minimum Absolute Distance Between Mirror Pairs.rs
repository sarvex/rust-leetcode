use std::collections::HashMap;

impl Solution {
    /// Single pass: track rightmost index that "provides" each value for minimal distance.
    ///
    /// # Intuition
    /// A mirror pair (i, j) has reverse(nums[i]) == nums[j] with i < j. For each j, the best
    /// candidate is the largest i < j with reverse(nums[i]) == nums[j], giving distance j - i.
    ///
    /// # Approach
    /// 1. Maintain a map: value v → largest index i seen so far with reverse(nums[i]) == v.
    /// 2. At each index j, if map contains nums[j], we have a mirror pair (map[nums[j]], j);
    ///    update the minimum distance.
    /// 3. Then record that index j provides reverse(nums[j]) for future positions.
    /// 4. Reversed values exceeding i32::MAX can never match any nums[j], so skip them
    ///    and use i32 keys for faster hashing.
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is max digits per element (≤10).
    /// - Space: O(n) for the map in the worst case.
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut provides: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let mut min_dist = i32::MAX;

        for (j, &val) in nums.iter().enumerate() {
            if let Some(&i) = provides.get(&val) {
                min_dist = min_dist.min((j - i) as i32);
                if min_dist == 1 {
                    return 1;
                }
            }
            let rev = Self::reverse_digits(val);
            if rev <= i64::from(i32::MAX) {
                provides.insert(rev as i32, j);
            }
        }

        if min_dist == i32::MAX { -1 } else { min_dist }
    }

    /// Reverses decimal digits of `x`; leading zeros are dropped (e.g. 120 → 21).
    /// Returns i64 to handle potential overflow (e.g. reverse(10^9) = 9_000_000_000),
    /// but callers filter out values exceeding i32::MAX.
    #[inline(always)]
    fn reverse_digits(x: i32) -> i64 {
        let mut n = i64::from(x);
        let mut r = 0_i64;
        while n != 0 {
            r = r * 10 + n % 10;
            n /= 10;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![12, 21, 45, 33, 54]), 1);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![120, 21]), 1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![21, 120]), -1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![42]), -1);
    }

    #[test]
    fn test_no_mirror_pair() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![1, 2, 3, 4]), -1);
    }

    #[test]
    fn test_adjacent_mirror() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![12, 21]), 1);
    }

    #[test]
    fn test_consecutive_reversed() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![1, 1]), 1);
    }
}
