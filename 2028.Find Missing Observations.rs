impl Solution {
    /// Finds missing dice rolls to achieve a target mean.
    ///
    /// # Intuition
    /// Compute the required sum for the missing rolls. If it falls within
    /// [n, 6n], distribute evenly with the remainder spread across rolls.
    ///
    /// # Approach
    /// 1. Calculate the needed total for n missing rolls.
    /// 2. If infeasible (outside [n, 6n]), return empty.
    /// 3. Fill each roll with the base value, distributing the remainder.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n)
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let total_needed = (n + m) * mean - rolls.iter().sum::<i32>();

        if total_needed > n * 6 || total_needed < n {
            return vec![];
        }

        let base = total_needed / n;
        let extra = (total_needed % n) as usize;
        let mut result = vec![base; n as usize];
        for i in 0..extra {
            result[i] += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feasible() {
        let result = Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result.iter().sum::<i32>(), 6 * 4 - 12);
    }

    #[test]
    fn test_infeasible() {
        assert_eq!(Solution::missing_rolls(vec![1, 5, 6], 3, 4), vec![]);
    }

    #[test]
    fn test_single_missing() {
        let result = Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4);
        assert!(result.is_empty()); // Need 24 - 10 = 14, but max is 24
    }
}
