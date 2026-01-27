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
