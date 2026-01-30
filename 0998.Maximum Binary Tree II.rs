use std::cell::RefCell;
use std::rc::Rc;

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


impl Solution {
    /// Inserts a value into a maximum binary tree as the last appended element.
    ///
    /// # Intuition
    /// The new value is appended to the original array. If it is larger than
    /// the root, it becomes the new root with the old tree as its left child.
    /// Otherwise, recurse into the right subtree.
    ///
    /// # Approach
    /// If the tree is empty or val > root.val, create a new node with the
    /// existing tree as the left child. Otherwise recursively insert into
    /// the right subtree.
    ///
    /// # Complexity
    /// - Time: O(n) worst case following right spine
    /// - Space: O(n) recursion stack
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) if node.borrow().val < val => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Some(node),
                right: None,
            }))),
            Some(node) => {
                let right = node.borrow_mut().right.take();
                node.borrow_mut().right = Self::insert_into_max_tree(right, val);
                Some(node)
            }
        }
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

    fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut result = Vec::with_capacity(values.len() * 2); // Account for potential nulls
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(n) = node {
                result.push(Some(n.borrow().val));
                queue.push_back(n.borrow().left.clone());
                queue.push_back(n.borrow().right.clone());
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
    fn test_insert_larger_than_root() {
        // Tree: [4,1,3,null,null,2]
        // Insert: 5
        // Expected: [5,4,null,1,3,null,null,2]
        let root = build_tree(vec![Some(4), Some(1), Some(3), None, None, Some(2)]);
        let result = Solution::insert_into_max_tree(root, 5);
        let expected = vec![
            Some(5),
            Some(4),
            None,
            Some(1),
            Some(3),
            None,
            None,
            Some(2),
        ];
        assert_eq!(tree_to_vec(result), expected);
    }

    #[test]
    fn test_insert_smaller_than_root() {
        // Tree: [5,2,4,null,1]
        // Insert: 3
        // Expected: [5,2,4,null,1,null,3]
        let root = build_tree(vec![Some(5), Some(2), Some(4), None, Some(1)]);
        let result = Solution::insert_into_max_tree(root, 3);
        let expected = vec![Some(5), Some(2), Some(4), None, Some(1), None, Some(3)];
        assert_eq!(tree_to_vec(result), expected);
    }

    #[test]
    fn test_insert_empty_tree() {
        // Tree: []
        // Insert: 1
        // Expected: [1]
        let root = None;
        let result = Solution::insert_into_max_tree(root, 1);
        assert_eq!(tree_to_vec(result), vec![Some(1)]);
    }

    #[test]
    fn test_insert_single_node() {
        // Tree: [3]
        // Insert: 5
        // Expected: [5,3]
        let root = build_tree(vec![Some(3)]);
        let result = Solution::insert_into_max_tree(root, 5);
        assert_eq!(tree_to_vec(result), vec![Some(5), Some(3)]);
    }
}
