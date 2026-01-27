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
    /// Checks if two trees have the same leaf value sequence.
    ///
    /// # Intuition
    /// Collect leaves from each tree via DFS and compare the sequences.
    ///
    /// # Approach
    /// DFS both trees, collecting leaf values into vectors. Compare the
    /// two leaf vectors for equality.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n and m are tree sizes
    /// - Space: O(n + m) for leaf vectors and recursion stacks
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn collect_leaves(node: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                if n.left.is_none() && n.right.is_none() {
                    leaves.push(n.val);
                } else {
                    collect_leaves(&n.left, leaves);
                    collect_leaves(&n.right, leaves);
                }
            }
        }

        let (mut leaves1, mut leaves2) = (Vec::new(), Vec::new());
        collect_leaves(&root1, &mut leaves1);
        collect_leaves(&root2, &mut leaves2);
        leaves1 == leaves2
    }
}
