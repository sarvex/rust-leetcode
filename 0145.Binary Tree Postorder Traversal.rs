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
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Returns postorder traversal of a binary tree using recursive DFS.
    ///
    /// # Intuition
    /// Postorder visits left subtree, then right subtree, then root.
    ///
    /// # Approach
    /// Recursively collect node values in left-right-root order.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack where h is tree height
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::dfs(&root, &mut result);
        result
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::dfs(&node.left, result);
            Self::dfs(&node.right, result);
            result.push(node.val);
        }
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
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut result = Vec::new();
            Self::dfs(&root, &mut result);
            result
        }

        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                Self::dfs(&node.left, result);
                Self::dfs(&node.right, result);
                result.push(node.val);
            }
        }
    }

    // Helper function to create a tree from array representation
    fn tree_from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
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
        // Tree: [1,null,2,3]
        //       1
        //        \
        //         2
        //        /
        //       3
        // Postorder: left -> right -> root = [3, 2, 1]
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));

        root.borrow_mut().right = Some(Rc::clone(&node2));
        node2.borrow_mut().left = Some(node3);

        let result = Solution::postorder_traversal(Some(root));
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_empty_tree() {
        // Tree: []
        let root = tree_from_vec(vec![]);
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        let root = tree_from_vec(vec![Some(1)]);
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_complete_tree() {
        // Tree: [1,2,3,4,5,6,7]
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        // Postorder: 4 -> 5 -> 2 -> 6 -> 7 -> 3 -> 1
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![4, 5, 2, 6, 7, 3, 1]);
    }

    #[test]
    fn test_left_skewed_tree() {
        // Tree: left-skewed
        //       1
        //      /
        //     2
        //    /
        //   3
        //  /
        // 4
        // Postorder: 4 -> 3 -> 2 -> 1
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));

        root.borrow_mut().left = Some(Rc::clone(&node2));
        node2.borrow_mut().left = Some(Rc::clone(&node3));
        node3.borrow_mut().left = Some(node4);

        let result = Solution::postorder_traversal(Some(root));
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_right_skewed_tree() {
        // Tree: right-skewed
        //       1
        //        \
        //         2
        //          \
        //           3
        //            \
        //             4
        // Postorder: 4 -> 3 -> 2 -> 1
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));

        root.borrow_mut().right = Some(Rc::clone(&node2));
        node2.borrow_mut().right = Some(Rc::clone(&node3));
        node3.borrow_mut().right = Some(node4);

        let result = Solution::postorder_traversal(Some(root));
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_balanced_tree() {
        // Tree: [1,2,3,4,5,null,6]
        //         1
        //       /   \
        //      2     3
        //     / \     \
        //    4   5     6
        // Postorder: 4 -> 5 -> 2 -> 6 -> 3 -> 1
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
        ]);
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![4, 5, 2, 6, 3, 1]);
    }
}
