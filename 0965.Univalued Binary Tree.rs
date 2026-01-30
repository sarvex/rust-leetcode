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
    /// Determines if a binary tree is univalued (all nodes have the same value).
    ///
    /// # Intuition
    /// A univalued tree has every node containing an identical value. We can
    /// capture the root's value and perform a DFS traversal, short-circuiting
    /// as soon as a mismatch is found.
    ///
    /// # Approach
    /// 1. Extract the target value from the root node.
    /// 2. Recursively traverse using DFS, comparing each node's value.
    /// 3. Return `false` immediately if any node differs (short-circuit).
    /// 4. Use `map_or` for idiomatic Option handling in the base case.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(h) where h is the tree height (recursion stack)
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
            node.as_ref().map_or(true, |n| {
                let borrowed = n.borrow();
                borrowed.val == target
                    && dfs(&borrowed.left, target)
                    && dfs(&borrowed.right, target)
            })
        }

        root.as_ref().map_or(true, |r| dfs(&root, r.borrow().val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(vals.len());
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
    fn test_unival_tree() {
        // Tree: [1, 1, 1, 1, 1, null, 1]
        //       1
        //      / \
        //     1   1
        //    / \   \
        //   1   1   1
        let root = build_tree(&[Some(1), Some(1), Some(1), Some(1), Some(1), None, Some(1)]);
        assert!(Solution::is_unival_tree(root));
    }

    #[test]
    fn test_non_unival_tree() {
        // Tree: [2, 2, 2, 5, 2]
        //       2
        //      / \
        //     2   2
        //    / \
        //   5   2
        let root = build_tree(&[Some(2), Some(2), Some(2), Some(5), Some(2)]);
        assert!(!Solution::is_unival_tree(root));
    }

    #[test]
    fn test_single_node() {
        // Single node is always unival
        let root = build_tree(&[Some(42)]);
        assert!(Solution::is_unival_tree(root));
    }

    #[test]
    fn test_all_zeros() {
        // Tree with all zeros
        //       0
        //      / \
        //     0   0
        //    / \
        //   0   0
        let root = build_tree(&[Some(0), Some(0), Some(0), Some(0), Some(0)]);
        assert!(Solution::is_unival_tree(root));
    }

    #[test]
    fn test_mixed_values_early_fail() {
        // Tree: [1, 2, 1] - fails early at left child
        //       1
        //      / \
        //     2   1
        let root = build_tree(&[Some(1), Some(2), Some(1)]);
        assert!(!Solution::is_unival_tree(root));
    }

    #[test]
    fn test_deep_unival_tree() {
        // Deep tree with all same values
        //       7
        //      /
        //     7
        //    /
        //   7
        //  /
        // 7
        let root = build_tree(&[Some(7), Some(7), None, Some(7), None, None, None, Some(7)]);
        assert!(Solution::is_unival_tree(root));
    }
}
