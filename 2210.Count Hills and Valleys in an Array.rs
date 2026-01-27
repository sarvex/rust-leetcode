impl Solution {
    /// Count hills and valleys in an array, skipping consecutive equal elements.
    ///
    /// # Intuition
    /// A hill is a local maximum and a valley is a local minimum when compared
    /// to the previous distinct value and the next distinct value.
    ///
    /// # Approach
    /// Track the last distinct index `j`. For each position `i` that differs from
    /// its successor, check if nums[i] forms a hill or valley relative to nums[j]
    /// and nums[i+1].
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;
        let mut j = 0;

        for i in 1..n - 1 {
            if nums[i] == nums[i + 1] {
                continue;
            }
            if (nums[i] > nums[j] && nums[i] > nums[i + 1])
                || (nums[i] < nums[j] && nums[i] < nums[i + 1])
            {
                count += 1;
            }
            j = i;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_hills_valleys() {
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
    }

    #[test]
    fn constant_array() {
        assert_eq!(Solution::count_hill_valley(vec![5, 5, 5]), 0);
    }

    #[test]
    fn monotonic_increase() {
        assert_eq!(Solution::count_hill_valley(vec![1, 2, 3, 4]), 0);
    }
}
