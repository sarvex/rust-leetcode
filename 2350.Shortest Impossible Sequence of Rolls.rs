impl Solution {
    /// Finds the shortest sequence not formable as a subsequence of rolls.
    ///
    /// # Intuition
    /// Each time we collect all k distinct values scanning left to right, we can
    /// construct one more position of any target sequence. The answer is one more
    /// than the number of complete rounds.
    ///
    /// # Approach
    /// 1. Track seen values with a boolean vec
    /// 2. Count unique values in the current round
    /// 3. When count reaches k, increment rounds and reset
    /// 4. Answer is complete_rounds + 1
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through rolls
    /// - Space: O(k) — for the boolean tracking array
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut seen = vec![false; k + 1];
        let mut unique_count = 0;
        let mut complete_rounds = 0;

        for roll in rolls {
            let idx = roll as usize;
            if !seen[idx] {
                seen[idx] = true;
                unique_count += 1;
                if unique_count == k {
                    complete_rounds += 1;
                    seen.fill(false);
                    unique_count = 0;
                }
            }
        }

        complete_rounds + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::shortest_sequence(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::shortest_sequence(vec![1, 1, 2, 2], 2), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::shortest_sequence(vec![1, 1, 3, 2, 2, 2, 3, 3], 4),
            1
        );
    }

    #[test]
    fn test_single_roll_complete() {
        assert_eq!(Solution::shortest_sequence(vec![1], 1), 2);
    }

    #[test]
    fn test_single_roll_incomplete() {
        assert_eq!(Solution::shortest_sequence(vec![1], 2), 1);
    }

    #[test]
    fn test_multiple_complete_rounds() {
        assert_eq!(
            Solution::shortest_sequence(vec![1, 2, 3, 1, 2, 3, 1, 2, 3], 3),
            4
        );
    }

    #[test]
    fn test_boundary_case() {
        assert_eq!(
            Solution::shortest_sequence(vec![2, 1, 2, 1, 1, 1, 2, 2], 2),
            4
        );
    }
}
