impl Solution {
    /// Circular array transformation using modular arithmetic.
    ///
    /// # Intuition
    /// Each element determines how many steps to move in the circular array.
    /// Using `rem_euclid` handles both positive and negative offsets correctly.
    ///
    /// # Approach
    /// For each index i, compute the target index as (i + nums[i]) mod n,
    /// using Euclidean remainder to handle negative values properly.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result array
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        nums.iter()
            .enumerate()
            .map(|(i, &val)| nums[(i as i32 + val).rem_euclid(n) as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::construct_transformed_array(vec![3, -2, 1, 1]),
            vec![1, 1, 1, 3]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::construct_transformed_array(vec![-1, 4, -1]),
            vec![-1, -1, 4]
        );
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(
            Solution::construct_transformed_array(vec![0, 0, 0]),
            vec![0, 0, 0]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::construct_transformed_array(vec![5]), vec![5]);
    }

    #[test]
    fn test_full_wrap() {
        assert_eq!(
            Solution::construct_transformed_array(vec![4, 4, 4, 4]),
            vec![4, 4, 4, 4]
        );
    }
}
