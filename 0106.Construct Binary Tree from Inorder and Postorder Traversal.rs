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
    /// Recursive partitioning with inorder index map for postorder tree construction.
    ///
    /// # Intuition
    /// The last element of the postorder slice is the root. Its position
    /// in the inorder array divides into left and right subtrees. A hash
    /// map provides O(1) inorder position lookup.
    ///
    /// # Approach
    /// Build a value-to-index map for inorder. The root is at `postorder[j + n - 1]`.
    /// Compute the left subtree size from the inorder split, then recursively
    /// build left and right subtrees from their respective postorder slices.
    ///
    /// # Complexity
    /// - Time: O(n) — each node constructed once
    /// - Space: O(n) — hash map and recursion stack
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index_map: HashMap<i32, usize> =
            inorder.iter().enumerate().map(|(i, v)| (*v, i)).collect();

        fn dfs(
            postorder: &[i32],
            index_map: &HashMap<i32, usize>,
            in_start: usize,
            post_start: usize,
            size: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if size == 0 {
                return None;
            }
            let root_val = postorder[post_start + size - 1];
            let in_pos = index_map[&root_val];
            let left_size = in_pos - in_start;

            let left = dfs(postorder, index_map, in_start, post_start, left_size);
            let right = dfs(
                postorder,
                index_map,
                in_pos + 1,
                post_start + left_size,
                size - 1 - left_size,
            );
            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left,
                right,
            })))
        }

        let n = inorder.len();
        dfs(&postorder, &index_map, 0, 0, n)
    }
}
