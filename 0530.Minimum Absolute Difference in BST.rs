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
    /// Finds the minimum absolute difference between any two BST nodes via in-order traversal.
    ///
    /// # Intuition
    /// In-order traversal of a BST yields sorted values. The minimum difference
    /// must occur between consecutive elements in this sorted sequence.
    ///
    /// # Approach
    /// 1. Perform in-order DFS, tracking the previous value.
    /// 2. Update the minimum difference at each node.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const INF: i32 = 1 << 30;
        let mut min_diff = INF;
        let mut prev = -INF;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min_diff: &mut i32, prev: &mut i32) {
            if let Some(rc) = node {
                let inner = rc.borrow();
                dfs(&inner.left, min_diff, prev);
                *min_diff = (*min_diff).min(inner.val - *prev);
                *prev = inner.val;
                dfs(&inner.right, min_diff, prev);
            }
        }

        dfs(&root, &mut min_diff, &mut prev);
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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

    #[test]
    fn test_case_1() {
        // BST: [4, 2, 6, 1, 3]
        //       4
        //      / \
        //     2   6
        //    / \
        //   1   3
        // In-order: 1, 2, 3, 4, 6 -> min diff = 1
        let root = create_tree(vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn test_case_2() {
        // BST: [1, 0, 48, None, None, 12, 49]
        //         1
        //        / \
        //       0   48
        //          /  \
        //         12  49
        // In-order: 0, 1, 12, 48, 49 -> min diff = 1
        let root = create_tree(vec![
            Some(1),
            Some(0),
            Some(48),
            None,
            None,
            Some(12),
            Some(49),
        ]);
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn test_case_3() {
        // Two node BST: [2, 1]
        //     2
        //    /
        //   1
        // In-order: 1, 2 -> min diff = 1
        let root = create_tree(vec![Some(2), Some(1)]);
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn test_case_4() {
        // BST: [236, 104, 701, None, 227, None, 911]
        //       236
        //       / \
        //    104   701
        //       \     \
        //       227   911
        // In-order: 104, 227, 236, 701, 911 -> min diff = 9 (236-227)
        let root = create_tree(vec![
            Some(236),
            Some(104),
            Some(701),
            None,
            Some(227),
            None,
            Some(911),
        ]);
        assert_eq!(Solution::get_minimum_difference(root), 9);
    }

    #[test]
    fn test_case_5() {
        // Right-skewed BST: [1, None, 3, None, None, 2]
        //   1
        //    \
        //     3
        //    /
        //   2
        // Actually construct manually for clarity
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        three.borrow_mut().left = Some(two);
        root.borrow_mut().right = Some(three);
        // In-order: 1, 2, 3 -> min diff = 1
        assert_eq!(Solution::get_minimum_difference(Some(root)), 1);
    }
}
