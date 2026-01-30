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


impl Solution {
    /// Inverts a binary tree by recursively swapping left and right children.
    ///
    /// # Intuition
    /// Swap children at every node. Recursion handles the entire subtree.
    ///
    /// # Approach
    /// 1. If the node exists, take left and right children.
    /// 2. Recursively invert both.
    /// 3. Assign the inverted right to left and inverted left to right.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.left = Self::invert_tree(right);
            node.right = Self::invert_tree(left);
        }
        root
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

    fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(n) = node {
                let n = n.borrow();
                result.push(Some(n.val));
                queue.push_back(n.left.clone());
                queue.push_back(n.right.clone());
            } else {
                result.push(None);
            }
        }

        // Remove trailing None values
        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }

    #[test]
    fn test_invert_tree_balanced() {
        // Tree: [4,2,7,1,3,6,9]
        //        4
        //       / \
        //      2   7
        //     / \ / \
        //    1  3 6  9
        // Inverted: [4,7,2,9,6,3,1]
        let root = build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
        let inverted = Solution::invert_tree(root);
        let expected = build_tree(&[Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]);
        assert_eq!(tree_to_vec(&inverted), tree_to_vec(&expected));
    }

    #[test]
    fn test_invert_tree_simple() {
        // Tree: [2,1,3]
        //       2
        //      / \
        //     1   3
        // Inverted: [2,3,1]
        let root = build_tree(&[Some(2), Some(1), Some(3)]);
        let inverted = Solution::invert_tree(root);
        let expected = build_tree(&[Some(2), Some(3), Some(1)]);
        assert_eq!(tree_to_vec(&inverted), tree_to_vec(&expected));
    }

    #[test]
    fn test_invert_tree_empty() {
        // Empty tree
        let root = build_tree(&[]);
        let inverted = Solution::invert_tree(root);
        assert!(inverted.is_none());
    }

    #[test]
    fn test_invert_tree_single_node() {
        // Tree: [1]
        let root = build_tree(&[Some(1)]);
        let inverted = Solution::invert_tree(root);
        let expected = build_tree(&[Some(1)]);
        assert_eq!(tree_to_vec(&inverted), tree_to_vec(&expected));
    }

    #[test]
    fn test_invert_tree_unbalanced() {
        // Tree: [1,2,null,3]
        //       1
        //      /
        //     2
        //    /
        //   3
        // Inverted: [1,null,2,null,3]
        let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
        let inverted = Solution::invert_tree(root);
        let expected = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
        assert_eq!(tree_to_vec(&inverted), tree_to_vec(&expected));
    }
}
