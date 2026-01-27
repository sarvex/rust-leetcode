impl Solution {
    /// Backtracking with sorted pruning for combination sum.
    ///
    /// # Intuition
    /// Sorting candidates allows early termination when the remaining target
    /// is smaller than the current candidate. Allowing repeated use of the
    /// same element (starting from the same index) naturally handles the
    /// unbounded selection constraint.
    ///
    /// # Approach
    /// Sort candidates. Use DFS from each index, subtracting the chosen
    /// value from the remaining target. When the target reaches zero, record
    /// the combination. Prune when the target drops below the current candidate.
    ///
    /// # Complexity
    /// - Time: O(n^(T/M)) — n candidates, T target, M minimum candidate
    /// - Space: O(T/M) — recursion depth bounded by target / minimum value
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result = Vec::new();

        fn dfs(
            candidates: &[i32],
            start: usize,
            remaining: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining == 0 {
                result.push(path.clone());
                return;
            }
            for i in start..candidates.len() {
                if candidates[i] > remaining {
                    break;
                }
                path.push(candidates[i]);
                dfs(candidates, i, remaining - candidates[i], path, result);
                path.pop();
            }
        }

        dfs(&candidates, 0, target, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        result.sort();
        assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn multiple_combinations() {
        let mut result = Solution::combination_sum(vec![2, 3, 5], 8);
        result.sort();
        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    #[test]
    fn no_combination() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}
