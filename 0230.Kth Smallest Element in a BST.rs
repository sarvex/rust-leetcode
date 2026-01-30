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
    /// Finds the kth smallest element in a BST using early-terminating inorder DFS.
    ///
    /// # Intuition
    /// Inorder traversal of a BST yields elements in sorted order. Stop
    /// as soon as k elements have been collected.
    ///
    /// # Approach
    /// 1. Perform inorder DFS, collecting values into a vector.
    /// 2. Early-terminate once k elements are collected.
    /// 3. Return the kth element.
    ///
    /// # Complexity
    /// - Time: O(k) on average, O(n) worst case
    /// - Space: O(k) for collected values plus O(h) recursion stack
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let k = k as usize;
        let mut values = Vec::with_capacity(k);
        Self::inorder(root, &mut values, k);
        values[k - 1]
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>, k: usize) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            Self::inorder(node.left.take(), values, k);
            if values.len() >= k {
                return;
            }
            values.push(node.val);
            if values.len() >= k {
                return;
            }
            Self::inorder(node.right.take(), values, k);
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
        let mut queue = VecDeque::with_capacity(vals.len());
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
    fn test_kth_smallest_example1() {
        // BST: [3,1,4,null,2]
        //       3
        //      / \
        //     1   4
        //      \
        //       2
        // k=1 -> 1
        let root = build_tree(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(Solution::kth_smallest(root, 1), 1);
    }

    #[test]
    fn test_kth_smallest_example2() {
        // BST: [5,3,6,2,4,null,null,1]
        //         5
        //        / \
        //       3   6
        //      / \
        //     2   4
        //    /
        //   1
        // k=3 -> 3
        let root = build_tree(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::kth_smallest(root, 3), 3);
    }

    #[test]
    fn test_kth_smallest_single_node() {
        // BST: [1]
        // k=1 -> 1
        let root = build_tree(vec![Some(1)]);
        assert_eq!(Solution::kth_smallest(root, 1), 1);
    }

    #[test]
    fn test_kth_smallest_two_nodes() {
        // BST: [2,1]
        //       2
        //      /
        //     1
        // k=2 -> 2
        let root = build_tree(vec![Some(2), Some(1)]);
        assert_eq!(Solution::kth_smallest(root, 2), 2);
    }

    #[test]
    fn test_kth_smallest_right_skewed() {
        // BST: [1,null,2,null,3]
        //       1
        //        \
        //         2
        //          \
        //           3
        // k=2 -> 2
        let root = build_tree(vec![Some(1), None, Some(2), None, Some(3)]);
        assert_eq!(Solution::kth_smallest(root, 2), 2);
    }

    #[test]
    fn test_kth_smallest_left_skewed() {
        // BST: [3,2,null,1]
        //       3
        //      /
        //     2
        //    /
        //   1
        // k=3 -> 3
        let root = build_tree(vec![Some(3), Some(2), None, Some(1)]);
        assert_eq!(Solution::kth_smallest(root, 3), 3);
    }
}
