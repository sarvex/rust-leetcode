impl Solution {
    /// For each possible final character, keep all its occurrences and delete the rest; choose the
    /// character that minimizes total deletion cost (i.e. maximizes sum of kept costs).
    ///
    /// # Intuition
    /// The result must be a non-empty string of one repeated character. For a fixed character `c`,
    /// cost = total_cost − sum of cost[i] where s[i] == c. So we minimize cost by maximizing the
    /// sum of costs of the character we keep.
    ///
    /// # Approach
    /// Compute total cost, then for each character sum the costs at positions where s[i] equals
    /// that character. Answer = total − max(per-char sum). Use a 26-element array for lowercase
    /// letters.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_cost(s: String, cost: Vec<i32>) -> i64 {
        let total: i64 = cost.iter().map(|&c| c as i64).sum();
        let mut keep_sum = [0i64; 26];
        for (ch, &c) in s.bytes().zip(cost.iter()) {
            keep_sum[(ch - b'a') as usize] += c as i64;
        }
        let max_keep = keep_sum.into_iter().max().unwrap_or(0);
        total - max_keep
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::min_cost("aabaac".into(), vec![1, 2, 3, 4, 1, 10]),
            11
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_cost("abc".into(), vec![10, 5, 8]), 13);
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::min_cost("zzzzz".into(), vec![67, 67, 67, 67, 67]),
            0
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::min_cost("a".into(), vec![5]), 0);
    }

    #[test]
    fn test_two_different() {
        assert_eq!(Solution::min_cost("ab".into(), vec![1, 2]), 1);
    }
}
