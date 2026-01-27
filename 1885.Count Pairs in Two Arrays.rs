impl Solution {
    /// Counts pairs where nums1[i] + nums1[j] > nums2[i] + nums2[j].
    ///
    /// # Intuition
    /// The condition simplifies to (nums1[i] - nums2[i]) + (nums1[j] - nums2[j]) > 0.
    /// Sorting the difference array enables a two-pointer approach.
    ///
    /// # Approach
    /// 1. Compute the difference array diff[i] = nums1[i] - nums2[i].
    /// 2. Sort the differences.
    /// 3. Use two pointers from both ends to count valid pairs.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn count_pairs(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut diff: Vec<i32> = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        diff.sort_unstable();

        let mut result = 0i64;
        let (mut lo, mut hi) = (0, diff.len() - 1);
        while lo < hi {
            if diff[lo] + diff[hi] > 0 {
                result += (hi - lo) as i64;
                hi -= 1;
            } else {
                lo += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::count_pairs(vec![2, 1, 2, 1], vec![1, 2, 1, 2]), 1);
    }

    #[test]
    fn test_all_valid() {
        assert_eq!(Solution::count_pairs(vec![3, 3, 3], vec![1, 1, 1]), 3);
    }

    #[test]
    fn test_none_valid() {
        assert_eq!(Solution::count_pairs(vec![1, 1], vec![3, 3]), 0);
    }
}
