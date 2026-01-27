impl Solution {
    /// Detects a 132 pattern using a monotonic stack traversed in reverse.
    ///
    /// # Intuition
    /// Scanning right-to-left, maintain a decreasing stack. When a larger
    /// element pops smaller ones, the popped value becomes the candidate
    /// for "2" (the middle value). Any element smaller than this candidate
    /// completes the 132 pattern.
    ///
    /// # Approach
    /// 1. Traverse from right to left.
    /// 2. If the current element is less than the candidate "2", return true.
    /// 3. Pop stack elements smaller than the current to update the candidate.
    /// 4. Push the current element.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut candidate = i32::MIN;
        let mut stack = Vec::new();
        for &val in nums.iter().rev() {
            if val < candidate {
                return true;
            }
            while stack.last().map_or(false, |&top| top < val) {
                candidate = stack.pop().unwrap();
            }
            stack.push(val);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn test_not_found() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_negative() {
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
    }
}
