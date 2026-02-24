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
    /// Finds all root-to-leaf paths that sum to a target using backtracking DFS.
    ///
    /// # Intuition
    /// Explore every root-to-leaf path, maintaining a running path and remaining sum.
    /// At each leaf, check if the remaining sum equals the node's value.
    ///
    /// # Approach
    /// 1. Use DFS with a mutable path vector for backtracking.
    /// 2. Subtract each node's value from the remaining target.
    /// 3. At leaf nodes, if remaining target is zero, record the current path.
    /// 4. Pop the last element after exploring both subtrees.
    ///
    /// # Complexity
    /// - Time: O(n^2) — visit every node, path cloning at leaves costs O(n)
    /// - Space: O(n) — recursion stack and path storage
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        Self::dfs(&root, target_sum, &mut path, &mut result);
        result
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        remaining: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            let remaining = remaining - node.val;
            path.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                if remaining == 0 {
                    result.push(path.clone());
                }
            } else {
                Self::dfs(&node.left, remaining, path, result);
                Self::dfs(&node.right, remaining, path, result);
            }
            path.pop();
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < vals.len() {
            if let Some(node) = queue.pop_front() {
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
        }

        Some(root)
    }

    #[test]
    fn test_path_sum_example1() {
        // Tree: [5,4,8,11,null,13,4,7,2,null,null,5,1]
        // Target: 22
        // Expected: [[5,4,11,2],[5,8,4,5]]
        let tree = build_tree(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ]);

        let mut result = Solution::path_sum(tree, 22);
        result.sort();
        let mut expected = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_path_sum_empty_tree() {
        let tree = None;
        let result = Solution::path_sum(tree, 0);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_path_sum_single_node_match() {
        let tree = build_tree(&[Some(1)]);
        let result = Solution::path_sum(tree, 1);
        assert_eq!(result, vec![vec![1]]);
    }
}
