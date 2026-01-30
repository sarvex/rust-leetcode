use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    /// Recursive partitioning with inorder index map for tree construction.
    ///
    /// # Intuition
    /// The first element of the preorder slice is always the root. Its
    /// position in the inorder array divides the sequence into left and
    /// right subtrees. A hash map provides O(1) lookup for this position.
    ///
    /// # Approach
    /// Build a value-to-index map for the inorder array. Recursively
    /// construct subtrees by slicing the preorder array: left subtree
    /// takes the next `k - j` elements, right subtree takes the rest.
    ///
    /// # Complexity
    /// - Time: O(n) - each node constructed once with O(1) lookups
    /// - Space: O(n) - hash map and recursion stack
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut index_map = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            index_map.insert(v, i);
        }

        fn dfs(
            preorder: &[i32],
            index_map: &HashMap<i32, usize>,
            pre_start: usize,
            in_start: usize,
            size: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if size == 0 {
                return None;
            }
            let root_val = preorder[pre_start];
            let in_pos = index_map[&root_val];
            let left_size = in_pos - in_start;

            let mut root = TreeNode::new(root_val);
            root.left = dfs(preorder, index_map, pre_start + 1, in_start, left_size);
            root.right = dfs(
                preorder,
                index_map,
                pre_start + 1 + left_size,
                in_pos + 1,
                size - 1 - left_size,
            );
            Some(Rc::new(RefCell::new(root)))
        }

        dfs(&preorder, &index_map, 0, 0, preorder.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            if let Some(node_opt) = queue.pop_front() {
                if let Some(node) = node_opt {
                    let borrowed = node.borrow();
                    result.push(Some(borrowed.val));
                    queue.push_back(borrowed.left.clone());
                    queue.push_back(borrowed.right.clone());
                } else {
                    result.push(None);
                }
            }
        }

        // Trim trailing None values
        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }

    #[test]
    fn test_example_tree() {
        // Preorder: [3,9,20,15,7]
        // Inorder: [9,3,15,20,7]
        // Expected tree: [3,9,20,null,null,15,7]
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let tree = Solution::build_tree(preorder, inorder);
        let result = tree_to_vec(tree);
        assert_eq!(
            result,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
        );
    }

    #[test]
    fn test_single_node() {
        let preorder = vec![1];
        let inorder = vec![1];
        let tree = Solution::build_tree(preorder, inorder);
        let result = tree_to_vec(tree);
        assert_eq!(result, vec![Some(1)]);
    }

    #[test]
    fn test_empty_tree() {
        let preorder = vec![];
        let inorder = vec![];
        let tree = Solution::build_tree(preorder, inorder);
        assert!(tree.is_none());
    }

    #[test]
    fn test_left_skewed() {
        // Preorder: [1,2,3]
        // Inorder: [3,2,1]
        // Tree:
        //       1
        //      /
        //     2
        //    /
        //   3
        let preorder = vec![1, 2, 3];
        let inorder = vec![3, 2, 1];
        let tree = Solution::build_tree(preorder, inorder);
        let result = tree_to_vec(tree);
        assert_eq!(result, vec![Some(1), Some(2), None, Some(3)]);
    }

    #[test]
    fn test_right_skewed() {
        // Preorder: [1,2,3]
        // Inorder: [1,2,3]
        // Tree:
        //   1
        //    \
        //     2
        //      \
        //       3
        let preorder = vec![1, 2, 3];
        let inorder = vec![1, 2, 3];
        let tree = Solution::build_tree(preorder, inorder);
        let result = tree_to_vec(tree);
        assert_eq!(result, vec![Some(1), None, Some(2), None, Some(3)]);
    }

    #[test]
    fn test_complete_tree() {
        // Preorder: [1,2,4,5,3,6,7]
        // Inorder: [4,2,5,1,6,3,7]
        // Tree:
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        let preorder = vec![1, 2, 4, 5, 3, 6, 7];
        let inorder = vec![4, 2, 5, 1, 6, 3, 7];
        let tree = Solution::build_tree(preorder, inorder);
        let result = tree_to_vec(tree);
        assert_eq!(
            result,
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7)
            ]
        );
    }
}
