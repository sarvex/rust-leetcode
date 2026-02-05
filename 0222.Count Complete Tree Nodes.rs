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

impl Solution {
    /// Counts nodes in a complete binary tree using left-depth comparison.
    ///
    /// # Intuition
    /// In a complete binary tree, if left and right subtree depths are equal,
    /// the left subtree is perfect. Otherwise, the right subtree is perfect
    /// but one level shorter.
    ///
    /// # Approach
    /// 1. Compute left depth of both subtrees.
    /// 2. If equal, left subtree is perfect with `2^depth` nodes; recurse right.
    /// 3. If unequal, right subtree is perfect with `2^depth` nodes; recurse left.
    ///
    /// # Complexity
    /// - Time: O(log^2 n) — log n depth computations of O(log n) each
    /// - Space: O(log n) — recursion stack
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left_depth = Self::depth(&node.left);
                let right_depth = Self::depth(&node.right);
                if left_depth == right_depth {
                    Self::count_nodes(node.right.clone()) + (1 << left_depth)
                } else {
                    Self::count_nodes(node.left.clone()) + (1 << right_depth)
                }
            }
        }
    }

    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => Self::depth(&node.borrow().left) + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_complete_tree_6_nodes() {
        // Test tree: [1,2,3,4,5,6]
        //       1
        //      / \
        //     2   3
        //    / \ /
        //   4  5 6
        // Count: 6
        let root = build_tree(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        assert_eq!(Solution::count_nodes(root), 6);
    }

    #[test]
    fn test_empty_tree() {
        // Empty tree
        let root = build_tree(vec![]);
        assert_eq!(Solution::count_nodes(root), 0);
    }

    #[test]
    fn test_single_node() {
        // Test tree: [1]
        // Count: 1
        let root = build_tree(vec![Some(1)]);
        assert_eq!(Solution::count_nodes(root), 1);
    }

    #[test]
    fn test_perfect_tree_3_levels() {
        // Test tree: [1,2,3,4,5,6,7]
        //       1
        //      / \
        //     2   3
        //    / \ / \
        //   4  5 6  7
        // Count: 7 (2^3 - 1)
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        assert_eq!(Solution::count_nodes(root), 7);
    }

    #[test]
    fn test_perfect_tree_4_levels() {
        // Test tree: Perfect binary tree with 4 levels
        // Count: 15 (2^4 - 1)
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
        ]);
        assert_eq!(Solution::count_nodes(root), 15);
    }

    #[test]
    fn test_almost_perfect_tree() {
        // Test tree: [1,2,3,4,5,6,7,8,9,10,11]
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        //   / \ / \
        //  8  9 10 11
        // Count: 11
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
        ]);
        assert_eq!(Solution::count_nodes(root), 11);
    }

    #[test]
    fn test_two_levels() {
        // Test tree: [1,2,3]
        //     1
        //    / \
        //   2   3
        // Count: 3
        let root = build_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::count_nodes(root), 3);
    }
}
