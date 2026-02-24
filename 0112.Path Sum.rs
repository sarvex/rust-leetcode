use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    /// Recursive target subtraction for root-to-leaf path sum detection.
    ///
    /// # Intuition
    /// Subtract each node's value from the target as we descend. At a leaf,
    /// check if the remaining target equals the leaf's value.
    ///
    /// # Approach
    /// If the node is None, return false. If it is a leaf, check if its
    /// value equals the remaining target. Otherwise, recurse on both
    /// children with the reduced target.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited at most once
    /// - Space: O(h) — recursion depth equals tree height
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow();
                let remaining = target_sum - node.val;
                if node.left.is_none() && node.right.is_none() {
                    return remaining == 0;
                }
                Self::has_path_sum(node.left.clone(), remaining)
                    || Self::has_path_sum(node.right.clone(), remaining)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();

            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < values.len() {
                if let Some(val) = values[i] {
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
    fn test_has_path_sum_true() {
        // Tree: [5,4,8,11,null,13,4,7,2,null,null,null,1]
        //        5
        //       / \
        //      4   8
        //     /   / \
        //    11  13  4
        //   / \      \
        //  7   2      1
        // Target: 22 (path: 5->4->11->2)
        let root = build_tree(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        assert!(Solution::has_path_sum(root, 22));
    }

    #[test]
    fn test_has_path_sum_false() {
        // Tree: [1,2,3]
        //       1
        //      / \
        //     2   3
        // Target: 5 (no path sums to 5)
        let root = build_tree(&[Some(1), Some(2), Some(3)]);
        assert!(!Solution::has_path_sum(root, 5));
    }

    #[test]
    fn test_has_path_sum_empty_tree() {
        // Empty tree
        // Target: 0
        let root = build_tree(&[]);
        assert!(!Solution::has_path_sum(root, 0));
    }

    #[test]
    fn test_has_path_sum_single_node_true() {
        // Tree: [1]
        // Target: 1
        let root = build_tree(&[Some(1)]);
        assert!(Solution::has_path_sum(root, 1));
    }

    #[test]
    fn test_has_path_sum_single_node_false() {
        // Tree: [1]
        // Target: 2
        let root = build_tree(&[Some(1)]);
        assert!(!Solution::has_path_sum(root, 2));
    }

    #[test]
    fn test_has_path_sum_negative_values() {
        // Tree: [1,2,-3,-5,null,4,-2]
        //        1
        //       / \
        //      2   -3
        //     /     \
        //   -5       4
        //           /
        //         -2
        // Target: -1 (path: 1->-3->4->-2)
        let root = build_tree(&[
            Some(1),
            Some(2),
            Some(-3),
            Some(-5),
            None,
            Some(4),
            None,
            None,
            None,
            Some(-2),
        ]);
        assert!(Solution::has_path_sum(root, 0));
    }
}
