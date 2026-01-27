impl Solution {
    /// Greedy selection of largest elements until subset sum exceeds remainder.
    ///
    /// # Intuition
    /// Sorting in descending order and greedily taking the largest elements
    /// ensures the fewest elements needed for the subsequence sum to exceed
    /// the remaining sum.
    ///
    /// # Approach
    /// 1. Sort array in descending order
    /// 2. Accumulate elements until the running sum exceeds total minus running sum
    /// 3. Return the collected elements
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) auxiliary (excluding output)
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let total: i32 = nums.iter().sum();
        let mut running = 0;
        nums.into_iter()
            .take_while(|&num| {
                running += num;
                running <= total - running + num
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
    }

    #[test]
    fn all_equal() {
        assert_eq!(
            Solution::min_subsequence(vec![4, 4, 7, 6, 7]),
            vec![7, 7, 6]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::min_subsequence(vec![6]), vec![6]);
    }
}
