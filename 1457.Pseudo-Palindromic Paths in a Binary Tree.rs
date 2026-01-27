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
    /// Bitmask DFS counting pseudo-palindromic root-to-leaf paths.
    ///
    /// # Intuition
    /// A path forms a pseudo-palindrome if at most one digit has odd frequency.
    /// XOR-ing a bitmask with `1 << val` toggles parity for each digit. At a
    /// leaf, the path is pseudo-palindromic if the mask has at most one bit set.
    ///
    /// # Approach
    /// 1. DFS with a bitmask tracking digit parity
    /// 2. At each node, toggle the bit for node value
    /// 3. At leaves, check `mask & (mask - 1) == 0` (power of two or zero)
    /// 4. Sum valid leaf counts
    ///
    /// # Complexity
    /// - Time: O(n) visiting every node once
    /// - Space: O(h) recursion depth
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mask: i32) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let mask = mask ^ (1 << n.val);
                    if n.left.is_none() && n.right.is_none() {
                        return if mask & (mask - 1) == 0 { 1 } else { 0 };
                    }
                    dfs(n.left.clone(), mask) + dfs(n.right.clone(), mask)
                }
                None => 0,
            }
        }

        dfs(root, 0)
    }
}
