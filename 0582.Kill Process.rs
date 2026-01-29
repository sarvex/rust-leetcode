use std::collections::HashMap;

impl Solution {
    /// Finds all processes to kill using DFS on the process tree.
    ///
    /// # Intuition
    /// Build a parent-to-children adjacency map, then DFS from the target
    /// process to collect all descendants.
    ///
    /// # Approach
    /// 1. Build a hash map mapping each parent PID to its children.
    /// 2. DFS from the `kill` PID, collecting all reachable processes.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut children: HashMap<i32, Vec<i32>> = HashMap::new();
        for (&p, &pp) in pid.iter().zip(ppid.iter()) {
            children.entry(pp).or_default().push(p);
        }

        fn dfs(id: i32, children: &HashMap<i32, Vec<i32>>, result: &mut Vec<i32>) {
            result.push(id);
            if let Some(kids) = children.get(&id) {
                for &child in kids {
                    dfs(child, children, result);
                }
            }
        }

        let mut result = Vec::new();
        dfs(kill, &children, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut result = Solution::kill_process(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5);
        result.sort_unstable();
        assert_eq!(result, vec![5, 10]);
    }

    #[test]
    fn test_root() {
        let mut result = Solution::kill_process(vec![1], vec![0], 1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_kill_root() {
        // Kill root process, should kill all
        let mut result = Solution::kill_process(vec![1, 2, 3], vec![0, 1, 1], 1);
        result.sort_unstable();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_leaf_process() {
        // Kill a leaf process with no children
        let result = Solution::kill_process(vec![1, 2, 3, 4], vec![0, 1, 1, 2], 3);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_deep_tree() {
        // Chain: 1 -> 2 -> 3 -> 4 -> 5
        let mut result = Solution::kill_process(vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3, 4], 2);
        result.sort_unstable();
        assert_eq!(result, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_wide_tree() {
        // Process 1 has many children: 2, 3, 4, 5
        let mut result = Solution::kill_process(vec![1, 2, 3, 4, 5], vec![0, 1, 1, 1, 1], 1);
        result.sort_unstable();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
