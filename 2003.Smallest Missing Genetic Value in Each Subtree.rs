impl Solution {
    /// Finds the smallest missing genetic value in each subtree.
    ///
    /// # Intuition
    /// Only the path from the node with value 1 to the root matters.
    /// All other nodes have answer 1. Walk up from the node with value 1,
    /// DFS-collecting all values at each step.
    ///
    /// # Approach
    /// 1. Build a child adjacency list from parents.
    /// 2. Find the node with genetic value 1.
    /// 3. Walk from that node to the root, DFS-collecting all subtree values.
    /// 4. Track the smallest missing value incrementally.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        let mut children: Vec<Vec<usize>> = vec![vec![]; n];
        let mut start = -1i32;
        for (i, &p) in parents.iter().enumerate() {
            if i > 0 {
                children[p as usize].push(i);
            }
            if nums[i] == 1 {
                start = i as i32;
            }
        }

        if start == -1 {
            return result;
        }

        let mut visited = vec![false; n];
        let mut seen = vec![false; n + 2];
        let mut missing = 2usize;

        fn dfs(
            node: usize,
            visited: &mut [bool],
            seen: &mut [bool],
            children: &[Vec<usize>],
            nums: &[i32],
        ) {
            if visited[node] {
                return;
            }
            visited[node] = true;
            if (nums[node] as usize) < seen.len() {
                seen[nums[node] as usize] = true;
            }
            for &child in &children[node] {
                dfs(child, visited, seen, children, nums);
            }
        }

        let mut current = start;
        while current != -1 {
            dfs(current as usize, &mut visited, &mut seen, &children, &nums);
            while seen[missing] {
                missing += 1;
            }
            result[current as usize] = missing as i32;
            current = parents[current as usize];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4]),
            vec![5, 1, 1, 1]
        );
    }

    #[test]
    fn test_no_value_one() {
        assert_eq!(
            Solution::smallest_missing_value_subtree(vec![-1, 0, 1], vec![2, 3, 4]),
            vec![1, 1, 1]
        );
    }
}
