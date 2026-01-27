impl Solution {
    /// Divides players into pairs of equal total skill and returns the sum of chemistry.
    ///
    /// # Intuition
    /// Sorting allows pairing the weakest with the strongest. If all pairs share
    /// the same total skill, the pairing is valid; otherwise it's impossible.
    ///
    /// # Approach
    /// 1. Sort skills in ascending order
    /// 2. Pair from both ends inward, verifying each pair sums to the same target
    /// 3. Accumulate chemistry (product of each pair)
    ///
    /// # Complexity
    /// - Time: O(n log n) — sorting dominates
    /// - Space: O(log n) — sort stack
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        skill.sort_unstable();
        let n = skill.len();
        let target = skill[0] + skill[n - 1];

        (0..n / 2)
            .try_fold(0i64, |acc, i| {
                if skill[i] + skill[n - 1 - i] == target {
                    Some(acc + (skill[i] as i64) * (skill[n - 1 - i] as i64))
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pairing() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    }

    #[test]
    fn test_equal_skills() {
        assert_eq!(Solution::divide_players(vec![3, 4]), 12);
    }

    #[test]
    fn test_invalid_pairing() {
        assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
    }
}
