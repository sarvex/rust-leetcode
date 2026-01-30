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
use std::rc::Rc;


impl Solution {
    /// Finds the longest path where all nodes have the same value.
    ///
    /// # Intuition
    /// DFS returns the longest single-direction path of the target value.
    /// At each node, the path through it equals left + right arms. Track
    /// the global maximum.
    ///
    /// # Approach
    /// 1. DFS with a target value parameter.
    /// 2. If the node matches, extend the path from matching children.
    /// 3. Update the global max with left + right.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, max_path: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    let left = dfs(&inner.left, inner.val, max_path);
                    let right = dfs(&inner.right, inner.val, max_path);
                    *max_path = (*max_path).max(left + right);
                    if inner.val == target {
                        left.max(right) + 1
                    } else {
                        0
                    }
                }
            }
        }

        let mut result = 0;
        if let Some(ref rc) = root {
            dfs(&root, rc.borrow().val, &mut result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

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
        // Tree: [5,4,5,1,1,null,5]
        //       5
        //      / \
        //     4   5
        //    / \   \
        //   1   1   5
        // Path: 5 -> 5 -> 5 (length = 2)
        let root = create_tree(&[Some(5), Some(4), Some(5), Some(1), Some(1), None, Some(5)]);
        assert_eq!(Solution::longest_univalue_path(root), 2);
    }

    #[test]
    fn test_example_2() {
        // Tree: [1,4,5,4,4,null,5]
        //       1
        //      / \
        //     4   5
        //    / \   \
        //   4   4   5
        // Path: 4 -> 4 -> 4 (length = 2)
        let root = create_tree(&[Some(1), Some(4), Some(5), Some(4), Some(4), None, Some(5)]);
        assert_eq!(Solution::longest_univalue_path(root), 2);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        // No path with more than one node
        let root = create_tree(&[Some(1)]);
        assert_eq!(Solution::longest_univalue_path(root), 0);
    }

    #[test]
    fn test_all_same_value() {
        // Tree: [1,1,1,1,1,1,1]
        //       1
        //      / \
        //     1   1
        //    / \ / \
        //   1  1 1  1
        // Path goes through root: 1 -> 1 -> 1 -> 1 -> 1 (length = 4)
        let root = create_tree(&[Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1)]);
        assert_eq!(Solution::longest_univalue_path(root), 4);
    }

    #[test]
    fn test_no_univalue_path() {
        // Tree: [1,2,3,4,5,6,7]
        //       1
        //      / \
        //     2   3
        //    / \ / \
        //   4  5 6  7
        // All values different, no path
        let root = create_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
        assert_eq!(Solution::longest_univalue_path(root), 0);
    }

    #[test]
    fn test_path_not_through_root() {
        // Tree: [1,2,2,2,2,2]
        //       1
        //      / \
        //     2   2
        //    / \ /
        //   2  2 2
        // Best path is in left subtree: 2 -> 2 -> 2 (length = 2)
        let root = create_tree(&[Some(1), Some(2), Some(2), Some(2), Some(2), Some(2)]);
        assert_eq!(Solution::longest_univalue_path(root), 2);
    }
}
