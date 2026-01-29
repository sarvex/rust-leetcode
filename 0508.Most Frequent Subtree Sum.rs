// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    /// Finds the most frequent subtree sums via DFS.
    ///
    /// # Intuition
    /// Each subtree has a sum (node value + sum of left subtree + sum of right subtree).
    /// We need to find which sum(s) appear most frequently across all subtrees.
    ///
    /// # Approach
    /// 1. Use DFS to compute each subtree sum and record it in a frequency map
    /// 2. Find the maximum frequency among all sums
    /// 3. Collect all sums that have this maximum frequency
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(n) for the frequency map and recursion stack
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, freq: &mut HashMap<i32, i32>) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    let sum = inner.val + dfs(&inner.left, freq) + dfs(&inner.right, freq);
                    *freq.entry(sum).or_insert(0) += 1;
                    sum
                }
            }
        }

        let mut freq = HashMap::new();
        dfs(&root, &mut freq);
        let max_count = freq.values().copied().max().unwrap_or(0);
        freq.into_iter()
            .filter(|(_, v)| *v == max_count)
            .map(|(k, _)| k)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            
            if i < vals.len() && vals[i].is_some() {
                let left = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            if i < vals.len() && vals[i].is_some() {
                let right = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_case_1() {
        // Tree: [5, 2, -3]
        //       5
        //      / \
        //     2  -3
        // Subtree sums: 2 (left leaf), -3 (right leaf), 4 (root: 5+2-3)
        let root = create_tree(vec![Some(5), Some(2), Some(-3)]);
        let mut result = Solution::find_frequent_tree_sum(root);
        result.sort_unstable();
        let mut expected = vec![2, -3, 4];
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        // Tree: [5, 2, -5]
        //       5
        //      / \
        //     2  -5
        // Subtree sums: 2 (left leaf), -5 (right leaf), 2 (root: 5+2-5)
        // Frequency: 2 appears twice, -5 appears once
        let root = create_tree(vec![Some(5), Some(2), Some(-5)]);
        let result = Solution::find_frequent_tree_sum(root);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_case_3() {
        // Single node tree
        let root = create_tree(vec![Some(10)]);
        let result = Solution::find_frequent_tree_sum(root);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_case_4() {
        // Empty tree
        let root = None;
        let result = Solution::find_frequent_tree_sum(root);
        assert!(result.is_empty());
    }

    #[test]
    fn test_case_5() {
        // Tree with all same values
        //       1
        //      / \
        //     1   1
        // Subtree sums: 1 (left leaf), 1 (right leaf), 3 (root: 1+1+1)
        let root = create_tree(vec![Some(1), Some(1), Some(1)]);
        let result = Solution::find_frequent_tree_sum(root);
        assert_eq!(result, vec![1]);
    }
}
