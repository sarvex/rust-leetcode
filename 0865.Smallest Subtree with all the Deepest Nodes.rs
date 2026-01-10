use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// Single-pass DFS returning depth and LCA of deepest nodes
    ///
    /// # Intuition
    /// The smallest subtree containing all deepest nodes is the Lowest Common
    /// Ancestor (LCA) of all deepest leaves. For any node, if its left and right
    /// subtrees have equal maximum depths, that node is the LCA of the deepest
    /// leaves in its subtree.
    ///
    /// # Approach
    /// 1. Perform post-order DFS traversal returning (depth, node) tuples
    /// 2. For each node, recursively compute left and right subtree depths
    /// 3. If left_depth > right_depth, propagate left result upward
    /// 4. If right_depth > left_depth, propagate right result upward
    /// 5. If depths are equal, current node becomes LCA at that depth
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes, single traversal
    /// - Space: O(h) where h is tree height, recursion stack only
    pub fn subtree_with_all_deepest(root: TreeLink) -> TreeLink {
        fn dfs(node: &TreeLink) -> (i32, TreeLink) {
            let Some(n) = node else {
                return (-1, None);
            };

            let borrowed = n.borrow();
            let (left_depth, left_lca) = dfs(&borrowed.left);
            let (right_depth, right_lca) = dfs(&borrowed.right);

            match left_depth.cmp(&right_depth) {
                std::cmp::Ordering::Greater => (left_depth + 1, left_lca),
                std::cmp::Ordering::Less => (right_depth + 1, right_lca),
                std::cmp::Ordering::Equal => (left_depth + 1, node.clone()),
            }
        }

        dfs(&root).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> TreeLink {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < vals.len() {
            if let Some(current) = queue.pop_front() {
                if i < vals.len() {
                    if let Some(val) = vals[i] {
                        let left = Rc::new(RefCell::new(TreeNode::new(val)));
                        current.borrow_mut().left = Some(Rc::clone(&left));
                        queue.push_back(left);
                    }
                }
                i += 1;

                if i < vals.len() {
                    if let Some(val) = vals[i] {
                        let right = Rc::new(RefCell::new(TreeNode::new(val)));
                        current.borrow_mut().right = Some(Rc::clone(&right));
                        queue.push_back(right);
                    }
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_example_1() {
        // Input: root = [3,5,1,6,2,0,8,null,null,7,4]
        // Output: [2,7,4]
        let root = build_tree(&[
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
        let result = Solution::subtree_with_all_deepest(root);
        assert_eq!(result.as_ref().unwrap().borrow().val, 2);
    }

    #[test]
    fn test_example_2() {
        // Input: root = [1]
        // Output: [1]
        let root = build_tree(&[Some(1)]);
        let result = Solution::subtree_with_all_deepest(root);
        assert_eq!(result.as_ref().unwrap().borrow().val, 1);
    }

    #[test]
    fn test_example_3() {
        // Input: root = [0,1,3,null,2]
        // Output: [2]
        let root = build_tree(&[Some(0), Some(1), Some(3), None, Some(2)]);
        let result = Solution::subtree_with_all_deepest(root);
        assert_eq!(result.as_ref().unwrap().borrow().val, 2);
    }

    #[test]
    fn test_balanced_deep_leaves() {
        // When multiple leaves at same deepest level share a common ancestor
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = Solution::subtree_with_all_deepest(root);
        // Nodes 4 and 5 are deepest, their LCA is node 2
        assert_eq!(result.as_ref().unwrap().borrow().val, 2);
    }
}
