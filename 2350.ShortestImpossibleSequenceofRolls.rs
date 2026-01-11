impl Solution {
    /// Shortest Impossible Sequence of Rolls
    ///
    /// # Intuition
    /// For any sequence of length L to be formable as a subsequence, we need enough
    /// "coverage" of all k values. Each time we can collect all k distinct values
    /// in order, we can construct one position of any target sequence.
    ///
    /// # Approach
    /// Use a greedy counting technique with a boolean array:
    /// 1. Track seen values with a boolean vec (O(1) access, no hashing)
    /// 2. Count unique values seen in current round
    /// 3. When count reaches k, increment rounds and reset
    /// 4. Answer is (complete rounds) + 1
    ///
    /// # Complexity
    /// - Time: O(n) single pass through rolls
    /// - Space: O(k) for the boolean array
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
        let rolls = vec![4, 2, 1, 2, 3, 3, 2, 4, 1];
        let k = 4;
        assert_eq!(Solution::shortest_sequence(rolls, k), 3);
    }

    #[test]
    fn test_example_2() {
        let rolls = vec![1, 1, 2, 2];
        let k = 2;
        assert_eq!(Solution::shortest_sequence(rolls, k), 2);
    }

    #[test]
    fn test_example_3() {
        let rolls = vec![1, 1, 3, 2, 2, 2, 3, 3];
        let k = 4;
        assert_eq!(Solution::shortest_sequence(rolls, k), 1);
    }

    #[test]
    fn test_single_roll_complete() {
        let rolls = vec![1];
        let k = 1;
        assert_eq!(Solution::shortest_sequence(rolls, k), 2);
    }

    #[test]
    fn test_single_roll_incomplete() {
        let rolls = vec![1];
        let k = 2;
        assert_eq!(Solution::shortest_sequence(rolls, k), 1);
    }

    #[test]
    fn test_multiple_complete_rounds() {
        let rolls = vec![1, 2, 3, 1, 2, 3, 1, 2, 3];
        let k = 3;
        assert_eq!(Solution::shortest_sequence(rolls, k), 4);
    }

    #[test]
    fn test_boundary_case() {
        let rolls = vec![2, 1, 2, 1, 1, 1, 2, 2];
        let k = 2;
        assert_eq!(Solution::shortest_sequence(rolls, k), 4);
    }
}
