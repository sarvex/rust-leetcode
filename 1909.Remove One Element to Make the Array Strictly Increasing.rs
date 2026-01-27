impl Solution {
    /// Checks if removing one element can make the array strictly increasing.
    ///
    /// # Intuition
    /// Find the first violation of strict ordering. Then check if removing
    /// either the element before or at the violation restores order.
    ///
    /// # Approach
    /// 1. Find the first index i where nums[i] >= nums[i+1].
    /// 2. Check if skipping index i yields a strictly increasing sequence.
    /// 3. Check if skipping index i+1 yields a strictly increasing sequence.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let is_valid = |skip: usize| -> bool {
            let mut prev = 0;
            for (i, &x) in nums.iter().enumerate() {
                if i == skip {
                    continue;
                }
                if prev >= x {
                    return false;
                }
                prev = x;
            }
            true
        };

        let violation = (0..nums.len() - 1).find(|&i| nums[i] >= nums[i + 1]);
        match violation {
            None => true,
            Some(i) => is_valid(i) || is_valid(i + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removable() {
        assert!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]));
    }

    #[test]
    fn test_not_removable() {
        assert!(!Solution::can_be_increasing(vec![2, 3, 1, 2]));
    }

    #[test]
    fn test_already_increasing() {
        assert!(Solution::can_be_increasing(vec![1, 2, 3]));
    }
}
