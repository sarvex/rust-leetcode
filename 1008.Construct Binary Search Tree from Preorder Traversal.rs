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
    /// Constructs a BST from preorder traversal using binary search partitioning.
    ///
    /// # Intuition
    /// The first element is the root. Elements smaller go left, larger go right.
    /// Binary search finds the partition point efficiently.
    ///
    /// # Approach
    /// Recursively take the first element as root, binary search for the split
    /// between left and right subtree elements, then recurse on both halves.
    ///
    /// # Complexity
    /// - Time: O(n log n) average with binary search
    /// - Space: O(n) recursion stack
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], lo: usize, hi: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if lo > hi {
                return None;
            }
            let root = Rc::new(RefCell::new(TreeNode::new(preorder[lo])));
            let mut l = lo + 1;
            let mut r = hi + 1;
            while l < r {
                let mid = l + (r - l) / 2;
                if preorder[mid] > preorder[lo] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            let mut node = root.borrow_mut();
            if lo + 1 <= l.saturating_sub(1) {
                node.left = build(preorder, lo + 1, l - 1);
            }
            if l <= hi {
                node.right = build(preorder, l, hi);
            }
            drop(node);
            Some(root)
        }
        if preorder.is_empty() {
            return None;
        }
        build(&preorder, 0, preorder.len() - 1)
    }
}
