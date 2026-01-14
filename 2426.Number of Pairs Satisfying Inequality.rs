impl Solution {
    /// Counts pairs satisfying the transformed inequality using modified merge sort.
    ///
    /// # Intuition
    /// Transform the inequality `nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff`
    /// into `d[i] <= d[j] + diff` where `d[k] = nums1[k] - nums2[k]`. This reduces
    /// the problem to counting pairs in a single array during merge sort.
    ///
    /// # Approach
    /// 1. Compute difference array `d[i] = nums1[i] - nums2[i]`
    /// 2. Use merge sort to count valid pairs during the merge phase
    /// 3. For each element in right half, count elements in left half satisfying condition
    /// 4. Since both halves are sorted, use two-pointer technique for O(n) merge counting
    ///
    /// # Complexity
    /// - Time: O(n log n) - standard merge sort complexity
    /// - Space: O(n) - temporary arrays during merge
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let n = nums1.len();
        let mut differences: Vec<i64> = (0..n)
            .map(|i| i64::from(nums1[i]) - i64::from(nums2[i]))
            .collect();

        Self::merge_sort_count(&mut differences, i64::from(diff))
    }

    /// Recursively sorts array while counting valid pairs across partitions.
    fn merge_sort_count(arr: &mut [i64], diff: i64) -> i64 {
        let n = arr.len();
        if n <= 1 {
            return 0;
        }

        let mid = n / 2;
        let left_count = Self::merge_sort_count(&mut arr[..mid], diff);
        let right_count = Self::merge_sort_count(&mut arr[mid..], diff);
        let cross_count = Self::merge_and_count(arr, mid, diff);

        left_count + right_count + cross_count
    }

    /// Merges two sorted halves while counting pairs where left[i] <= right[j] + diff.
    fn merge_and_count(arr: &mut [i64], mid: usize, diff: i64) -> i64 {
        let left = arr[..mid].to_vec();
        let right = arr[mid..].to_vec();

        // Count pairs: for each element in right, count valid elements in left
        // Since both are sorted, use two-pointer technique
        let mut count = 0i64;
        let mut left_ptr = 0;

        for &right_val in &right {
            let threshold = right_val + diff;
            while left_ptr < left.len() && left[left_ptr] <= threshold {
                left_ptr += 1;
            }
            count += left_ptr as i64;
        }

        // Standard merge to maintain sorted order for parent calls
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i];
                i += 1;
            } else {
                arr[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            arr[k] = left[i];
            i += 1;
            k += 1;
        }

        while j < right.len() {
            arr[k] = right[j];
            j += 1;
            k += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![3, 2, 5];
        let nums2 = vec![2, 2, 1];
        let diff = 1;
        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 3);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![3, -1];
        let nums2 = vec![-2, 2];
        let diff = -1;
        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 0);
    }

    #[test]
    fn test_all_pairs_valid() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![3, 2, 1];
        let diff = 10;
        // d = [-2, 0, 2], all pairs satisfy d[i] <= d[j] + 10
        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 3);
    }

    #[test]
    fn test_no_pairs_valid() {
        let nums1 = vec![10, 1];
        let nums2 = vec![1, 10];
        let diff = -20;
        // d = [9, -9], need 9 <= -9 + (-20) = -29, false
        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 0);
    }

    #[test]
    fn test_single_pair() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        let diff = 0;
        // d = [0, 0], need 0 <= 0 + 0 = 0, true
        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 1);
    }

    #[test]
    fn test_negative_values() {
        let nums1 = vec![-10000, 10000];
        let nums2 = vec![10000, -10000];
        let diff = 0;
        // d = [-20000, 20000], need -20000 <= 20000 + 0, true
        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 1);
    }
}
