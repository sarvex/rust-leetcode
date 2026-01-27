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
    /// Converts BST to Greater Sum Tree using reverse in-order traversal.
    ///
    /// # Intuition
    /// Reverse in-order (right → node → left) visits nodes in descending order.
    /// Accumulate the running sum and assign it to each node.
    ///
    /// # Approach
    /// DFS right subtree first, accumulating the sum. Update the current node's
    /// value with the running sum, then recurse into the left subtree.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
            match node {
                None => sum,
                Some(n) => {
                    let mut n = n.borrow_mut();
                    let right_sum = dfs(&mut n.right, sum);
                    n.val += right_sum;
                    dfs(&mut n.left, n.val)
                }
            }
        }
        dfs(&mut root, 0);
        root
    }
}
