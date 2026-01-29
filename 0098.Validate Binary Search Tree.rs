// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode { val, left: None, right: None }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Inorder traversal with predecessor tracking for BST validation.
    ///
    /// # Intuition
    /// A valid BST has a strictly increasing inorder traversal. Tracking
    /// the previous value during inorder DFS detects any violation.
    ///
    /// # Approach
    /// Perform inorder DFS, maintaining a mutable `prev` option. After
    /// visiting the left subtree, check that the current value exceeds
    /// `prev`. Update `prev` and recurse right.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(n) — recursion stack for skewed trees
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
            if let Some(n) = node {
                let n = n.borrow();
                if !dfs(&n.left, prev) {
                    return false;
                }
                if prev.map_or(false, |p| p >= n.val) {
                    return false;
                }
                *prev = Some(n.val);
                dfs(&n.right, prev)
            } else {
                true
            }
        }

        dfs(&root, &mut None)
    }
}

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

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() {
            if let Some(node) = queue.pop_front() {
                if i < vals.len() {
                    if let Some(v) = vals[i] {
                        let left = Rc::new(RefCell::new(TreeNode::new(v)));
                        node.borrow_mut().left = Some(Rc::clone(&left));
                        queue.push_back(left);
                    }
                    i += 1;
                }
                if i < vals.len() {
                    if let Some(v) = vals[i] {
                        let right = Rc::new(RefCell::new(TreeNode::new(v)));
                        node.borrow_mut().right = Some(Rc::clone(&right));
                        queue.push_back(right);
                    }
                    i += 1;
                }
            }
        }
        Some(root)
    }

    #[test]
    fn test_valid_bst() {
        let root = build_tree(&[Some(2), Some(1), Some(3)]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_invalid_bst() {
        let root = build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn test_empty_tree() {
        assert!(Solution::is_valid_bst(None));
    }

    #[test]
    fn test_single_node() {
        let root = build_tree(&[Some(1)]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_equal_values_invalid() {
        let root = build_tree(&[Some(2), Some(2), Some(2)]);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn test_left_only_valid() {
        let root = build_tree(&[Some(2), Some(1)]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_right_only_valid() {
        let root = build_tree(&[Some(1), None, Some(2)]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_subtree_violation() {
        // Tree: 5 -> (4, 6) where 6 -> (3, 7)
        // The 3 under 6 violates BST property (3 < 5 but is in right subtree)
        let root = build_tree(&[Some(5), Some(4), Some(6), None, None, Some(3), Some(7)]);
        assert!(!Solution::is_valid_bst(root));
    }
}
