impl Solution {
    /// Finds all combinations of k numbers from 1-9 that sum to n using backtracking.
    ///
    /// # Intuition
    /// Explore combinations in ascending order to avoid duplicates. Prune when
    /// the running sum exceeds the target or the combination grows too large.
    ///
    /// # Approach
    /// 1. Backtrack from digit `start` through 9.
    /// 2. Add the current digit and recurse with the next start.
    /// 3. If the combination has k elements summing to n, record it.
    /// 4. Prune if sum exceeds target or path exceeds k elements.
    ///
    /// # Complexity
    /// - Time: O(C(9, k)) — bounded by choosing k from 9
    /// - Space: O(k) recursion depth
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        Self::backtrack(1, n, k as usize, &mut path, &mut result);
        result
    }

    fn backtrack(
        start: i32,
        remaining: i32,
        k: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remaining < 0 || path.len() > k {
            return;
        }
        if remaining == 0 && path.len() == k {
            result.push(path.clone());
            return;
        }
        for digit in start..=9 {
            path.push(digit);
            Self::backtrack(digit + 1, remaining - digit, k, path, result);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_numbers_sum_seven() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }

    #[test]
    fn three_numbers_sum_nine() {
        let mut result = Solution::combination_sum3(3, 9);
        result.sort();
        assert_eq!(result, vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);
    }

    #[test]
    fn impossible_combination() {
        assert!(Solution::combination_sum3(4, 1).is_empty());
    }
}
