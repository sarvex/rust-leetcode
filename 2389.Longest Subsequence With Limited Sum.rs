impl Solution {
    /// Answers queries for longest subsequence with sum at most each query value.
    ///
    /// # Intuition
    /// Greedily pick the smallest elements first. Sort and compute prefix sums,
    /// then binary search each query against the prefix sums.
    ///
    /// # Approach
    /// 1. Sort elements and compute prefix sums in-place
    /// 2. For each query, binary search the prefix sums to find the maximum count
    ///
    /// # Complexity
    /// - Time: O(n log n + q log n)
    /// - Space: O(1) auxiliary (in-place prefix sums)
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        queries
            .iter()
            .map(|&q| match nums.binary_search(&q) {
                Ok(idx) => idx as i32 + 1,
                Err(idx) => idx as i32,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }

    #[test]
    fn test_exact_match() {
        assert_eq!(Solution::answer_queries(vec![1, 2, 3], vec![6]), vec![3]);
    }

    #[test]
    fn test_zero_query() {
        assert_eq!(Solution::answer_queries(vec![1, 2, 3], vec![0]), vec![0]);
    }
}
