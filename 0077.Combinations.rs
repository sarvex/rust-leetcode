impl Solution {
    /// Backtracking inclusion/exclusion for generating all k-combinations from [1..n].
    ///
    /// # Intuition
    /// Each number is either included or excluded. DFS through numbers
    /// 1 to n, branching on inclusion, and collecting when the combination
    /// reaches size k.
    ///
    /// # Approach
    /// Recursive DFS starting from 1. At each number, include it and recurse,
    /// then exclude and recurse. Base cases: path length equals k (record),
    /// or current number exceeds n (backtrack).
    ///
    /// # Complexity
    /// - Time: O(C(n,k) × k) — generating all combinations
    /// - Space: O(k) — recursion depth and current path
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        fn dfs(start: i32, n: i32, k: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if path.len() == k {
                result.push(path.clone());
                return;
            }
            if start > n {
                return;
            }
            path.push(start);
            dfs(start + 1, n, k, path, result);
            path.pop();
            dfs(start + 1, n, k, path, result);
        }

        dfs(1, n, k as usize, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_choose_two() {
        let mut result = Solution::combine(4, 2);
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
    }

    #[test]
    fn n_equals_k() {
        assert_eq!(Solution::combine(3, 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn k_equals_one() {
        let mut result = Solution::combine(3, 1);
        result.sort();
        assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
    }
}
