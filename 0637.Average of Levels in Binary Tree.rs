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
    /// Computes the average value of each tree level using BFS.
    ///
    /// # Intuition
    /// Level-order traversal sums each level's values and divides by the
    /// level size for the average.
    ///
    /// # Approach
    /// 1. BFS with level-size counting.
    /// 2. Sum values using i64 to avoid overflow.
    /// 3. Compute the average for each level.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(w) where w is the maximum level width
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut sum: i64 = 0;
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let inner = node.borrow();
                    sum += inner.val as i64;
                    if let Some(left) = inner.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = inner.right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            result.push(sum as f64 / level_size as f64);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

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

        Some(root)
    }

    #[test]
    fn test_example_1() {
        // Tree: [3,9,20,null,null,15,7]
        //       3
        //      / \
        //     9   20
        //        /  \
        //       15   7
        let root = create_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![3.0, 14.5, 11.0]);
    }

    #[test]
    fn test_example_2() {
        // Tree: [3,9,20,15,7]
        //       3
        //      / \
        //     9   20
        //    / \
        //   15  7
        let root = create_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7)]);
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![3.0, 14.5, 11.0]);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        let root = create_tree(&[Some(1)]);
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![1.0]);
    }

    #[test]
    fn test_large_values() {
        // Test with large values to verify i64 sum handling
        // Tree: [2147483647,2147483647,2147483647]
        let root = create_tree(&[Some(2147483647), Some(2147483647), Some(2147483647)]);
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![2147483647.0, 2147483647.0]);
    }

    #[test]
    fn test_negative_values() {
        // Tree: [-1,-2,-3,-4,-5]
        //        -1
        //       /  \
        //     -2    -3
        //     / \
        //   -4  -5
        let root = create_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5)]);
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![-1.0, -2.5, -4.5]);
    }
}
