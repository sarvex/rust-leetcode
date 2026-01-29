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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    /// Recursive partitioning with inorder index map for tree construction.
    ///
    /// # Intuition
    /// The first element of the preorder slice is always the root. Its
    /// position in the inorder array divides the sequence into left and
    /// right subtrees. A hash map provides O(1) lookup for this position.
    ///
    /// # Approach
    /// Build a value-to-index map for the inorder array. Recursively
    /// construct subtrees by slicing the preorder array: left subtree
    /// takes the next `k - j` elements, right subtree takes the rest.
    ///
    /// # Complexity
    /// - Time: O(n) — each node constructed once with O(1) lookups
    /// - Space: O(n) — hash map and recursion stack
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index_map: HashMap<i32, usize> =
            inorder.iter().enumerate().map(|(i, v)| (*v, i)).collect();

        fn dfs(
            preorder: &[i32],
            index_map: &HashMap<i32, usize>,
            pre_start: usize,
            in_start: usize,
            size: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if size == 0 {
                return None;
            }
            let root_val = preorder[pre_start];
            let in_pos = index_map[&root_val];
            let left_size = in_pos - in_start;

            let mut root = TreeNode::new(root_val);
            root.left = dfs(preorder, index_map, pre_start + 1, in_start, left_size);
            root.right = dfs(
                preorder,
                index_map,
                pre_start + 1 + left_size,
                in_pos + 1,
                size - 1 - left_size,
            );
            Some(Rc::new(RefCell::new(root)))
        }

        dfs(&preorder, &index_map, 0, 0, preorder.len())
    }
}
