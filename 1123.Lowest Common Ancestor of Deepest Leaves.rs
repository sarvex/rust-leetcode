use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Finds LCA of deepest leaves using DFS with depth tracking
    ///
    /// #intuition
    /// - Track maximum depth to identify deepest level
    /// - Node is LCA when both subtrees reach maximum depth
    /// - Return both depth and LCA candidate in single pass
    ///
    /// #approach
    /// 1. Use DFS returning (depth, lca_candidate) tuple
    /// 2. Base case: None returns (0, None)
    /// 3. Recursively compute depths for left and right subtrees
    /// 4. If depths equal, current node is LCA candidate
    /// 5. Otherwise, return deeper subtree's result
    ///
    /// #complexity
    /// Time: O(n) where n is number of nodes, single DFS pass
    /// Space: O(h) where h is tree height, recursion stack only
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root).1
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        node.as_ref().map_or((0, None), |n| {
            let borrowed = n.borrow();
            let (left_depth, left_lca) = Self::dfs(borrowed.left.clone());
            let (right_depth, right_lca) = Self::dfs(borrowed.right.clone());
            drop(borrowed);

            match left_depth.cmp(&right_depth) {
                std::cmp::Ordering::Equal => (left_depth + 1, Some(n.clone())),
                std::cmp::Ordering::Greater => (left_depth + 1, left_lca),
                std::cmp::Ordering::Less => (right_depth + 1, right_lca),
            }
        })
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
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while i < vals.len() {
            if let Some(node) = queue.pop_front() {
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
        }
        Some(root)
    }

    #[test]
    fn test_lca_deepest_leaves_balanced() {
        let root = build_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
        ]);
        let result = Solution::lca_deepest_leaves(root);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_lca_deepest_leaves_single_node() {
        let root = build_tree(&[Some(1)]);
        let result = Solution::lca_deepest_leaves(root);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_lca_deepest_leaves_left_skewed() {
        let root = build_tree(&[Some(0), Some(1), None, Some(3), None]);
        let result = Solution::lca_deepest_leaves(root);
        assert_eq!(result.unwrap().borrow().val, 3);
    }
}
