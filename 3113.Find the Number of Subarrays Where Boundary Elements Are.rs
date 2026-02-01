impl Solution {
    /// Monotonic stack of boundary maxima.
    ///
    /// # Intuition
    /// A subarray is valid when both ends are equal and no element inside is larger.
    /// For a fixed right end, the left end must be a previous occurrence of the same value
    /// after the most recent greater element.
    ///
    /// # Approach
    /// Traverse left to right while keeping parallel decreasing stacks of values and counts.
    /// - Pop smaller values because a larger right boundary invalidates them.
    /// - If the top differs, push `(value, 0)` so the next increment counts the single element.
    /// - Increment the top count and add it to the answer.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut result = 0i64;
        let mut values: Vec<i32> = Vec::with_capacity(nums.len());
        let mut counts: Vec<i32> = Vec::with_capacity(nums.len());

        for value in nums {
            let mut len = values.len();
            while len > 0 && values[len - 1] < value {
                values.pop();
                counts.pop();
                len -= 1;
            }

            if len == 0 || values[len - 1] > value {
                values.push(value);
                counts.push(0);
                len += 1;
            }

            counts[len - 1] += 1;
            result += i64::from(counts[len - 1]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = Solution::number_of_subarrays(vec![1, 4, 3, 3, 2]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::number_of_subarrays(vec![3, 3, 3]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_3() {
        let result = Solution::number_of_subarrays(vec![1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_strictly_increasing() {
        let result = Solution::number_of_subarrays(vec![1, 2, 3, 4]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_repeated_with_gaps() {
        let result = Solution::number_of_subarrays(vec![2, 1, 2, 1, 2]);
        assert_eq!(result, 8);
    }
}
