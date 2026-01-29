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
    /// Mirror-comparison DFS for symmetric tree validation.
    ///
    /// # Intuition
    /// A tree is symmetric if its left subtree is a mirror of its right
    /// subtree. Two subtrees are mirrors when their roots match and each
    /// node's left child mirrors the other's right child.
    ///
    /// # Approach
    /// Compare the root's left and right children. Recursively verify
    /// that `left.left` mirrors `right.right` and `left.right` mirrors
    /// `right.left`.
    ///
    /// # Complexity
    /// - Time: O(n) - each node visited once
    /// - Space: O(h) - recursion depth equals tree height
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let (a, b) = (a.borrow(), b.borrow());
                    a.val == b.val
                        && is_mirror(&a.left, &b.right)
                        && is_mirror(&a.right, &b.left)
                }
                _ => false,
            }
        }

        match root {
            Some(r) => {
                let r = r.borrow();
                is_mirror(&r.left, &r.right)
            }
            None => true,
        }
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
    fn test_symmetric_tree() {
        // Tree: [1,2,2,3,4,4,3]
        //         1
        //       /   \
        //      2     2
        //     / \   / \
        //    3   4 4   3
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        assert!(Solution::is_symmetric(root));
    }

    #[test]
    fn test_asymmetric_tree() {
        // Tree: [1,2,2,null,3,null,3]
        //         1
        //       /   \
        //      2     2
        //       \     \
        //        3     3
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);
        assert!(!Solution::is_symmetric(root));
    }

    #[test]
    fn test_empty_tree() {
        let root = tree_from_vec(vec![]);
        assert!(Solution::is_symmetric(root));
    }

    #[test]
    fn test_single_node() {
        let root = tree_from_vec(vec![Some(1)]);
        assert!(Solution::is_symmetric(root));
    }

    #[test]
    fn test_two_level_symmetric() {
        // Tree: [1,2,2]
        //      1
        //     / \
        //    2   2
        let root = tree_from_vec(vec![Some(1), Some(2), Some(2)]);
        assert!(Solution::is_symmetric(root));
    }

    #[test]
    fn test_two_level_asymmetric_values() {
        // Tree: [1,2,3]
        //      1
        //     / \
        //    2   3
        let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
        assert!(!Solution::is_symmetric(root));
    }
}
