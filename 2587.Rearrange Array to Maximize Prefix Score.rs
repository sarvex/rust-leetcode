impl Solution {
    /// Count maximum prefix score elements by sorting descending.
    ///
    /// # Intuition
    /// To maximize the number of positive prefix sums, sort in descending order
    /// so the largest values contribute first.
    ///
    /// # Approach
    /// 1. Sort in descending order
    /// 2. Accumulate prefix sums, counting while the sum remains positive
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));

        let mut prefix_sum: i64 = 0;
        nums.iter()
            .take_while(|x| {
                prefix_sum += x as i64;
                prefix_sum > 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed() {
        assert_eq!(Solution::max_score(vec![2, -1, 0, 1, -3, 3, -3]), 6);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(Solution::max_score(vec![-2, -3, 0]), 0);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(Solution::max_score(vec![1, 2, 3]), 3);
    }
}
