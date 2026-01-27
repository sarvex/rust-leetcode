impl Solution {
    /// Backtracking inclusion/exclusion for generating all subsets (power set).
    ///
    /// # Intuition
    /// Each element is independently included or excluded. DFS through all
    /// elements, branching at each, generates every possible subset.
    ///
    /// # Approach
    /// Recursive DFS from index 0. At each index, first recurse without
    /// including the element, then include it and recurse. At the end of
    /// the array, record the current subset.
    ///
    /// # Complexity
    /// - Time: O(n × 2^n) — 2^n subsets, each taking O(n) to copy
    /// - Space: O(n) — recursion depth and current path
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        fn dfs(index: usize, nums: &[i32], path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if index == nums.len() {
                result.push(path.clone());
                return;
            }
            dfs(index + 1, nums, path, result);
            path.push(nums[index]);
            dfs(index + 1, nums, path, result);
            path.pop();
        }

        dfs(0, &nums, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_elements() {
        let mut result = Solution::subsets(vec![1, 2, 3]);
        for s in &mut result {
            s.sort();
        }
        result.sort();
        assert_eq!(result.len(), 8);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn single_element() {
        let mut result = Solution::subsets(vec![0]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![0]]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::subsets(vec![]).len(), 1);
    }
}
