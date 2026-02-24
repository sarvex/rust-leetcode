use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

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

impl Solution {
    /// Finds LCA of deepest leaves using DFS with depth tracking.
    ///
    /// # Intuition
    /// The LCA of deepest leaves is the node where both subtrees reach the
    /// maximum depth. If one subtree is deeper, propagate that result upward.
    ///
    /// # Approach
    /// DFS returning `(depth, lca_candidate)`. If left and right depths are
    /// equal, the current node is the LCA. Otherwise return the deeper side.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (u32, Option<Rc<RefCell<TreeNode>>>) {
        node.as_ref().map_or((0, None), |n| {
            let borrowed = n.borrow();
            let (left_depth, left_lca) = Self::dfs(&borrowed.left);
            let (right_depth, right_lca) = Self::dfs(&borrowed.right);

            match left_depth.cmp(&right_depth) {
                Ordering::Equal => (left_depth + 1, Some(n.clone())),
                Ordering::Greater => (left_depth + 1, left_lca),
                Ordering::Less => (right_depth + 1, right_lca),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(values.len());
        queue.push_back(root.clone());

        let mut i = 1;
        while i < values.len() && !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if i < values.len() && values[i].is_some() {
                let left = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            if i < values.len() && values[i].is_some() {
                let right = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_basic_tree() {
        // Tree: [3,5,1,6,2,0,8,null,null,7,4]
        // Deepest leaves: 7, 4
        // LCA: 2
        let root = build_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        // Deepest leaves: 1
        // LCA: 1
        let root = build_tree(vec![Some(1)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_left_skewed() {
        // Tree: [1,2,null,3]
        // Deepest leaf: 3
        // LCA: 3
        let root = build_tree(vec![Some(1), Some(2), None, Some(3)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_two_deepest_leaves() {
        // Tree: [1,2,3,4,5]
        // Deepest leaves: 4, 5
        // LCA: 2
        let root = build_tree(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_empty_tree() {
        let result = Solution::lca_deepest_leaves(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_all_leaves_same_depth() {
        // Tree: [1,2,3]
        // Deepest leaves: 2, 3
        // LCA: 1
        let root = build_tree(vec![Some(1), Some(2), Some(3)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().val, 1);
    }
}
