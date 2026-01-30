// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    /// Prunes subtrees that do not contain a 1 via post-order traversal.
    ///
    /// # Intuition
    /// A subtree should be removed if it contains no 1. Post-order traversal
    /// naturally prunes children before deciding on the parent.
    ///
    /// # Approach
    /// Recursively prune left and right children. If the current node is 0
    /// and both children are pruned (None), remove this node too.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        let left = Self::prune_tree(node.borrow_mut().left.take());
        let right = Self::prune_tree(node.borrow_mut().right.take());
        if node.borrow().val == 0 && left.is_none() && right.is_none() {
            return None;
        }
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(vals.len());
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

            if i < vals.len() && vals[i].is_some() {
                let left = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            if i < vals.len() && vals[i].is_some() {
                let right = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::with_capacity(16);
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(n) = node {
                result.push(Some(n.borrow().val));
                queue.push_back(n.borrow().left.clone());
                queue.push_back(n.borrow().right.clone());
            } else {
                result.push(None);
            }
        }

        // Remove trailing Nones
        while result.last() == Some(&None) {
            result.pop();
        }
        result
    }

    #[test]
    fn test_prune_zeros() {
        // Tree: [1, null, 0, 0, 1] => [1, null, 0, null, 1]
        //       1              1
        //        \      =>      \
        //         0              0
        //        / \              \
        //       0   1              1
        let root = build_tree(&[Some(1), None, Some(0), Some(0), Some(1)]);
        let pruned = Solution::prune_tree(root);
        let expected = vec![Some(1), None, Some(0), None, Some(1)];
        assert_eq!(tree_to_vec(pruned), expected);
    }

    #[test]
    fn test_prune_complex() {
        // Tree: [1, 0, 1, 0, 0, 0, 1] => [1, null, 1, null, 1]
        //       1              1
        //      / \      =>      \
        //     0   1              1
        //    / \ / \              \
        //   0  0 0  1              1
        let root = build_tree(&[
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(0),
            Some(0),
            Some(1),
        ]);
        let pruned = Solution::prune_tree(root);
        let expected = vec![Some(1), None, Some(1), None, Some(1)];
        assert_eq!(tree_to_vec(pruned), expected);
    }

    #[test]
    fn test_prune_all_zeros() {
        // Tree with all zeros should return None
        let root = build_tree(&[Some(0), Some(0), Some(0)]);
        let pruned = Solution::prune_tree(root);
        assert_eq!(pruned, None);
    }

    #[test]
    fn test_no_pruning_needed() {
        // Tree: [1, 1, 1] - no pruning needed
        //       1
        //      / \
        //     1   1
        let root = build_tree(&[Some(1), Some(1), Some(1)]);
        let pruned = Solution::prune_tree(root);
        let expected = vec![Some(1), Some(1), Some(1)];
        assert_eq!(tree_to_vec(pruned), expected);
    }

    #[test]
    fn test_single_one() {
        // Single node with value 1
        let root = build_tree(&[Some(1)]);
        let pruned = Solution::prune_tree(root);
        assert_eq!(tree_to_vec(pruned), vec![Some(1)]);
    }
}
