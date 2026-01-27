impl Solution {
    /// Backtracking with duplicate pruning for permutations of a multiset.
    ///
    /// # Intuition
    /// Sorting the array groups duplicates together. At each recursion level,
    /// skip an element if it equals the previous one and the previous was not
    /// used in this level, ensuring each unique permutation is generated once.
    ///
    /// # Approach
    /// Sort the input. Use DFS with a visited array. At each depth, iterate
    /// candidates and skip duplicates via the condition
    /// `j > 0 && nums[j] == nums[j-1] && !visited[j-1]`.
    ///
    /// # Complexity
    /// - Time: O(n × n!) — worst case with all unique elements
    /// - Space: O(n) — recursion depth and auxiliary arrays
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut result = Vec::new();
        let mut path = vec![0; n];
        let mut visited = vec![false; n];

        fn dfs(
            nums: &[i32],
            path: &mut [i32],
            visited: &mut [bool],
            result: &mut Vec<Vec<i32>>,
            depth: usize,
        ) {
            if depth == nums.len() {
                result.push(path.to_vec());
                return;
            }
            for j in 0..nums.len() {
                if visited[j] || (j > 0 && nums[j] == nums[j - 1] && !visited[j - 1]) {
                    continue;
                }
                path[depth] = nums[j];
                visited[j] = true;
                dfs(nums, path, visited, result, depth + 1);
                visited[j] = false;
            }
        }

        dfs(&nums, &mut path, &mut visited, &mut result, 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_duplicates() {
        let mut result = Solution::permute_unique(vec![1, 1, 2]);
        result.sort();
        assert_eq!(result, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::permute_unique(vec![1, 1, 1]), vec![vec![1, 1, 1]]);
    }

    #[test]
    fn all_unique() {
        let mut result = Solution::permute_unique(vec![1, 2, 3]);
        result.sort();
        assert_eq!(result.len(), 6);
    }
}
