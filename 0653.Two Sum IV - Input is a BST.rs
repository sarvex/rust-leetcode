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
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    /// Checks if two nodes in a BST sum to k using BFS with a complement set.
    ///
    /// # Intuition
    /// For each node, check if its complement (k - val) has been seen.
    /// BFS traversal visits all nodes systematically.
    ///
    /// # Approach
    /// 1. BFS through the tree.
    /// 2. For each node, check if its value is in the complement set.
    /// 3. Add (k - val) to the set.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if let Some(rc) = node {
                let inner = rc.borrow();
                if seen.contains(&inner.val) {
                    return true;
                }
                seen.insert(k - inner.val);
                queue.push_back(inner.left.clone());
                queue.push_back(inner.right.clone());
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_example_1() {
        // Tree: [5,3,6,2,4,null,7], k = 9
        //       5
        //      / \
        //     3   6
        //    / \   \
        //   2   4   7
        let root = create_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        assert_eq!(Solution::find_target(root, 9), true);
    }

    #[test]
    fn test_example_2() {
        // Tree: [5,3,6,2,4,null,7], k = 28
        //       5
        //      / \
        //     3   6
        //    / \   \
        //   2   4   7
        let root = create_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        assert_eq!(Solution::find_target(root, 28), false);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1], k = 2
        let root = create_tree(&[Some(1)]);
        assert_eq!(Solution::find_target(root, 2), false);
    }

    #[test]
    fn test_two_nodes_sum_found() {
        // Tree: [2,1,3], k = 4
        //       2
        //      / \
        //     1   3
        let root = create_tree(&[Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::find_target(root, 4), true); // 1 + 3 = 4
    }

    #[test]
    fn test_negative_values() {
        // Tree: [0,-2,2,-4,-1,null,3], k = -1
        //       0
        //      / \
        //    -2   2
        //    / \   \
        //  -4  -1   3
        let root = create_tree(&[
            Some(0),
            Some(-2),
            Some(2),
            Some(-4),
            Some(-1),
            None,
            Some(3),
        ]);
        assert_eq!(Solution::find_target(root, -1), true); // -4 + 3 = -1
    }

    #[test]
    fn test_duplicate_values() {
        // Tree: [2,1,3,null,1], k = 2
        //       2
        //      / \
        //     1   3
        //      \
        //       1
        let root = create_tree(&[Some(2), Some(1), Some(3), None, Some(1)]);
        assert_eq!(Solution::find_target(root, 2), true); // 1 + 1 = 2
    }
}
