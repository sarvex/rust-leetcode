impl Solution {
    /// Determines the best poker hand from five cards.
    ///
    /// # Intuition
    /// Check hand types in descending strength: Flush (all same suit), Three of a
    /// Kind (any rank appears 3+), Pair (any rank appears 2+), else High Card.
    ///
    /// # Approach
    /// 1. Check flush: all suits identical
    /// 2. Count rank frequencies; return best matching hand type
    ///
    /// # Complexity
    /// - Time: O(1) — at most 5 cards
    /// - Space: O(1) — fixed 14-entry frequency array
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|&s| s == suits[0]) {
            return "Flush".to_string();
        }

        let mut freq = [0u8; 14];
        let max_freq = ranks.iter().fold(0u8, |max_f, &r| {
            freq[r as usize] += 1;
            max_f.max(freq[r as usize])
        });

        match max_freq {
            3..=u8::MAX => "Three of a Kind",
            2 => "Pair",
            _ => "High Card",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flush() {
        assert_eq!(
            Solution::best_hand(vec![1, 2, 3, 4, 5], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush"
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        assert_eq!(
            Solution::best_hand(vec![3, 3, 3, 1, 2], vec!['a', 'b', 'c', 'd', 'a']),
            "Three of a Kind"
        );
    }

    #[test]
    fn test_pair() {
        assert_eq!(
            Solution::best_hand(vec![1, 2, 3, 4, 2], vec!['a', 'b', 'c', 'd', 'a']),
            "Pair"
        );
    }

    #[test]
    fn test_high_card() {
        assert_eq!(
            Solution::best_hand(vec![1, 2, 3, 4, 5], vec!['a', 'b', 'c', 'd', 'a']),
            "High Card"
        );
    }
}
