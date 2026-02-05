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
    /// Finds minimum difference between any two BST nodes via in-order traversal.
    ///
    /// # Intuition
    /// In-order traversal of a BST yields sorted values. The minimum difference
    /// must occur between consecutive elements in this sorted sequence.
    ///
    /// # Approach
    /// 1. Perform in-order DFS (left → node → right)
    /// 2. Track the previous node's value using `Option<i32>` for clean first-node handling
    /// 3. At each node, compute difference from previous and update minimum
    /// 4. Use references instead of cloning to avoid unnecessary allocations
    ///
    /// # Complexity
    /// - Time: O(n) - visits each node exactly once
    /// - Space: O(h) where h is tree height for recursion stack
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min_diff: &mut i32, prev: &mut Option<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                dfs(&n.left, min_diff, prev);
                if let Some(p) = *prev {
                    *min_diff = (*min_diff).min(n.val - p);
                }
                *prev = Some(n.val);
                dfs(&n.right, min_diff, prev);
            }
        }

        let mut min_diff = i32::MAX;
        let mut prev = None;
        dfs(&root, &mut min_diff, &mut prev);
        min_diff
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

    #[test]
    fn test_basic_bst() {
        // Tree: [4, 2, 6, 1, 3]
        //       4
        //      / \
        //     2   6
        //    / \
        //   1   3
        let root = build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3)]);
        assert_eq!(Solution::min_diff_in_bst(root), 1);
    }

    #[test]
    fn test_small_bst() {
        // Tree: [1, 0, 48, null, null, 12, 49]
        //       1
        //      / \
        //     0   48
        //        /  \
        //       12  49
        let root = build_tree(&[Some(1), Some(0), Some(48), None, None, Some(12), Some(49)]);
        assert_eq!(Solution::min_diff_in_bst(root), 1);
    }

    #[test]
    fn test_linear_bst() {
        // Tree: [90, 69, null, 49, null, 52]
        //       90
        //      /
        //     69
        //    /
        //   49
        //    \
        //     52
        let root = build_tree(&[
            Some(90),
            Some(69),
            None,
            Some(49),
            None,
            None,
            None,
            None,
            None,
            Some(52),
        ]);
        assert_eq!(Solution::min_diff_in_bst(root), 3); // 52-49=3
    }

    #[test]
    fn test_two_node_tree() {
        // Tree: [5, 3]
        //       5
        //      /
        //     3
        let root = build_tree(&[Some(5), Some(3)]);
        assert_eq!(Solution::min_diff_in_bst(root), 2);
    }

    #[test]
    fn test_single_path_tree() {
        // Tree: [10, 5, null, 2]
        //       10
        //      /
        //     5
        //    /
        //   2
        let root = build_tree(&[Some(10), Some(5), None, Some(2)]);
        assert_eq!(Solution::min_diff_in_bst(root), 3); // 5-2=3
    }
}
