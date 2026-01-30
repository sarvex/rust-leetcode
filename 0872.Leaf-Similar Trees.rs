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
    /// Checks if two trees have the same leaf value sequence.
    ///
    /// # Intuition
    /// Collect leaves from each tree via DFS and compare the sequences.
    ///
    /// # Approach
    /// DFS both trees, collecting leaf values into vectors. Compare the
    /// two leaf vectors for equality.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n and m are tree sizes
    /// - Space: O(n + m) for leaf vectors and recursion stacks
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn collect_leaves(node: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
            let Some(n) = node else {
                return;
            };

            let borrowed = n.borrow();
            if borrowed.left.is_none() && borrowed.right.is_none() {
                leaves.push(borrowed.val);
            } else {
                collect_leaves(&borrowed.left, leaves);
                collect_leaves(&borrowed.right, leaves);
            }
        }

        // Estimate capacity based on typical binary tree leaf count
        let (mut leaves1, mut leaves2) = (Vec::with_capacity(32), Vec::with_capacity(32));
        collect_leaves(&root1, &mut leaves1);
        collect_leaves(&root2, &mut leaves2);
        leaves1 == leaves2
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
    fn test_leaf_similar_trees() {
        // Tree1: [3, 5, 1, 6, 2, 9, 8, null, null, 7, 4]
        // Tree2: [3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8]
        // Both have leaf sequence: [6, 7, 4, 9, 8]
        let root1 = build_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(9),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let root2 = build_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(7),
            Some(4),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(9),
            Some(8),
        ]);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_leaf_different_trees() {
        // Tree1: [1, 2, 3] - leaves: [2, 3]
        // Tree2: [1, 3, 2] - leaves: [3, 2]
        let root1 = build_tree(&[Some(1), Some(2), Some(3)]);
        let root2 = build_tree(&[Some(1), Some(3), Some(2)]);
        assert!(!Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_single_node_trees() {
        // Both trees have single node with same value
        let root1 = build_tree(&[Some(5)]);
        let root2 = build_tree(&[Some(5)]);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_different_structure_same_leaves() {
        // Tree1: [1, 2, null, 3] - leaves: [3]
        //       1
        //      /
        //     2
        //    /
        //   3
        // Tree2: [1, null, 2, null, 3] - leaves: [3]
        //       1
        //        \
        //         2
        //          \
        //           3
        let root1 = build_tree(&[Some(1), Some(2), None, Some(3)]);
        let root2 = build_tree(&[Some(1), None, Some(2), None, None, None, Some(3)]);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_empty_vs_nonempty() {
        // One empty tree, one non-empty
        let root1 = None;
        let root2 = build_tree(&[Some(1)]);
        assert!(!Solution::leaf_similar(root1, root2));
    }
}
