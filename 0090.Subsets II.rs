impl Solution {
    /// Backtracking with duplicate skipping for subsets of a multiset.
    ///
    /// # Intuition
    /// Sorting groups duplicates together. When excluding an element,
    /// skip all subsequent duplicates at the same level to avoid
    /// generating identical subsets.
    ///
    /// # Approach
    /// Sort the array. DFS includes the current element and recurses,
    /// then skips all consecutive duplicates before recursing without
    /// inclusion. At the end of the array, record the current subset.
    ///
    /// # Complexity
    /// - Time: O(n × 2^n) — at most 2^n subsets
    /// - Space: O(n) — recursion depth and current path
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();

        fn dfs(index: usize, nums: &[i32], path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if index >= nums.len() {
                result.push(path.clone());
                return;
            }
            path.push(nums[index]);
            dfs(index + 1, nums, path, result);
            path.pop();

            let mut skip = index;
            while skip + 1 < nums.len() && nums[skip + 1] == nums[skip] {
                skip += 1;
            }
            dfs(skip + 1, nums, path, result);
        }

        dfs(0, &nums, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_duplicates() {
        let mut result = Solution::subsets_with_dup(vec![1, 2, 2]);
        for s in &mut result {
            s.sort();
        }
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );
    }

    #[test]
    fn no_duplicates() {
        let mut result = Solution::subsets_with_dup(vec![0]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![0]]);
    }

    #[test]
    fn all_same() {
        let mut result = Solution::subsets_with_dup(vec![1, 1, 1]);
        for s in &mut result {
            s.sort();
        }
        result.sort();
        assert_eq!(result, vec![vec![], vec![1], vec![1, 1], vec![1, 1, 1]]);
    }
}
