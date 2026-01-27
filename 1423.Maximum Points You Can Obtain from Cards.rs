impl Solution {
    /// Sliding window over edge card selections.
    ///
    /// # Intuition
    /// Taking k cards from the edges is equivalent to choosing i cards from
    /// the left and k-i from the right. A sliding window starting with all
    /// k cards from the right, then progressively swapping right for left,
    /// finds the optimal split.
    ///
    /// # Approach
    /// 1. Start with sum of the last k cards
    /// 2. Slide: add left card, remove rightmost card from window
    /// 3. Track maximum sum across all k+1 configurations
    ///
    /// # Complexity
    /// - Time: O(k)
    /// - Space: O(1)
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let mut sum: i32 = card_points[n - k..].iter().sum();
        let mut best = sum;

        for i in 0..k {
            sum += card_points[i] - card_points[n - k + i];
            best = best.max(sum);
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_from_both_ends() {
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    }

    #[test]
    fn take_all_left() {
        assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    }

    #[test]
    fn single_card() {
        assert_eq!(Solution::max_score(vec![1, 1000, 1], 1), 1);
    }
}
