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
    /// Recursive DFS for binary tree inorder traversal (left-root-right).
    ///
    /// # Intuition
    /// Inorder traversal visits the left subtree, then the root, then
    /// the right subtree. Recursion naturally follows this structure.
    ///
    /// # Approach
    /// DFS helper takes a reference to the current node and a mutable
    /// result vector. Recurse left, push the value, recurse right.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(n) — recursion stack for skewed trees
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                dfs(&n.left, result);
                result.push(n.val);
                dfs(&n.right, result);
            }
        }

        dfs(&root, &mut result);
        result
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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(vals.len());
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
    fn test_standard_case() {
        let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(Solution::inorder_traversal(None), vec![]);
    }

    #[test]
    fn test_single_node() {
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::inorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_left_skewed() {
        let root = build_tree(&[Some(3), Some(2), None, Some(1)]);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_right_skewed() {
        let root = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_balanced() {
        let root = build_tree(&[Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3]);
    }
}
