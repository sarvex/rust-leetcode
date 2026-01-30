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
    /// Converts BST to Greater Sum Tree using reverse in-order traversal.
    ///
    /// # Intuition
    /// In a BST, all nodes greater than the current node are in its right subtree
    /// or are ancestors via right edges. A reverse in-order traversal (right → node
    /// → left) visits nodes in descending order, allowing us to accumulate a running
    /// sum of all greater values seen so far.
    ///
    /// # Approach
    /// 1. Perform reverse in-order DFS: process right subtree first
    /// 2. Maintain a running sum of all values processed so far
    /// 3. Add running sum to current node's value
    /// 4. Update running sum to include current node
    /// 5. Process left subtree with updated sum
    ///
    /// # Complexity
    /// - Time: O(n) - visits each node exactly once
    /// - Space: O(h) - recursion stack depth equals tree height
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                dfs(&n.right, sum);
                *sum += n.val;
                n.val = *sum;
                dfs(&n.left, sum);
            }
        }

        dfs(&root, &mut 0);
        root
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
    fn test_basic_bst() {
        // Input: [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
        // Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
        let root = build_tree(vec![
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8),
        ]);
        let result = Solution::bst_to_gst(root);
        let expected = vec![
            Some(30),
            Some(36),
            Some(21),
            Some(36),
            Some(35),
            Some(26),
            Some(15),
            None,
            None,
            None,
            Some(33),
            None,
            None,
            None,
            Some(8),
        ];
        assert_eq!(tree_to_vec(result), expected);
    }

    #[test]
    fn test_single_node() {
        let root = build_tree(vec![Some(5)]);
        let result = Solution::bst_to_gst(root);
        assert_eq!(tree_to_vec(result), vec![Some(5)]);
    }

    #[test]
    fn test_two_nodes() {
        // Input: [1,null,2]
        // Output: [3,null,2]
        let root = build_tree(vec![Some(1), None, Some(2)]);
        let result = Solution::bst_to_gst(root);
        assert_eq!(tree_to_vec(result), vec![Some(3), None, Some(2)]);
    }

    #[test]
    fn test_three_nodes() {
        // Input: [2,1,3]
        // Output: [5,6,3]
        let root = build_tree(vec![Some(2), Some(1), Some(3)]);
        let result = Solution::bst_to_gst(root);
        assert_eq!(tree_to_vec(result), vec![Some(5), Some(6), Some(3)]);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(Solution::bst_to_gst(None), None);
    }
}
