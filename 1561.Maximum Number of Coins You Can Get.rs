impl Solution {
    /// Greedy sorting to maximize second-largest picks.
    ///
    /// # Intuition
    /// Sort piles and always let Bob take the smallest pile. In each triple,
    /// Alice takes the largest and you take the second largest. Skipping the
    /// smallest third of piles and taking every other pile from the top
    /// maximizes your total.
    ///
    /// # Approach
    /// 1. Sort piles
    /// 2. Skip the smallest `n/3` piles (Bob's picks)
    /// 3. From the remaining, take every other pile starting from the first
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        piles[piles.len() / 3..].iter().step_by(2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_piles() {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
    }

    #[test]
    fn equal_piles() {
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
    }

    #[test]
    fn large_range() {
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}
