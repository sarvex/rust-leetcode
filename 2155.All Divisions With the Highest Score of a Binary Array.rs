impl Solution {
    /// Find all division indices yielding the highest score in a binary array.
    ///
    /// # Intuition
    /// The division score at index i equals the count of zeros in the left part
    /// plus the count of ones in the right part. Track running totals in one pass.
    ///
    /// # Approach
    /// 1. Initialize right-ones as the total sum, left-zeros as 0.
    /// 2. Iterate through each division point, updating counts and tracking the max.
    /// 3. Collect all indices matching the maximum score.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let mut left_zeros = 0;
        let mut right_ones: i32 = nums.iter().sum();
        let mut max_score = right_ones;
        let mut result = vec![0];

        for (i, &x) in nums.iter().enumerate() {
            left_zeros += x ^ 1;
            right_ones -= x;
            let score = left_zeros + right_ones;
            match score.cmp(&max_score) {
                std::cmp::Ordering::Equal => result.push((i + 1) as i32),
                std::cmp::Ordering::Greater => {
                    max_score = score;
                    result = vec![(i + 1) as i32];
                }
                std::cmp::Ordering::Less => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_array() {
        assert_eq!(Solution::max_score_indices(vec![0, 0, 1, 0]), vec![2, 4]);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::max_score_indices(vec![0, 0, 0]), vec![3]);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::max_score_indices(vec![1, 1, 1]), vec![0]);
    }
}
