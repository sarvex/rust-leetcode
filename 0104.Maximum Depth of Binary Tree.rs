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
    /// Recursive DFS for maximum depth of a binary tree.
    ///
    /// # Intuition
    /// The depth of a tree is 1 plus the maximum depth of its subtrees.
    /// An empty tree has depth 0.
    ///
    /// # Approach
    /// Base case: return 0 for None. Otherwise return 1 plus the max of
    /// left and right subtree depths.
    ///
    /// # Complexity
    /// - Time: O(n) - each node visited once
    /// - Space: O(h) - recursion depth equals tree height
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    1 + dfs(&n.left).max(dfs(&n.right))
                }
            }
        }

        dfs(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn tree_from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();

            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_example_tree() {
        // Tree: [3,9,20,null,null,15,7]
        //       3
        //      / \
        //     9   20
        //        /  \
        //       15   7
        // Depth: 3
        let root = tree_from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn test_single_node() {
        let root = tree_from_vec(vec![Some(1)]);
        assert_eq!(Solution::max_depth(root), 1);
    }

    #[test]
    fn test_empty_tree() {
        let root = tree_from_vec(vec![]);
        assert_eq!(Solution::max_depth(root), 0);
    }

    #[test]
    fn test_two_node_tree() {
        // Tree: [1,2]
        //       1
        //      /
        //     2
        let root = tree_from_vec(vec![Some(1), Some(2)]);
        assert_eq!(Solution::max_depth(root), 2);
    }

    #[test]
    fn test_left_skewed_tree() {
        // Left-skewed tree with depth 4
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));

        root.borrow_mut().left = Some(Rc::clone(&node2));
        node2.borrow_mut().left = Some(Rc::clone(&node3));
        node3.borrow_mut().left = Some(node4);

        assert_eq!(Solution::max_depth(Some(root)), 4);
    }

    #[test]
    fn test_right_skewed_tree() {
        // Right-skewed tree with depth 4
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));

        root.borrow_mut().right = Some(Rc::clone(&node2));
        node2.borrow_mut().right = Some(Rc::clone(&node3));
        node3.borrow_mut().right = Some(node4);

        assert_eq!(Solution::max_depth(Some(root)), 4);
    }

    #[test]
    fn test_complete_tree() {
        // Tree: [1,2,3,4,5,6,7]
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        // Depth: 3
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }
}
