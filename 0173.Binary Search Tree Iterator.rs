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

/// BST iterator using flattened inorder traversal.
///
/// # Intuition
/// Pre-compute the inorder traversal and serve elements sequentially via an index.
///
/// # Approach
/// 1. Perform a full inorder traversal at construction, storing values in a vector.
/// 2. `next()` returns the current element and advances the index.
/// 3. `has_next()` checks if the index is within bounds.
///
/// # Complexity
/// - Time: O(n) construction, O(1) per `next`/`has_next`
/// - Space: O(n) for the values vector
struct BSTIterator {
    vals: Vec<i32>,
    index: usize,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut vals = Vec::new();
        Self::inorder(&root, &mut vals);
        Self { vals, index: 0 }
    }

    fn next(&mut self) -> i32 {
        let val = self.vals[self.index];
        self.index += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.index < self.vals.len()
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder(&node.left, result);
            result.push(node.val);
            Self::inorder(&node.right, result);
        }
    }
}
