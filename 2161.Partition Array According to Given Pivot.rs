impl Solution {
    /// In-place three-way partition using index cursors into a pre-allocated result.
    ///
    /// # Intuition
    /// We know the final positions of each partition upfront: count elements
    /// less than pivot to find where the pivot block starts, then place
    /// elements into the result in a single forward pass — no separate
    /// intermediate vectors needed.
    ///
    /// # Approach
    /// 1. One pass to count elements less than pivot (`less_count`) and equal
    ///    to pivot (`equal_count`), giving us the start offsets for each zone.
    /// 2. Second pass writes each element directly to its zone using three
    ///    advancing cursors: `lo` (less), `mid` (equal), `hi` (greater).
    ///
    /// This avoids three intermediate allocations and the concatenation step.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) — result buffer only; no auxiliary vectors
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let n = nums.len();
        let mut less_count = 0usize;
        let mut equal_count = 0usize;

        for &x in &nums {
            match x.cmp(&pivot) {
                std::cmp::Ordering::Less => less_count += 1,
                std::cmp::Ordering::Equal => equal_count += 1,
                std::cmp::Ordering::Greater => {}
            }
        }

        let mut result = vec![0; n];
        let mut lo = 0;
        let mut mid = less_count;
        let mut hi = less_count + equal_count;

        for &x in &nums {
            match x.cmp(&pivot) {
                std::cmp::Ordering::Less => {
                    result[lo] = x;
                    lo += 1;
                }
                std::cmp::Ordering::Equal => {
                    result[mid] = x;
                    mid += 1;
                }
                std::cmp::Ordering::Greater => {
                    result[hi] = x;
                    hi += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::pivot_array(vec![-3, 4, 3, 2], 2),
            vec![-3, 2, 4, 3]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::pivot_array(vec![5], 5), vec![5]);
    }

    #[test]
    fn test_all_equal_to_pivot() {
        assert_eq!(Solution::pivot_array(vec![3, 3, 3], 3), vec![3, 3, 3]);
    }

    #[test]
    fn test_pivot_at_boundaries() {
        assert_eq!(
            Solution::pivot_array(vec![1, 2, 3, 4, 5], 1),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(
            Solution::pivot_array(vec![1, 2, 3, 4, 5], 5),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(
            Solution::pivot_array(vec![-1_000_000, 0, 1_000_000], 0),
            vec![-1_000_000, 0, 1_000_000]
        );
    }
}
