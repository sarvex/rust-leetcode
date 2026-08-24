impl Solution {
    /// Determines if Alice wins the sum game using game theory analysis.
    ///
    /// # Intuition
    /// The game reduces to a mathematical condition. If the total number of '?'
    /// is odd, Alice always wins because she gets the last move and can break
    /// any balance. If even, Bob wins only when the digit-sum difference plus
    /// the contribution from optimally-placed '?' pairs equals zero.
    ///
    /// # Approach
    /// 1. Split the string into two halves.
    /// 2. Compute `diff = sum_left - sum_right` and count '?' in each half.
    /// 3. If total '?' count is odd, Alice wins immediately (return true).
    /// 4. Otherwise, each pair of '?' on the same side contributes a net 9 to
    ///    the difference (one player adds 9, the other adds 0, net effect is 9
    ///    per pair toward balancing). Check if `diff + 9 * (q1 - q2) / 2 == 0`.
    ///    If so Bob wins; otherwise Alice wins.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn sum_game(num: String) -> bool {
        let n = num.len();
        let half = n / 2;
        let bytes = num.as_bytes();

        let mut diff: i64 = 0;
        let mut q1: i64 = 0;
        let mut q2: i64 = 0;

        for (i, &b) in bytes.iter().enumerate() {
            if b == b'?' {
                if i < half {
                    q1 += 1;
                } else {
                    q2 += 1;
                }
            } else {
                let digit = i64::from(b - b'0');
                if i < half {
                    diff += digit;
                } else {
                    diff -= digit;
                }
            }
        }

        // Odd total '?' means Alice has one extra move — she always wins
        if (q1 + q2) % 2 == 1 {
            return true;
        }

        // Each unmatched pair of '?' on one side contributes 9 to the difference.
        // A '?' in the left half adds to left sum, a '?' in the right subtracts.
        // With optimal play, each pair of same-side '?' nets 9 (one player picks 9,
        // the opponent picks 0). Bob wins iff the adjusted difference is zero.
        // diff + 9 * (q1 - q2) / 2 == 0
        diff * 2 + 9 * (q1 - q2) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_question_marks_equal() {
        assert!(!Solution::sum_game("5023".to_string()));
    }

    #[test]
    fn test_alice_wins_odd_questions() {
        assert!(Solution::sum_game("25??".to_string()));
    }

    #[test]
    fn test_bob_wins_balanced() {
        assert!(!Solution::sum_game("?3295???".to_string()));
    }

    #[test]
    fn test_all_question_marks_even() {
        // "????" -> 4 '?', q1=2, q2=2, diff=0 => diff*2 + 9*(2-2)=0 => Bob wins
        assert!(!Solution::sum_game("????".to_string()));
    }

    #[test]
    fn test_single_question_mark_left() {
        // "?0" -> odd total '?' => Alice wins
        assert!(Solution::sum_game("?0".to_string()));
    }

    #[test]
    fn test_single_question_mark_right() {
        // "0?" -> odd total '?' => Alice wins
        assert!(Solution::sum_game("0?".to_string()));
    }

    #[test]
    fn test_large_diff_alice_wins() {
        // "90" -> diff = 9-0 = 9, no '?' => sums unequal => Alice wins? 
        // Wait, no '?' means game is over. 9 != 0, so Alice wins.
        assert!(Solution::sum_game("90".to_string()));
    }

    #[test]
    fn test_no_question_equal_sums() {
        // "1221" -> 1+2 = 2+1 => Bob wins
        assert!(!Solution::sum_game("1221".to_string()));
    }
}
