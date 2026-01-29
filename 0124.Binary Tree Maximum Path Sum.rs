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
use std::rc::Rc;

pub struct Solution;

impl Solution {
    /// Finds the maximum path sum in a binary tree using post-order DFS.
    ///
    /// # Intuition
    /// At each node, the maximum path through it includes the node value plus
    /// the best non-negative contributions from left and right subtrees.
    /// Track the global maximum while returning the best single-branch gain.
    ///
    /// # Approach
    /// 1. Recursively compute the maximum gain from each subtree, clamped to zero.
    /// 2. At each node, update the global max with `val + left_gain + right_gain`.
    /// 3. Return `val + max(left_gain, right_gain)` as the single-branch contribution.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Self::dfs(&root, &mut result);
        result
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = Self::dfs(&node.left, result).max(0);
                let right = Self::dfs(&node.right, result).max(0);
                *result = (*result).max(node.val + left + right);
                node.val + left.max(right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            
            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_simple_tree() {
        // Test tree: [1,2,3]
        //     1
        //    / \
        //   2   3
        // Max path: 2 + 1 + 3 = 6
        let root = build_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::max_path_sum(root), 6);
    }

    #[test]
    fn test_negative_values() {
        // Test tree: [-10,9,20,null,null,15,7]
        //      -10
        //      /  \
        //     9    20
        //         /  \
        //        15   7
        // Max path: 15 + 20 + 7 = 42
        let root = build_tree(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_path_sum(root), 42);
    }

    #[test]
    fn test_single_node_negative() {
        // Test tree: [-3]
        // Max path: -3
        let root = build_tree(vec![Some(-3)]);
        assert_eq!(Solution::max_path_sum(root), -3);
    }

    #[test]
    fn test_all_negative() {
        // Test tree: [-2,-1,-3]
        //     -2
        //    /  \
        //   -1  -3
        // Max path: -1 (single node with max value)
        let root = build_tree(vec![Some(-2), Some(-1), Some(-3)]);
        assert_eq!(Solution::max_path_sum(root), -1);
    }

    #[test]
    fn test_complex_path() {
        // Test tree: [5,4,8,11,null,13,4,7,2,null,null,null,1]
        //         5
        //        / \
        //       4   8
        //      /   / \
        //     11  13  4
        //    / \       \
        //   7   2       1
        // Max path: 7 + 11 + 4 + 5 + 8 + 13 = 48
        let root = build_tree(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::max_path_sum(root), 48);
    }
}
