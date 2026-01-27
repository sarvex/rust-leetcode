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
    /// Generates all possible full binary trees with n nodes using memoized recursion.
    ///
    /// # Intuition
    /// A full binary tree has 0 or 2 children at every node. For n nodes,
    /// split n-1 nodes between left and right subtrees in all valid ways.
    ///
    /// # Approach
    /// Recursively generate all full binary trees. For each split `(i, n-1-i)`,
    /// combine all left and right subtree combinations. Memoize results.
    ///
    /// # Complexity
    /// - Time: O(2^(n/2)) Catalan number growth
    /// - Space: O(n * 2^(n/2)) for memoization
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut memo: Vec<Option<Vec<Option<Rc<RefCell<TreeNode>>>>>> =
            vec![None; (n + 1) as usize];
        Self::build(n, &mut memo)
    }

    fn build(
        n: i32,
        memo: &mut [Option<Vec<Option<Rc<RefCell<TreeNode>>>>>],
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(ref cached) = memo[n as usize] {
            return cached.clone();
        }

        let mut result = Vec::new();
        if n == 1 {
            result.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        } else {
            for i in (1..n - 1).step_by(2) {
                let j = n - 1 - i;
                for left in Self::build(i, memo) {
                    for right in Self::build(j, memo) {
                        result.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }
        }

        memo[n as usize] = Some(result.clone());
        result
    }
}
