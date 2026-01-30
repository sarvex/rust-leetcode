
impl Solution {
    /// Counts unique k-diff pairs using a sorted two-pointer approach.
    ///
    /// # Intuition
    /// Sort the array and use two pointers to find pairs with difference k,
    /// skipping duplicates to count each unique pair once.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Use left and right pointers; advance based on the current difference.
    /// 3. Skip duplicate values after finding a valid pair.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut count = 0;
        let (mut left, mut right) = (0, 1);
        while right < n {
            let diff = nums[right] - nums[left];
            match diff.cmp(&k) {
                std::cmp::Ordering::Equal => {
                    count += 1;
                    right += 1;
                    while right < n && nums[right] == nums[right - 1] {
                        right += 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    right += 1;
                }
                std::cmp::Ordering::Greater => {
                    left += 1;
                    while left < right && left > 0 && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    if left == right {
                        right += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k2() {
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    }

    #[test]
    fn test_k1() {
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    }

    #[test]
    fn test_k0() {
        assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    }

    #[test]
    fn test_k0_multiple_duplicates() {
        // Multiple pairs with k=0: (1,1), (2,2), (3,3)
        assert_eq!(Solution::find_pairs(vec![1, 1, 2, 2, 3, 3], 0), 3);
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 10), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::find_pairs(vec![1, 3], 2), 1);
        assert_eq!(Solution::find_pairs(vec![1, 3], 1), 0);
    }

    #[test]
    fn test_all_same() {
        // Only one unique k=0 pair
        assert_eq!(Solution::find_pairs(vec![5, 5, 5, 5], 0), 1);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Solution::find_pairs(vec![-1, -2, -3], 1), 2);
    }
}
