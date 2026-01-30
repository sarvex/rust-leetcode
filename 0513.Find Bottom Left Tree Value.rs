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
use std::collections::VecDeque;
use std::rc::Rc;


impl Solution {
    /// Finds the leftmost value in the bottom row using BFS.
    ///
    /// # Intuition
    /// The bottom-left value is the first node we encounter in the last level of the tree.
    /// Using level-order traversal (BFS), we can track the first value of each level.
    ///
    /// # Approach
    /// 1. Perform BFS level by level
    /// 2. Record the first value of each level
    /// 3. The last recorded value is the bottom-left value
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(w) where w is the maximum width of the tree
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = 0;
        while !queue.is_empty() {
            result = queue.front().unwrap().as_ref().unwrap().borrow().val;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let inner = node.as_ref().unwrap().borrow();
                if inner.left.is_some() {
                    queue.push_back(inner.left.clone());
                }
                if inner.right.is_some() {
                    queue.push_back(inner.right.clone());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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
    fn test_case_1() {
        // Tree: [2, 1, 3]
        //       2
        //      / \
        //     1   3
        // Bottom left value is 1
        let root = create_tree(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::find_bottom_left_value(root), 1);
    }

    #[test]
    fn test_case_2() {
        // Tree: [1, 2, 3, 4, None, 5, 6, None, None, 7]
        //         1
        //       /   \
        //      2     3
        //     /     / \
        //    4     5   6
        //         /
        //        7
        // Bottom left value is 7
        let root = create_tree(vec![
            Some(1), 
            Some(2), Some(3), 
            Some(4), None, Some(5), Some(6), 
            None, None, None, None, Some(7)
        ]);
        assert_eq!(Solution::find_bottom_left_value(root), 7);
    }

    #[test]
    fn test_case_3() {
        // Single node tree
        let root = create_tree(vec![Some(5)]);
        assert_eq!(Solution::find_bottom_left_value(root), 5);
    }

    #[test]
    fn test_case_4() {
        // Left-skewed tree
        //    1
        //   /
        //  2
        // /
        //3
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let left2 = Rc::new(RefCell::new(TreeNode::new(3)));
        left1.borrow_mut().left = Some(left2);
        root.borrow_mut().left = Some(left1);
        assert_eq!(Solution::find_bottom_left_value(Some(root)), 3);
    }

    #[test]
    fn test_case_5() {
        // Right-skewed tree (bottom left is rightmost)
        //  1
        //   \
        //    2
        //     \
        //      3
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let right1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let right2 = Rc::new(RefCell::new(TreeNode::new(3)));
        right1.borrow_mut().right = Some(right2);
        root.borrow_mut().right = Some(right1);
        assert_eq!(Solution::find_bottom_left_value(Some(root)), 3);
    }
}
