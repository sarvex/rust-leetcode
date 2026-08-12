use std::collections::HashSet;

impl Solution {
    /// Find the smallest missing integer >= sum of the longest sequential prefix.
    ///
    /// # Intuition
    /// Scan the prefix until the sequential property breaks, accumulate its sum,
    /// then walk upward from that sum until we find a value absent from the array.
    ///
    /// # Approach
    /// 1. Extend the prefix as long as each element equals its predecessor + 1.
    /// 2. Compute the prefix sum along the way.
    /// 3. Load all array values into a HashSet for O(1) membership tests.
    /// 4. Increment a candidate starting at the prefix sum until it is not in the set.
    ///
    /// # Complexity
    /// - Time: O(n) — one pass for prefix + set construction, at most O(n) probe steps
    /// - Space: O(n) — HashSet storing all distinct values
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let prefix_sum: i32 = nums
            .windows(2)
            .take_while(|w| w[1] == w[0] + 1)
            .fold(nums[0], |sum, w| sum + w[1]);

        let set: HashSet<i32> = nums.into_iter().collect();

        let mut candidate = prefix_sum;
        while set.contains(&candidate) {
            candidate += 1;
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Sequential prefix [1,2,3] sums to 6; 6 is absent.
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
    }

    #[test]
    fn test_example_2() {
        // Sequential prefix [3,4,5] sums to 12; 12,13,14 present; 15 absent.
        assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }

    #[test]
    fn test_single_element() {
        // Prefix is just [5], sum = 5; 5 is in the array, so answer is 6.
        assert_eq!(Solution::missing_integer(vec![5]), 6);
    }

    #[test]
    fn test_no_extension() {
        // No sequential pair: prefix = [2], sum = 2; 2 is present, 3 absent.
        assert_eq!(Solution::missing_integer(vec![2, 1, 4]), 3);
    }

    #[test]
    fn test_full_array_sequential() {
        // Entire array [1,2,3,4,5] is sequential, sum = 15; 15 absent.
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_boundary_max_values() {
        // All 50s (non-sequential after first); prefix = [50], sum = 50;
        // 50 is in array, so answer is 51.
        assert_eq!(Solution::missing_integer(vec![50; 50]), 51);
    }
}
