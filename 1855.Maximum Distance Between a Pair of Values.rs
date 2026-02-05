impl Solution {
    /// Finds the maximum distance between valid index pairs using binary search.
    ///
    /// # Intuition
    /// Since nums1 is non-increasing and nums2 is non-increasing, for each
    /// element in nums1 we can binary search for the rightmost position in
    /// nums2 where the value is still >= nums1[i].
    ///
    /// # Approach
    /// 1. For each index i in nums1, binary search in nums2[i..] for the
    ///    last index j where nums2[j] >= nums1[i].
    /// 2. Track the maximum j - i.
    ///
    /// # Complexity
    /// - Time: O(m * log n)
    /// - Space: O(1)
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums1
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let j = nums2[i..].partition_point(|&x| x >= val) + i;
                if j > i { (j - i - 1) as i32 } else { 0 }
            })
            .max()
            .unwrap_or(0)
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
}
