impl Solution {
    /// Computes minimum seconds to fill all cups using greedy pairing.
    ///
    /// # Intuition
    /// Each second we can fill at most two different types. The bottleneck is either
    /// the largest amount alone or the ceiling of the total sum divided by two.
    ///
    /// # Approach
    /// 1. Sort the three amounts
    /// 2. If the largest exceeds the sum of the other two, it dictates the answer
    /// 3. Otherwise, ceiling of total / 2 is optimal
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort_unstable();
        let surplus = amount[0] + amount[1] - amount[2];
        if surplus <= 0 {
            amount[2]
        } else {
            (surplus + 1) / 2 + amount[2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
    }

    #[test]
    fn test_dominant_type() {
        assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::fill_cups(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_single_type() {
        assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
    }

    #[test]
    fn test_two_types_equal() {
        assert_eq!(Solution::fill_cups(vec![3, 3, 0]), 3);
    }
}
