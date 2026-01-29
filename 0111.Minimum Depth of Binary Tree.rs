use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    /// Finds minimum depth of a binary tree using recursive DFS with leaf-awareness.
    ///
    /// # Intuition
    /// Minimum depth is the shortest root-to-leaf path. A node with only one child
    /// is not a leaf, so we must continue down the non-null subtree.
    ///
    /// # Approach
    /// 1. Return 0 for an empty tree.
    /// 2. If one child is missing, recurse into the other child.
    /// 3. If both children exist, take the minimum of their depths.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (None, right) => 1 + Self::dfs(right),
                    (left, None) => 1 + Self::dfs(left),
                    (left, right) => 1 + Self::dfs(left).min(Self::dfs(right)),
                }
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
    fn test_min_depth_balanced_tree() {
        // Tree: [3,9,20,null,null,15,7]
        //       3
        //      / \
        //     9   20
        //        /  \
        //       15   7
        // Minimum depth = 2 (path: 3->9)
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::min_depth(root), 2);
    }

    #[test]
    fn test_min_depth_skewed_tree() {
        // Tree: [2,null,3,null,4,null,5,null,6]
        //     2
        //      \
        //       3
        //        \
        //         4
        //          \
        //           5
        //            \
        //             6
        // Minimum depth = 5
        let root = build_tree(&[Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)]);
        assert_eq!(Solution::min_depth(root), 5);
    }

    #[test]
    fn test_min_depth_single_node() {
        // Tree: [1]
        // Minimum depth = 1
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::min_depth(root), 1);
    }

    #[test]
    fn test_min_depth_empty_tree() {
        // Empty tree
        // Minimum depth = 0
        let root = build_tree(&[]);
        assert_eq!(Solution::min_depth(root), 0);
    }

    #[test]
    fn test_min_depth_left_skewed() {
        // Tree: [1,2,null,3,null,4]
        //       1
        //      /
        //     2
        //    /
        //   3
        //  /
        // 4
        // Minimum depth = 4
        let root = build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4)]);
        assert_eq!(Solution::min_depth(root), 4);
    }
}
