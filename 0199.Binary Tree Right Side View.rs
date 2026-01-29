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
    /// Returns the right side view of a binary tree using BFS level-order traversal.
    ///
    /// # Intuition
    /// The rightmost node at each level is visible from the right side.
    /// BFS processes levels; the first node dequeued when processing right-to-left
    /// is the rightmost.
    ///
    /// # Approach
    /// 1. BFS with a queue, processing right children before left.
    /// 2. At each level, the first node in the queue is the rightmost.
    /// 3. Collect these values as the right side view.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(w) — where w is the maximum tree width
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let level_size = queue.len();
            result.push(queue[0].as_ref().unwrap().borrow().val);
            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let mut node = node.borrow_mut();
                    if node.right.is_some() {
                        queue.push_back(node.right.take());
                    }
                    if node.left.is_some() {
                        queue.push_back(node.left.take());
                    }
                }
            }
        }
        result
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
        // Test tree: [1,2,3,null,5,null,4]
        //     1
        //    / \
        //   2   3
        //    \   \
        //     5   4
        // Right side view: [1,3,4]
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    #[test]
    fn test_single_path_left() {
        // Test tree: [1,2,null,5]
        //     1
        //    /
        //   2
        //  /
        // 5
        // Right side view: [1,2,5]
        let root = build_tree(vec![Some(1), Some(2), None, Some(5)]);
        assert_eq!(Solution::right_side_view(root), vec![1, 2, 5]);
    }

    #[test]
    fn test_empty_tree() {
        // Empty tree
        let root = build_tree(vec![]);
        assert_eq!(Solution::right_side_view(root), vec![]);
    }

    #[test]
    fn test_single_node() {
        // Test tree: [1]
        // Right side view: [1]
        let root = build_tree(vec![Some(1)]);
        assert_eq!(Solution::right_side_view(root), vec![1]);
    }

    #[test]
    fn test_complete_tree() {
        // Test tree: [1,2,3,4,5,6,7]
        //       1
        //      / \
        //     2   3
        //    / \ / \
        //   4  5 6  7
        // Right side view: [1,3,7]
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 7]);
    }

    #[test]
    fn test_zigzag_tree() {
        // Test tree: [1,2,3,4,null,null,5,null,6]
        //       1
        //      / \
        //     2   3
        //    /     \
        //   4       5
        //    \
        //     6
        // Right side view: [1,3,5,6]
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
            None,
            Some(6),
        ]);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 5, 6]);
    }
}
