// Definition for a binary tree node.
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Trims a BST to only contain values in [low, high].
    ///
    /// # Intuition
    /// If a node's value is below low, its entire left subtree is also out of
    /// range — recurse on the right. Similarly for above high.
    ///
    /// # Approach
    /// 1. If val < low, return trimmed right subtree.
    /// 2. If val > high, return trimmed left subtree.
    /// 3. Otherwise, recursively trim both children.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(rc) => {
                let val = rc.borrow().val;
                if val < low {
                    let right = rc.borrow_mut().right.take();
                    Self::trim_bst(right, low, high)
                } else if val > high {
                    let left = rc.borrow_mut().left.take();
                    Self::trim_bst(left, low, high)
                } else {
                    let left = rc.borrow_mut().left.take();
                    let right = rc.borrow_mut().right.take();
                    rc.borrow_mut().left = Self::trim_bst(left, low, high);
                    rc.borrow_mut().right = Self::trim_bst(right, low, high);
                    Some(rc)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn create_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    fn tree_to_array(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(rc) = node {
                let inner = rc.borrow();
                result.push(Some(inner.val));
                queue.push_back(inner.left.clone());
                queue.push_back(inner.right.clone());
            } else {
                result.push(None);
            }
        }

        // Remove trailing None values
        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }

    #[test]
    fn test_example_1() {
        // Tree: [1,0,2], low = 1, high = 2
        //       1
        //      / \
        //     0   2
        // Expected: [1,null,2]
        let root = create_tree(&[Some(1), Some(0), Some(2)]);
        let result = Solution::trim_bst(root, 1, 2);
        let output = tree_to_array(result);
        assert_eq!(output, vec![Some(1), None, Some(2)]);
    }

    #[test]
    fn test_example_2() {
        // Tree: [3,0,4,null,2,null,null,1], low = 1, high = 3
        //       3
        //      / \
        //     0   4
        //      \
        //       2
        //      /
        //     1
        // Expected: [3,2,null,1]
        let root = create_tree(&[
            Some(3),
            Some(0),
            Some(4),
            None,
            Some(2),
            None,
            None,
            Some(1),
        ]);
        let result = Solution::trim_bst(root, 1, 3);

        // Verify the root is 3
        assert_eq!(result.as_ref().unwrap().borrow().val, 3);

        // Verify left child is 2
        let left = result.as_ref().unwrap().borrow().left.clone();
        assert_eq!(left.as_ref().unwrap().borrow().val, 2);

        // Verify 2's left child is 1
        let left_left = left.as_ref().unwrap().borrow().left.clone();
        assert_eq!(left_left.as_ref().unwrap().borrow().val, 1);

        // Verify right child is None
        let right = result.as_ref().unwrap().borrow().right.clone();
        assert!(right.is_none());
    }

    #[test]
    fn test_all_nodes_removed() {
        // Tree: [1], low = 2, high = 3
        // All nodes should be removed
        let root = create_tree(&[Some(1)]);
        let result = Solution::trim_bst(root, 2, 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_no_trimming_needed() {
        // Tree: [2,1,3], low = 1, high = 3
        //       2
        //      / \
        //     1   3
        // No trimming needed
        let root = create_tree(&[Some(2), Some(1), Some(3)]);
        let result = Solution::trim_bst(root, 1, 3);
        let output = tree_to_array(result);
        assert_eq!(output, vec![Some(2), Some(1), Some(3)]);
    }

    #[test]
    fn test_trim_complete_subtree() {
        // Tree: [5,3,7,1,4,6,8], low = 6, high = 8
        //       5
        //      / \
        //     3   7
        //    / \ / \
        //   1  4 6  8
        // Expected: [7,6,8]
        let root = create_tree(&[
            Some(5),
            Some(3),
            Some(7),
            Some(1),
            Some(4),
            Some(6),
            Some(8),
        ]);
        let result = Solution::trim_bst(root, 6, 8);

        // Verify root is 7
        assert_eq!(result.as_ref().unwrap().borrow().val, 7);

        // Verify left is 6 and right is 8
        let left = result.as_ref().unwrap().borrow().left.clone();
        let right = result.as_ref().unwrap().borrow().right.clone();
        assert_eq!(left.as_ref().unwrap().borrow().val, 6);
        assert_eq!(right.as_ref().unwrap().borrow().val, 8);
    }
}
