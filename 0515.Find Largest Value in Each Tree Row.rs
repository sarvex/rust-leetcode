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

pub struct Solution;

impl Solution {
    /// Finds the largest value in each tree row using BFS.
    ///
    /// # Intuition
    /// Use level-order traversal to process each level of the tree separately,
    /// keeping track of the maximum value found at each level.
    ///
    /// # Approach
    /// 1. Use BFS with level-size counting to process one level at a time
    /// 2. Track the maximum value within each level
    /// 3. Add each level's maximum to the result vector
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(w) where w is the maximum width of the tree
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let mut level_max = i32::MIN;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let inner = node.as_ref().unwrap().borrow();
                level_max = level_max.max(inner.val);
                if inner.left.is_some() {
                    queue.push_back(inner.left.clone());
                }
                if inner.right.is_some() {
                    queue.push_back(inner.right.clone());
                }
            }
            result.push(level_max);
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
        // Tree: [1, 3, 2, 5, 3, None, 9]
        //         1
        //       /   \
        //      3     2
        //     / \     \
        //    5   3     9
        // Level maximums: [1, 3, 9]
        let root = create_tree(vec![
            Some(1), 
            Some(3), Some(2), 
            Some(5), Some(3), None, Some(9)
        ]);
        assert_eq!(Solution::largest_values(root), vec![1, 3, 9]);
    }

    #[test]
    fn test_case_2() {
        // Tree: [1, 2, 3]
        //       1
        //      / \
        //     2   3
        // Level maximums: [1, 3]
        let root = create_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::largest_values(root), vec![1, 3]);
    }

    #[test]
    fn test_case_3() {
        // Single node tree
        let root = create_tree(vec![Some(42)]);
        assert_eq!(Solution::largest_values(root), vec![42]);
    }

    #[test]
    fn test_case_4() {
        // Empty tree
        let root = None;
        assert_eq!(Solution::largest_values(root), vec![]);
    }

    #[test]
    fn test_case_5() {
        // Tree with negative values
        //         -1
        //       /    \
        //     -5      -3
        //     / \    /  \
        //   -6  -2  -4  -10
        // Level maximums: [-1, -3, -2]
        let root = create_tree(vec![
            Some(-1), 
            Some(-5), Some(-3), 
            Some(-6), Some(-2), Some(-4), Some(-10)
        ]);
        assert_eq!(Solution::largest_values(root), vec![-1, -3, -2]);
    }

    #[test]
    fn test_case_6() {
        // Left-skewed tree
        //    10
        //   /
        //  20
        // /
        //30
        let root = Rc::new(RefCell::new(TreeNode::new(10)));
        let left1 = Rc::new(RefCell::new(TreeNode::new(20)));
        let left2 = Rc::new(RefCell::new(TreeNode::new(30)));
        left1.borrow_mut().left = Some(left2);
        root.borrow_mut().left = Some(left1);
        assert_eq!(Solution::largest_values(Some(root)), vec![10, 20, 30]);
    }
}
