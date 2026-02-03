impl Solution {
    /// Count operations using sorted indices and wrap counts.
    ///
    /// # Intuition
    /// The smallest remaining value must be removed next. Each operation either rotates the
    /// front element to the back or removes it, so the process is equivalent to repeatedly
    /// rotating a circular list until the next-smallest index reaches the front.
    ///
    /// # Approach
    /// Sort indices by value to get the removal order. The first removal always costs 1, so the
    /// baseline is `n`. Every time the next index is to the left of the previous index, the
    /// front wraps around the remaining array, adding `(n - i)` extra operations for the
    /// `i`-th removal in this order.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_unstable_by_key(|&idx| nums[idx]);

        let mut operations = n as i64;
        for i in 1..n {
            if indices[i - 1] > indices[i] {
                operations += (n - i) as i64;
            }
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::count_operations_to_empty_array(vec![3, 4, -1]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::count_operations_to_empty_array(vec![1, 2, 4, 3]),
            5
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::count_operations_to_empty_array(vec![1, 2, 3]), 3);
    }

    #[test]
    fn reversed_order() {
        assert_eq!(Solution::count_operations_to_empty_array(vec![3, 2, 1]), 6);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::count_operations_to_empty_array(vec![42]), 1);
    }

    #[test]
    fn boundary_values() {
        assert_eq!(
            Solution::count_operations_to_empty_array(vec![i32::MAX, i32::MIN]),
            3,
        );
    }
}
