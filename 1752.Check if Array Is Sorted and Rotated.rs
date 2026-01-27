impl Solution {
    /// Checks for at most one descending transition in circular order.
    ///
    /// # Intuition
    /// A sorted-and-rotated array has at most one point where the value decreases
    /// when viewed circularly. Count such transitions.
    ///
    /// # Approach
    /// 1. Count positions where `nums[i] > nums[(i+1) % n]`.
    /// 2. Return true if the count is at most 1.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let breaks = (0..n).filter(|&i| nums[i] > nums[(i + 1) % n]).count();
        breaks <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotated_sorted() {
        assert!(Solution::check(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn test_not_rotated() {
        assert!(!Solution::check(vec![2, 1, 3, 4]));
    }

    #[test]
    fn test_already_sorted() {
        assert!(Solution::check(vec![1, 2, 3]));
    }
}
