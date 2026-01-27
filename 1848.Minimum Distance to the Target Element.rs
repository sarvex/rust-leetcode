impl Solution {
    /// Finds the minimum distance from start to any occurrence of target.
    ///
    /// # Intuition
    /// Filter for target indices and compute the minimum absolute distance
    /// from the start index.
    ///
    /// # Approach
    /// 1. Enumerate elements and filter for those equal to target.
    /// 2. Map each matching index to its absolute distance from start.
    /// 3. Return the minimum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|&(_, &val)| val == target)
            .map(|(i, _)| (i as i32 - start).abs())
            .min()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    }

    #[test]
    fn test_target_at_start() {
        assert_eq!(Solution::get_min_distance(vec![1, 1, 1], 1, 0), 0);
    }

    #[test]
    fn test_multiple_targets() {
        assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 1, 2], 1, 2), 1);
    }
}
