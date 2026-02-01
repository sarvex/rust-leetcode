impl Solution {
    /// Returns the minimum possible sum of the first elements of three
    /// contiguous subarrays that partition `nums`.
    ///
    /// # Intuition
    /// The first subarray must start at index 0, so its cost is always `nums[0]`.
    /// For the remaining two subarrays, we need indices `i < j` such that
    /// `i` is the start of the second subarray and `j` is the start of the
    /// third. The total additional cost is `nums[i] + nums[j]`. Because all
    /// numbers are positive, minimizing the sum only depends on choosing the
    /// two smallest starting values that respect the ordering constraint.
    ///
    /// # Approach
    /// Scan from left to right, keeping the minimum value seen so far after
    /// index 0 (`min_prefix`). For each position `j > 0`, the best pair ending
    /// at `j` is `min_prefix + nums[j]`. Track the global minimum of this pair
    /// sum, then add `nums[0]` for the final answer.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut min_prefix = nums[1];
        let mut best_pair = i32::MAX;

        for &v in nums.iter().skip(2) {
            // Current j corresponds to v; i is the position of min_prefix.
            best_pair = best_pair.min(min_prefix + v);
            min_prefix = min_prefix.min(v);
        }

        nums[0] + best_pair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![1, 2, 3, 12];
        assert_eq!(Solution::minimum_cost(nums), 6);
    }

    #[test]
    fn example_two() {
        let nums = vec![5, 4, 3];
        assert_eq!(Solution::minimum_cost(nums), 12);
    }

    #[test]
    fn example_three() {
        let nums = vec![10, 3, 1, 1];
        assert_eq!(Solution::minimum_cost(nums), 12);
    }

    #[test]
    fn strictly_decreasing_middle() {
        let nums = vec![8, 7, 6, 5, 4];
        assert_eq!(Solution::minimum_cost(nums), 8 + 5 + 4);
    }
}
