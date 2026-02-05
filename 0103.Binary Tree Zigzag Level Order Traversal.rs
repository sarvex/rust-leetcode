// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode { val, left: None, right: None }
//   }
// }

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    /// BFS with alternating direction for zigzag level-order traversal.
    ///
    /// # Intuition
    /// Standard level-order BFS with a flag that toggles each level.
    /// On odd levels, reverse the collected values before appending.
    ///
    /// # Approach
    /// Process levels as in normal BFS. Toggle a `left_to_right` flag
    /// after each level. When the flag is false, reverse the level's
    /// values before adding to the result.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(n) — queue may hold an entire level
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut left_to_right = true;

        if let Some(root_node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root_node);

            while !queue.is_empty() {
                let level_size = queue.len();
                let mut level = Vec::with_capacity(level_size);

                for _ in 0..level_size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        level.push(node.val);
                        if let Some(ref left) = node.left {
                            queue.push_back(Rc::clone(left));
                        }
                        if let Some(ref right) = node.right {
                            queue.push_back(Rc::clone(right));
                        }
                    }
                }

                if !left_to_right {
                    level.reverse();
                }
                result.push(level);
                left_to_right = !left_to_right;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TreeNode definition for tests
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

    struct Solution;

    impl Solution {
        pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut left_to_right = true;

            if let Some(root_node) = root {
                let mut queue = VecDeque::new();
                queue.push_back(root_node);

                while !queue.is_empty() {
                    let level_size = queue.len();
                    let mut level = Vec::with_capacity(level_size);

                    for _ in 0..level_size {
                        if let Some(node) = queue.pop_front() {
                            let node = node.borrow();
                            level.push(node.val);
                            if let Some(ref left) = node.left {
                                queue.push_back(Rc::clone(left));
                            }
                            if let Some(ref right) = node.right {
                                queue.push_back(Rc::clone(right));
                            }
                        }
                    }

                    if !left_to_right {
                        level.reverse();
                    }
                    result.push(level);
                    left_to_right = !left_to_right;
                }
            }

            result
        }
    }

    // Helper function to create a tree from array representation
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
        let root = tree_from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![3], vec![20, 9], vec![15, 7]]);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        let root = tree_from_vec(vec![Some(1)]);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_empty_tree() {
        // Tree: []
        let root = tree_from_vec(vec![]);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_complete_tree() {
        // Tree: [1,2,3,4,5,6,7]
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        // Zigzag: [[1], [3,2], [4,5,6,7]]
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![1], vec![3, 2], vec![4, 5, 6, 7]]);
    }

    #[test]
    fn test_four_level_tree() {
        // Tree: [1,2,3,4,null,null,5,6,null,null,7]
        //         1
        //       /   \
        //      2     3
        //     /       \
        //    4         5
        //   /           \
        //  6             7
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));

        root.borrow_mut().left = Some(Rc::clone(&node2));
        root.borrow_mut().right = Some(Rc::clone(&node3));
        node2.borrow_mut().left = Some(Rc::clone(&node4));
        node3.borrow_mut().right = Some(Rc::clone(&node5));
        node4.borrow_mut().left = Some(node6);
        node5.borrow_mut().right = Some(node7);

        let result = Solution::zigzag_level_order(Some(root));
        // Level 0 (L->R): [1]
        // Level 1 (R->L): [3, 2]
        // Level 2 (L->R): [4, 5]
        // Level 3 (R->L): [7, 6]
        assert_eq!(result, vec![vec![1], vec![3, 2], vec![4, 5], vec![7, 6]]);
    }
}
