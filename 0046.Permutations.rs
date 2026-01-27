impl Solution {
    /// Backtracking with visited array for generating all permutations.
    ///
    /// # Intuition
    /// Each permutation is built position by position. At each position,
    /// try every unused element, mark it used, recurse for the next position,
    /// then backtrack by unmarking.
    ///
    /// # Approach
    /// Maintain a boolean visited array and a current permutation buffer.
    /// DFS through all positions, trying each unvisited element. When the
    /// buffer is full, record the permutation.
    ///
    /// # Complexity
    /// - Time: O(n × n!) — n! permutations, each taking O(n) to copy
    /// - Space: O(n) — recursion depth and auxiliary arrays
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
                if !visited[j] {
                    visited[j] = true;
                    path[depth] = nums[j];
                    dfs(nums, path, visited, result, depth + 1);
                    visited[j] = false;
                }
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
    fn three_elements() {
        let mut result = Solution::permute(vec![1, 2, 3]);
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn two_elements() {
        let mut result = Solution::permute(vec![0, 1]);
        result.sort();
        assert_eq!(result, vec![vec![0, 1], vec![1, 0]]);
    }
}
