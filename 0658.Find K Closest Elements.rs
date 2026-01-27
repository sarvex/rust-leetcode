impl Solution {
    /// Finds the k closest elements to x by shrinking from both ends.
    ///
    /// # Intuition
    /// Start with the full array and repeatedly remove the endpoint that is
    /// farther from x until exactly k elements remain.
    ///
    /// # Approach
    /// 1. Initialize [left, right) = [0, n).
    /// 2. While the window exceeds k, compare distances at both ends.
    /// 3. Shrink from the farther side, preferring smaller values on ties.
    ///
    /// # Complexity
    /// - Time: O(n - k)
    /// - Space: O(k) for the result
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let (mut lo, mut hi) = (0, arr.len());
        while hi - lo > k {
            if x - arr[lo] <= arr[hi - 1] - x {
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        arr[lo..hi].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_left_bias() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
            vec![1, 2, 3, 4]
        );
    }
}
