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
    /// Flattens a binary tree to a linked list in-place using preorder collection.
    ///
    /// # Intuition
    /// Collect all nodes in preorder, then relink each node's right to the next
    /// node in the sequence while clearing left pointers.
    ///
    /// # Approach
    /// 1. Perform a preorder traversal collecting all node references.
    /// 2. Iterate through collected nodes, setting each node's right to the next
    ///    and left to None.
    ///
    /// # Complexity
    /// - Time: O(n) — two passes over all nodes
    /// - Space: O(n) — storage for node references
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        Self::preorder(&mut nodes, root);
        for i in 0..nodes.len() - 1 {
            let node = nodes[i].as_ref().unwrap();
            node.borrow_mut().left = None;
            node.borrow_mut().right = nodes[i + 1].clone();
        }
    }

    fn preorder(
        nodes: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root {
            nodes.push(root.clone());
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            Self::preorder(nodes, &left);
            Self::preorder(nodes, &right);
        }
    }
}
