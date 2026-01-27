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
    /// In-order traversal of both BSTs with two-pointer merge.
    ///
    /// # Intuition
    /// In-order traversal of a BST yields sorted elements. Merging two sorted
    /// arrays with a two-pointer technique produces the final sorted result
    /// in linear time.
    ///
    /// # Approach
    /// 1. In-order DFS both trees into sorted vectors
    /// 2. Merge the two sorted vectors using a two-pointer scan
    ///
    /// # Complexity
    /// - Time: O(m + n) for traversal and merge
    /// - Space: O(m + n) for the sorted vectors and result
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                inorder(&n.left, out);
                out.push(n.val);
                inorder(&n.right, out);
            }
        }

        let mut a = Vec::new();
        let mut b = Vec::new();
        inorder(&root1, &mut a);
        inorder(&root2, &mut b);

        let mut result = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);

        while i < a.len() && j < b.len() {
            if a[i] <= b[j] {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&a[i..]);
        result.extend_from_slice(&b[j..]);
        result
    }
}
