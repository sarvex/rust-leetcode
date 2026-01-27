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
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Converts a binary tree to a string with parentheses via preorder DFS.
    ///
    /// # Intuition
    /// Preorder traversal with parentheses for children. Empty left children
    /// need explicit "()" only when the right child exists.
    ///
    /// # Approach
    /// 1. DFS: append the node value.
    /// 2. If both children are absent, return.
    /// 3. Wrap the left subtree in parentheses (even if empty when right exists).
    /// 4. Wrap the right subtree in parentheses if present.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
            if let Some(rc) = node {
                let inner = rc.borrow();
                result.push_str(&inner.val.to_string());
                if inner.left.is_none() && inner.right.is_none() {
                    return;
                }
                result.push('(');
                dfs(&inner.left, result);
                result.push(')');
                if inner.right.is_some() {
                    result.push('(');
                    dfs(&inner.right, result);
                    result.push(')');
                }
            }
        }

        let mut result = String::new();
        dfs(&root, &mut result);
        result
    }
}
