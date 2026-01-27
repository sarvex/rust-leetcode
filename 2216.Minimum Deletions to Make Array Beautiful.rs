impl Solution {
    /// Minimum deletions to make an array beautiful (no equal adjacent even-indexed pairs).
    ///
    /// # Intuition
    /// After deletions, in a beautiful array every even-indexed element differs from
    /// the next. Greedily skip duplicate pairs and enforce even final length.
    ///
    /// # Approach
    /// 1. Walk through the array; when position i (relative to the beautiful array)
    ///    is even and nums[i] == nums[i+1], delete one element (increment count).
    /// 2. Ensure the final array length is even.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut deletions = 0;
        let mut i = 0;

        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                deletions += 1;
                i += 1;
            } else {
                i += 2;
            }
        }

        deletions += (n - deletions) % 2;
        deletions as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 3, 5]), 1);
    }

    #[test]
    fn already_beautiful() {
        assert_eq!(Solution::min_deletion(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::min_deletion(vec![1, 1, 1, 1]), 4);
    }
}
