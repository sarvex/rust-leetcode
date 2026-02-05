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
    /// BFS level-order traversal summing the deepest level.
    ///
    /// # Intuition
    /// By processing the tree level by level, the sum computed during the last
    /// level is exactly the sum of the deepest leaves.
    ///
    /// # Approach
    /// 1. Initialize a queue with the root
    /// 2. For each level, reset the sum and accumulate all node values
    /// 3. Enqueue children for the next level
    /// 4. After the loop, the sum holds the deepest level's total
    ///
    /// # Complexity
    /// - Time: O(n) — visiting every node once
    /// - Space: O(w) — where w is the maximum level width
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut sum = 0;

        while !queue.is_empty() {
            sum = 0;
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    sum += node.val;
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < vals.len() {
                if let Some(val) = vals[i] {
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
    fn test_regular_tree() {
        // Test tree: [1,2,3,4,5,null,6,7,null,null,null,null,8]
        //         1
        //       /   \
        //      2     3
        //     / \     \
        //    4   5     6
        //   /           \
        //  7             8
        // Deepest leaves: 7 + 8 = 15
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
            Some(7),
            None,
            None,
            None,
            None,
            Some(8),
        ]);
        assert_eq!(Solution::deepest_leaves_sum(root), 15);
    }

    #[test]
    fn test_complete_tree() {
        // Test tree: [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
        //         6
        //       /   \
        //      7     8
        //     / \   / \
        //    2   7 1   3
        //   /   / \     \
        //  9   1   4     5
        // Deepest leaves: 9 + 1 + 4 + 5 = 19
        let root = build_tree(vec![
            Some(6),
            Some(7),
            Some(8),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(9),
            None,
            Some(1),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        assert_eq!(Solution::deepest_leaves_sum(root), 19);
    }

    #[test]
    fn test_single_node() {
        // Test tree: [50]
        // Deepest leaves: 50
        let root = build_tree(vec![Some(50)]);
        assert_eq!(Solution::deepest_leaves_sum(root), 50);
    }

    #[test]
    fn test_two_levels() {
        // Test tree: [1,2,3]
        //     1
        //    / \
        //   2   3
        // Deepest leaves: 2 + 3 = 5
        let root = build_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::deepest_leaves_sum(root), 5);
    }

    #[test]
    fn test_left_skewed_tree() {
        // Test tree: [1,2,null,3,null,4,null,5]
        //     1
        //    /
        //   2
        //  /
        // 3
        // /
        //4
        // /
        //5
        // Deepest leaves: 5
        let root = build_tree(vec![
            Some(1),
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
        ]);
        assert_eq!(Solution::deepest_leaves_sum(root), 5);
    }

    #[test]
    fn test_negative_values() {
        // Test tree: [1,-2,3,4,-5,null,-6]
        //       1
        //      / \
        //    -2   3
        //    / \   \
        //   4  -5  -6
        // Deepest leaves: 4 + (-5) + (-6) = -7
        let root = build_tree(vec![
            Some(1),
            Some(-2),
            Some(3),
            Some(4),
            Some(-5),
            None,
            Some(-6),
        ]);
        assert_eq!(Solution::deepest_leaves_sum(root), -7);
    }
}
