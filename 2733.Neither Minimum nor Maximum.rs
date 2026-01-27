impl Solution {
    /// Find any element that is neither the minimum nor maximum.
    ///
    /// # Intuition
    /// With 3+ elements, sorting the first three guarantees the middle one is
    /// neither global min nor max. For fewer than 3 elements, no such value exists.
    ///
    /// # Approach
    /// 1. If length < 3, return -1.
    /// 2. Take the first three elements, sort them, and return the middle value.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0..=2 => -1,
            _ => {
                let mut triple = [nums[0], nums[1], nums[2]];
                triple.sort_unstable();
                triple[1]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_middle_element() {
        assert_eq!(Solution::find_non_min_or_max(vec![3, 2, 1, 4]), 2);
    }

    #[test]
    fn too_few_elements() {
        assert_eq!(Solution::find_non_min_or_max(vec![1, 2]), -1);
    }

    #[test]
    fn exactly_three_elements() {
        assert_eq!(Solution::find_non_min_or_max(vec![5, 3, 9]), 5);
    }
}
