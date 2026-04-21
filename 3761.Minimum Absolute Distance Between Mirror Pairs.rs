use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

/// Fast integer hasher using FxHash-style multiply-shift.
///
/// Avoids SipHash overhead for simple integer keys on LeetCode
/// where cryptographic resistance is unnecessary.
struct FxHasher(u64);

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.wrapping_mul(0x517c_c1b7_2722_0a95)).wrapping_add(u64::from(b));
        }
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.0 = (i as u64).wrapping_mul(0x517c_c1b7_2722_0a95);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

struct FxBuildHasher;

impl BuildHasher for FxBuildHasher {
    type Hasher = FxHasher;

    #[inline]
    fn build_hasher(&self) -> FxHasher {
        FxHasher(0)
    }
}

impl Solution {
    /// Single pass with fast hashing: track rightmost provider index per value.
    ///
    /// # Intuition
    /// A mirror pair (i, j) has reverse(nums\[i\]) == nums\[j\] with i < j. For each j,
    /// the best candidate is the largest i < j with reverse(nums\[i\]) == nums\[j\],
    /// giving distance j − i.
    ///
    /// # Approach
    /// 1. Maintain a map: value v → largest index i seen so far with reverse(nums\[i\]) == v.
    /// 2. At each index j, if map contains nums\[j\], we have a mirror pair; update the
    ///    minimum distance.
    /// 3. Then record that index j provides reverse(nums\[j\]) for future positions.
    /// 4. Reversed values exceeding i32::MAX can never match any nums\[j\], so skip them.
    /// 5. Uses a lightweight FxHash-style hasher instead of default SipHash for faster
    ///    integer key lookups.
    ///
    /// # Complexity
    /// - Time: O(n × d) where d is max digits per element (≤ 10).
    /// - Space: O(n) for the map in the worst case.
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut provides: HashMap<i32, usize, FxBuildHasher> =
            HashMap::with_capacity_and_hasher(nums.len(), FxBuildHasher);
        let mut min_dist = i32::MAX;

        for (j, val) in nums.iter().enumerate() {
            if let Some(&i) = provides.get(val) {
                min_dist = min_dist.min((j - i) as i32);
                if min_dist == 1 {
                    return 1;
                }
            }
            let rev = Self::reverse_digits(*val);
            if rev <= i32::MAX as u64 {
                provides.insert(rev as i32, j);
            }
        }

        if min_dist == i32::MAX {
            -1
        } else {
            min_dist
        }
    }

    /// Reverses decimal digits of `x`; leading zeros are dropped (e.g. 120 → 21).
    ///
    /// Returns u64 to handle potential overflow when reversing large values
    /// (e.g. reverse(1_999_999_999) > i32::MAX). Callers filter out such results.
    #[inline(always)]
    fn reverse_digits(x: i32) -> u64 {
        let mut n = x as u64;
        let mut r = 0_u64;
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
        assert_eq!(
            Solution::min_mirror_pair_distance(vec![12, 21, 45, 33, 54]),
            1
        );
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
