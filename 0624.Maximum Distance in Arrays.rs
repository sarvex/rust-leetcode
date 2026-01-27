impl Solution {
    /// Finds the maximum absolute difference between elements from different arrays.
    ///
    /// # Intuition
    /// Track the global min and max as we iterate. For each new array, the
    /// best distance uses either its min against the running max, or its max
    /// against the running min.
    ///
    /// # Approach
    /// 1. Initialize running min/max from the first array.
    /// 2. For each subsequent array, compute distances using its endpoints.
    /// 3. Update running min/max.
    ///
    /// # Complexity
    /// - Time: O(m) where m = number of arrays
    /// - Space: O(1)
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_val = arrays[0][0];
        let mut max_val = *arrays[0].last().unwrap();
        let mut result = 0;
        for arr in arrays.iter().skip(1) {
            let cur_min = arr[0];
            let cur_max = *arr.last().unwrap();
            result = result
                .max((cur_max - min_val).abs())
                .max((max_val - cur_min).abs());
            min_val = min_val.min(cur_min);
            max_val = max_val.max(cur_max);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
            4
        );
    }

    #[test]
    fn test_single_element_arrays() {
        assert_eq!(Solution::max_distance(vec![vec![1], vec![1]]), 0);
    }
}
