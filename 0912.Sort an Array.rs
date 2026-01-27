impl Solution {
    /// Sorts an array using merge sort for guaranteed O(n log n) performance.
    ///
    /// # Intuition
    /// Merge sort divides the array in half, recursively sorts each half,
    /// and merges the sorted halves.
    ///
    /// # Approach
    /// Recursively split the array. Merge two sorted halves into a temporary
    /// buffer and copy back.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n) for the auxiliary buffer
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut buf = vec![0; n];
        Self::merge_sort(&mut nums, &mut buf, 0, n);
        nums
    }

    fn merge_sort(nums: &mut [i32], buf: &mut [i32], lo: usize, hi: usize) {
        if hi - lo <= 1 {
            return;
        }
        let mid = lo + (hi - lo) / 2;
        Self::merge_sort(nums, buf, lo, mid);
        Self::merge_sort(nums, buf, mid, hi);

        let (mut i, mut j, mut k) = (lo, mid, lo);
        while i < mid && j < hi {
            if nums[i] <= nums[j] {
                buf[k] = nums[i];
                i += 1;
            } else {
                buf[k] = nums[j];
                j += 1;
            }
            k += 1;
        }
        while i < mid {
            buf[k] = nums[i];
            i += 1;
            k += 1;
        }
        while j < hi {
            buf[k] = nums[j];
            j += 1;
            k += 1;
        }
        nums[lo..hi].copy_from_slice(&buf[lo..hi]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_with_negatives() {
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::sort_array(vec![1]), vec![1]);
    }
}
