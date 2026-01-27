impl Solution {
    /// Backtracking with duplicate skipping for combination sum with unique elements.
    ///
    /// # Intuition
    /// Unlike Combination Sum I, each candidate can be used at most once.
    /// Sorting and skipping consecutive duplicates at the same recursion level
    /// ensures all combinations are unique.
    ///
    /// # Approach
    /// Sort candidates. Use DFS advancing the start index by one at each level.
    /// Skip duplicates by checking if `candidates[j] == candidates[j-1]` when
    /// `j > start`. Prune when the remaining target is below the current candidate.
    ///
    /// # Complexity
    /// - Time: O(2^n) — each candidate is included or excluded
    /// - Space: O(n) — recursion depth
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
            for j in start..candidates.len() {
                if candidates[j] > remaining {
                    break;
                }
                if j > start && candidates[j] == candidates[j - 1] {
                    continue;
                }
                path.push(candidates[j]);
                dfs(candidates, j + 1, remaining - candidates[j], path, result);
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
        let mut result = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        result.sort();
        assert_eq!(
            result,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn all_same() {
        let result = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
        let mut result_sorted = result;
        result_sorted.sort();
        assert_eq!(result_sorted, vec![vec![1, 2, 2], vec![5]]);
    }

    #[test]
    fn no_combination() {
        assert_eq!(
            Solution::combination_sum2(vec![3, 5], 2),
            Vec::<Vec<i32>>::new()
        );
    }
}
