
impl Solution {
    /// Maximizes advantage of nums1 over nums2 using greedy assignment.
    ///
    /// # Intuition
    /// Sort nums1. For each nums2 element (processed in ascending order),
    /// assign the smallest nums1 element that beats it. Otherwise assign
    /// the smallest remaining (least useful) element.
    ///
    /// # Approach
    /// Sort nums1. Sort indices of nums2 by value. Use two pointers on
    /// sorted nums1: if the current element beats nums2's current target,
    /// assign from the left; otherwise assign the largest remaining from
    /// the right.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n) for index array and result
    pub fn advantage_count(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums1.len();
        let mut idx: Vec<usize> = Vec::with_capacity(n);
        idx.extend(0..n);
        idx.sort_unstable_by_key(|&i| nums2[i]);
        nums1.sort_unstable();

        let mut result = vec![0; n];
        let (mut lo, mut hi) = (0, n - 1);
        for &num in &nums1 {
            if num > nums2[idx[lo]] {
                result[idx[lo]] = num;
                lo += 1;
            } else {
                result[idx[hi]] = num;
                hi = hi.wrapping_sub(1);
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
        let result = Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]);
        assert_eq!(result, vec![2, 11, 7, 15]);
    }

    #[test]
    fn test_no_advantage() {
        let result = Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]);
        // 24 > 13, 32 > 25, 8 sacrificed against 32, 12 > 11
        assert_eq!(result, vec![24, 32, 8, 12]);
    }
}
