pub struct Solution;

impl Solution {
    /// Finds the peak index in a mountain array via binary search.
    ///
    /// # Intuition
    /// The peak is the only index where `arr[mid] > arr[mid+1]` transitions
    /// from false to true. Binary search narrows down this transition point.
    ///
    /// # Approach
    /// Binary search between indices 1 and n-2. If `arr[mid] > arr[mid+1]`,
    /// the peak is at mid or to the left; otherwise it is to the right.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (1, arr.len() - 2);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if arr[mid] > arr[mid + 1] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    }

    #[test]
    fn test_longer() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    }

    #[test]
    fn test_peak_at_end() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    }

    #[test]
    fn test_larger_mountain() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
    }
}
