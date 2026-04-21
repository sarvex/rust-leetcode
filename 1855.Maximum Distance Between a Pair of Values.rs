impl Solution {
    /// Finds the maximum distance between valid index pairs using two pointers.
    ///
    /// # Intuition
    /// Both arrays are non-increasing. For a fixed i, as j increases the
    /// predicate `nums2[j] >= nums1[i]` can only flip from true to false.
    /// When i increases, nums1[i] can only decrease, so the rightmost valid
    /// j never moves left. This monotonicity means a single forward sweep
    /// with two pointers beats per-index binary search.
    ///
    /// # Approach
    /// 1. Walk i over nums1 and j over nums2, both starting at 0.
    /// 2. While j < nums2.len() and nums2[j] >= nums1[i], advance j.
    /// 3. After the inner loop, j - 1 is the last valid match for i (if any),
    ///    so update the answer with (j - 1 - i) when j > i.
    /// 4. Advance i. Never reset j — it is monotonically non-decreasing.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(1)
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let (mut i, mut j, mut best) = (0usize, 0usize, 0i32);
        while i < m && j < n {
            if nums2[j] >= nums1[i] {
                best = best.max((j - i) as i32);
                j += 1;
            } else if i == j {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_distance() {
        assert_eq!(
            Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
            2
        );
    }

    #[test]
    fn test_no_valid_pairs() {
        assert_eq!(
            Solution::max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25]),
            2
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_distance(vec![5], vec![5]), 0);
    }

    #[test]
    fn test_nums1_longer_than_nums2() {
        assert_eq!(
            Solution::max_distance(
                vec![
                    9819, 9508, 7398, 7347, 6337, 5756, 5493, 5446, 5123, 3215, 1597, 774, 368, 313
                ],
                vec![9933, 9813, 9770, 9697, 9514, 9490, 9441, 9439, 8939, 8754, 8665, 8560],
            ),
            9
        );
    }
}
