use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

impl Solution {
    /// Recursive partitioning with inorder index map for postorder tree construction.
    ///
    /// # Intuition
    /// The last element of the postorder slice is the root. Its position
    /// in the inorder array divides into left and right subtrees. A hash
    /// map provides O(1) inorder position lookup.
    ///
    /// # Approach
    /// Build a value-to-index map for inorder. The root is at `postorder[j + n - 1]`.
    /// Compute the left subtree size from the inorder split, then recursively
    /// build left and right subtrees from their respective postorder slices.
    ///
    /// # Complexity
    /// - Time: O(n) - each node constructed once
    /// - Space: O(n) - hash map and recursion stack
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut index_map = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            index_map.insert(v, i);
        }

        fn dfs(
            postorder: &[i32],
            index_map: &HashMap<i32, usize>,
            in_start: usize,
            post_start: usize,
            size: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if size == 0 {
                return None;
            }
            let root_val = postorder[post_start + size - 1];
            let in_pos = index_map[&root_val];
            let left_size = in_pos - in_start;

            let left = dfs(postorder, index_map, in_start, post_start, left_size);
            let right = dfs(
                postorder,
                index_map,
                in_pos + 1,
                post_start + left_size,
                size - 1 - left_size,
            );
            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left,
                right,
            })))
        }

        let n = inorder.len();
        dfs(&postorder, &index_map, 0, 0, n)
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
        // Inorder: [9,3,15,20,7]
        // Postorder: [9,15,7,20,3]
        // Expected tree: [3,9,20,null,null,15,7]
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let tree = Solution::build_tree(inorder, postorder);
        let result = tree_to_vec(tree);
        assert_eq!(
            result,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
        );
    }

    #[test]
    fn test_single_node() {
        let inorder = vec![1];
        let postorder = vec![1];
        let tree = Solution::build_tree(inorder, postorder);
        let result = tree_to_vec(tree);
        assert_eq!(result, vec![Some(1)]);
    }

    #[test]
    fn test_empty_tree() {
        let inorder = vec![];
        let postorder = vec![];
        let tree = Solution::build_tree(inorder, postorder);
        assert!(tree.is_none());
    }

    #[test]
    fn test_left_skewed() {
        // Inorder: [3,2,1]
        // Postorder: [3,2,1]
        // Tree:
        //       1
        //      /
        //     2
        //    /
        //   3
        let inorder = vec![3, 2, 1];
        let postorder = vec![3, 2, 1];
        let tree = Solution::build_tree(inorder, postorder);
        let result = tree_to_vec(tree);
        assert_eq!(result, vec![Some(1), Some(2), None, Some(3)]);
    }

    #[test]
    fn test_right_skewed() {
        // Inorder: [1,2,3]
        // Postorder: [3,2,1]
        // Tree:
        //   1
        //    \
        //     2
        //      \
        //       3
        let inorder = vec![1, 2, 3];
        let postorder = vec![3, 2, 1];
        let tree = Solution::build_tree(inorder, postorder);
        let result = tree_to_vec(tree);
        assert_eq!(result, vec![Some(1), None, Some(2), None, Some(3)]);
    }

    #[test]
    fn test_complete_tree() {
        // Inorder: [4,2,5,1,6,3,7]
        // Postorder: [4,5,2,6,7,3,1]
        // Tree:
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        let inorder = vec![4, 2, 5, 1, 6, 3, 7];
        let postorder = vec![4, 5, 2, 6, 7, 3, 1];
        let tree = Solution::build_tree(inorder, postorder);
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
