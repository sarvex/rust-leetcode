use std::collections::HashMap;

impl Solution {
    /// Finds the indices of two numbers in the array that add up to the target
    ///
    /// # Arguments
    /// * `nums` - Vector of integers to search through
    /// * `target` - The target sum to find
    ///
    /// # Returns
    /// * Vector containing the indices of the two numbers that sum to target
    /// * Empty vector if no solution is found
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&j) = num_to_index.get(&complement) {
                return vec![j as i32, i as i32];
            }

            num_to_index.insert(num, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
