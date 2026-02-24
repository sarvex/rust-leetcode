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
use std::rc::Rc;

impl Solution {
    /// Recursive structural and value comparison for tree equality.
    ///
    /// # Intuition
    /// Two trees are identical when both are empty, or both have the same
    /// root value with recursively identical left and right subtrees.
    ///
    /// # Approach
    /// Pattern match on both nodes. If both present, compare values and
    /// recurse on children. If both absent, return true. Mixed cases return false.
    ///
    /// # Complexity
    /// - Time: O(min(n, m)) — visits each node until a mismatch
    /// - Space: O(min(h1, h2)) — recursion depth is the shorter tree height
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let (a, b) = (a.borrow(), b.borrow());
                    a.val == b.val && dfs(&a.left, &b.left) && dfs(&a.right, &b.right)
                }
                _ => false,
            }
        }

        dfs(&p, &q)
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(vals.len());
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() {
            if let Some(node) = queue.pop_front() {
                if i < vals.len() {
                    if let Some(v) = vals[i] {
                        let left = Rc::new(RefCell::new(TreeNode::new(v)));
                        node.borrow_mut().left = Some(Rc::clone(&left));
                        queue.push_back(left);
                    }
                    i += 1;
                }
                if i < vals.len() {
                    if let Some(v) = vals[i] {
                        let right = Rc::new(RefCell::new(TreeNode::new(v)));
                        node.borrow_mut().right = Some(Rc::clone(&right));
                        queue.push_back(right);
                    }
                    i += 1;
                }
            }
        }
        Some(root)
    }

    #[test]
    fn test_same_trees() {
        let p = build_tree(&[Some(1), Some(2), Some(3)]);
        let q = build_tree(&[Some(1), Some(2), Some(3)]);
        assert!(Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_different_structure() {
        let p = build_tree(&[Some(1), Some(2)]);
        let q = build_tree(&[Some(1), None, Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_different_values() {
        let p = build_tree(&[Some(1), Some(2), Some(1)]);
        let q = build_tree(&[Some(1), Some(1), Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_both_empty() {
        assert!(Solution::is_same_tree(None, None));
    }

    #[test]
    fn test_one_empty() {
        let p = build_tree(&[Some(1)]);
        assert!(!Solution::is_same_tree(p, None));
    }

    #[test]
    fn test_single_node_same() {
        let p = build_tree(&[Some(1)]);
        let q = build_tree(&[Some(1)]);
        assert!(Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_single_node_different() {
        let p = build_tree(&[Some(1)]);
        let q = build_tree(&[Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }
}
