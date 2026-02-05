impl Solution {
    /// Binary search for insertion position in a sorted array.
    ///
    /// # Intuition
    /// The insert position is the first index where `nums[index] >= target`.
    /// This is exactly the lower bound, computable with standard binary search.
    ///
    /// # Approach
    /// Maintain `[left, right)` bounds. Narrow by checking the midpoint:
    /// if `nums[mid] >= target`, the answer is at or before `mid`; otherwise
    /// it is after `mid`. Return `left` when the bounds converge.
    ///
    /// # Complexity
    /// - Time: O(log n) — standard binary search
    /// - Space: O(1) — scalar pointers only
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.partition_point(|&x| x < target) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_present() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn target_between() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn target_beyond_end() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn target_before_start() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
