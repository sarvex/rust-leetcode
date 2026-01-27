impl Solution {
    /// Finds the longest subarray whose bitwise AND equals the array maximum.
    ///
    /// # Intuition
    /// The bitwise AND of a subarray can never exceed any element in it, so the
    /// maximum AND value is the array's maximum element. The answer is the longest
    /// consecutive run of that maximum element.
    ///
    /// # Approach
    /// 1. Find the maximum value in the array
    /// 2. Count the longest consecutive run of that maximum value
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        nums.iter()
            .fold((0, 0), |(ans, cnt), &x| {
                if x == mx {
                    (ans.max(cnt + 1), cnt + 1)
                } else {
                    (ans, 0)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::longest_subarray(vec![5]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_subarray(vec![3, 3, 3, 3]), 4);
    }

    #[test]
    fn test_max_at_edges() {
        assert_eq!(Solution::longest_subarray(vec![5, 1, 5]), 1);
    }
}
