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

pub struct Solution;

impl Solution {
    /// Checks whether root value equals sum of its two children.
    ///
    /// # Intuition
    /// A binary tree with exactly two children can be validated by comparing
    /// the root value against the sum of left and right child values.
    ///
    /// # Approach
    /// Borrow the root node and its two children, then compare values directly.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.as_ref().map_or(false, |node| {
            let node = node.borrow();
            let left_val = node.left.as_ref().map_or(0, |left| left.borrow().val);
            let right_val = node.right.as_ref().map_or(0, |right| right.borrow().val);
            node.val == left_val + right_val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(root_val: i32, left_val: i32, right_val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let left = Rc::new(RefCell::new(TreeNode::new(left_val)));
        let right = Rc::new(RefCell::new(TreeNode::new(right_val)));
        
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        
        Some(root)
    }

    #[test]
    fn test_example_1() {
        // Tree: [10,4,6]
        //       10
        //      /  \
        //     4    6
        // 10 = 4 + 6, so return true
        let root = create_tree(10, 4, 6);
        assert!(Solution::check_tree(root));
    }

    #[test]
    fn test_example_2() {
        // Tree: [5,3,1]
        //       5
        //      / \
        //     3   1
        // 5 != 3 + 1 (4), so return false
        let root = create_tree(5, 3, 1);
        assert!(!Solution::check_tree(root));
    }

    #[test]
    fn test_sum_equals_with_zeros() {
        // Tree: [0,0,0]
        //       0
        //      / \
        //     0   0
        // 0 = 0 + 0, so return true
        let root = create_tree(0, 0, 0);
        assert!(Solution::check_tree(root));
    }

    #[test]
    fn test_negative_values() {
        // Tree: [-1,-5,4]
        //       -1
        //      /  \
        //    -5    4
        // -1 = -5 + 4, so return true
        let root = create_tree(-1, -5, 4);
        assert!(Solution::check_tree(root));
    }

    #[test]
    fn test_large_values() {
        // Tree: [100,50,50]
        //       100
        //      /   \
        //     50   50
        // 100 = 50 + 50, so return true
        let root = create_tree(100, 50, 50);
        assert!(Solution::check_tree(root));
    }

    #[test]
    fn test_unequal_sum_negative() {
        // Tree: [10,-3,5]
        //       10
        //      /  \
        //    -3    5
        // 10 != -3 + 5 (2), so return false
        let root = create_tree(10, -3, 5);
        assert!(!Solution::check_tree(root));
    }

    #[test]
    fn test_one_child_zero() {
        // Tree: [7,7,0]
        //       7
        //      / \
        //     7   0
        // 7 = 7 + 0, so return true
        let root = create_tree(7, 7, 0);
        assert!(Solution::check_tree(root));
    }

    #[test]
    fn test_both_children_negative() {
        // Tree: [-10,-4,-6]
        //       -10
        //      /   \
        //    -4    -6
        // -10 = -4 + (-6), so return true
        let root = create_tree(-10, -4, -6);
        assert!(Solution::check_tree(root));
    }
}
