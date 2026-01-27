impl Solution {
    /// Counts pairs satisfying the transformed inequality using modified merge sort.
    ///
    /// # Intuition
    /// Transform `nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff` into
    /// `d[i] <= d[j] + diff` where `d[k] = nums1[k] - nums2[k]`. Counting
    /// valid pairs becomes a merge-sort inversion count variant.
    ///
    /// # Approach
    /// 1. Compute difference array `d[i] = nums1[i] - nums2[i]`
    /// 2. Use merge sort to count valid pairs during the merge phase
    /// 3. For each element in right half, count valid left-half elements via two pointers
    ///
    /// # Complexity
    /// - Time: O(n log n) — merge sort
    /// - Space: O(n) — temporary arrays during merge
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let mut differences: Vec<i64> = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| i64::from(a) - i64::from(b))
            .collect();

        Self::merge_sort_count(&mut differences, i64::from(diff))
    }

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

    fn merge_and_count(arr: &mut [i64], mid: usize, diff: i64) -> i64 {
        let left = arr[..mid].to_vec();
        let right = arr[mid..].to_vec();

        let mut count = 0i64;
        let mut left_ptr = 0;

        for &right_val in &right {
            let threshold = right_val + diff;
            while left_ptr < left.len() && left[left_ptr] <= threshold {
                left_ptr += 1;
            }
            count += left_ptr as i64;
        }

        let (mut i, mut j, mut k) = (0, 0, 0);
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
        assert_eq!(
            Solution::number_of_pairs(vec![3, 2, 5], vec![2, 2, 1], 1),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_pairs(vec![3, -1], vec![-2, 2], -1), 0);
    }

    #[test]
    fn test_all_pairs_valid() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 2, 3], vec![3, 2, 1], 10),
            3
        );
    }

    #[test]
    fn test_no_pairs_valid() {
        assert_eq!(Solution::number_of_pairs(vec![10, 1], vec![1, 10], -20), 0);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::number_of_pairs(vec![0, 0], vec![0, 0], 0), 1);
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(
            Solution::number_of_pairs(vec![-10000, 10000], vec![10000, -10000], 0),
            1
        );
    }
}
