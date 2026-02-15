impl Solution {
    /// Returns the lexicographically smallest array of size n where absolute values
    /// form a permutation of 1..n and sum equals target.
    ///
    /// # Intuition
    /// Process numbers from largest to smallest, deciding the sign of each.
    /// To achieve lexicographically smallest result (most negative numbers first),
    /// we want larger numbers to be negative when possible.
    ///
    /// For each value `val` from n down to 1:
    /// - If we make it negative: we need `target + val` from remaining values
    /// - If we make it positive: we need `target - val` from remaining values
    ///
    /// Make `val` negative if `target + val` can be achieved with 1..(val-1)
    /// (i.e., `target + val <= sum(1..val-1)`), otherwise make it positive.
    ///
    /// # Approach
    /// 1. Check feasibility: target must be within [-max, max] and same parity as max.
    /// 2. Greedily assign signs from n down to 1, collecting negatives and positives.
    /// 3. Concatenate negatives (already in descending order) with reversed positives
    ///    (to get ascending order), producing lexicographically sorted result.
    ///
    /// # Complexity
    /// - Time: O(n) - single pass from n to 1
    /// - Space: O(n) - stores negatives and positives (can be optimized to O(1) with
    ///   pre-allocation and indices)
    pub fn lex_smallest_negated_perm(n: i32, target: i64) -> Vec<i32> {
        let max = n as i64 * (n as i64 + 1) / 2;
        
        // Check if target is achievable: within range and same parity as max
        if target.abs() > max || (max + target) % 2 == 1 {
            return vec![];
        }
        
        let mut neg: Vec<i32> = Vec::with_capacity(n as usize);
        let mut pos: Vec<i32> = Vec::with_capacity(n as usize);
        let mut target = target;
        
        // Greedy: process from largest to smallest
        for val in (1i64..=n as i64).rev() {
            // If we make val negative, we need target + val from remaining 1..(val-1)
            // Check if that's achievable: target + val <= sum(1..val-1) = val*(val-1)/2
            if target + val <= val * (val - 1) / 2 {
                neg.push(-val as i32);
                target += val;
            } else {
                pos.push(val as i32);
                target -= val;
            }
        }
        
        // neg is in descending order (most negative first)
        // pos is in descending order, reverse to get ascending
        // Combined: lexicographically smallest arrangement
        neg.extend(pos.iter().rev());
        neg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_solution(n: i32, target: i64, result: &[i32]) -> bool {
        if result.is_empty() {
            return true;
        }

        if result.len() != n as usize {
            return false;
        }

        let sum: i64 = result.iter().map(|&x| x as i64).sum();
        if sum != target {
            return false;
        }

        let mut abs_values: Vec<i32> = result.iter().map(|&x| x.abs()).collect();
        abs_values.sort();
        for i in 0..n {
            if abs_values[i as usize] != i + 1 {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_example_1() {
        let result = Solution::lex_smallest_negated_perm(3, 0);
        assert_eq!(result, vec![-3, 1, 2]);
        assert!(verify_solution(3, 0, &result));
    }

    #[test]
    fn test_example_2() {
        let result = Solution::lex_smallest_negated_perm(1, 10000000000);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_n1_positive() {
        let result = Solution::lex_smallest_negated_perm(1, 1);
        assert_eq!(result, vec![1]);
        assert!(verify_solution(1, 1, &result));
    }

    #[test]
    fn test_n1_negative() {
        let result = Solution::lex_smallest_negated_perm(1, -1);
        assert_eq!(result, vec![-1]);
        assert!(verify_solution(1, -1, &result));
    }

    #[test]
    fn test_n1_impossible() {
        let result = Solution::lex_smallest_negated_perm(1, 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_all_negative() {
        let result = Solution::lex_smallest_negated_perm(3, -6);
        assert_eq!(result, vec![-3, -2, -1]);
        assert!(verify_solution(3, -6, &result));
    }

    #[test]
    fn test_all_positive() {
        let result = Solution::lex_smallest_negated_perm(3, 6);
        assert_eq!(result, vec![1, 2, 3]);
        assert!(verify_solution(3, 6, &result));
    }

    #[test]
    fn test_n2_target_1() {
        let result = Solution::lex_smallest_negated_perm(2, 1);
        assert_eq!(result, vec![-1, 2]);
        assert!(verify_solution(2, 1, &result));
    }

    #[test]
    fn test_n2_target_neg1() {
        let result = Solution::lex_smallest_negated_perm(2, -1);
        assert_eq!(result, vec![-2, 1]);
        assert!(verify_solution(2, -1, &result));
    }

    #[test]
    fn test_n3_target_2() {
        let result = Solution::lex_smallest_negated_perm(3, 2);
        assert_eq!(result, vec![-2, 1, 3]);
        assert!(verify_solution(3, 2, &result));
    }

    #[test]
    fn test_large_n() {
        let result = Solution::lex_smallest_negated_perm(100000, 0);
        assert!(!result.is_empty());
        assert!(verify_solution(100000, 0, &result));
    }

    #[test]
    fn test_parity_mismatch() {
        let result = Solution::lex_smallest_negated_perm(2, 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_n4_target_0() {
        let result = Solution::lex_smallest_negated_perm(4, 0);
        assert!(verify_solution(4, 0, &result));
    }
}
