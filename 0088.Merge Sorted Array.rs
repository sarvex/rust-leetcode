impl Solution {
    /// Reverse three-pointer merge of two sorted arrays in-place.
    ///
    /// # Intuition
    /// Merging from the back avoids overwriting unprocessed elements in
    /// `nums1`. The largest unplaced element goes at the end position,
    /// working backwards until all of `nums2` is placed.
    ///
    /// # Approach
    /// Start pointers at the last valid elements of `nums1` (index m-1)
    /// and `nums2` (index n-1), plus a write pointer at m+n-1. Compare
    /// and place the larger element at the write position. Continue until
    /// `nums2` is fully merged.
    ///
    /// # Complexity
    /// - Time: O(m + n) — each element placed once
    /// - Space: O(1) — in-place modification
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        if n == 0 {
            return;
        }

        let mut write = m + n;
        while n > 0 {
            write -= 1;
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[write] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[write] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn empty_nums2() {
        let mut nums1 = vec![1];
        let mut nums2: Vec<i32> = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn empty_nums1() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn interleaved() {
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let mut nums2 = vec![2, 4, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
