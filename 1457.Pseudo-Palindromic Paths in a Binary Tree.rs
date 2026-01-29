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
    /// Bitmask DFS counting pseudo-palindromic root-to-leaf paths.
    ///
    /// # Intuition
    /// A path forms a pseudo-palindrome if at most one digit has odd frequency.
    /// XOR-ing a bitmask with `1 << val` toggles parity for each digit. At a
    /// leaf, the path is pseudo-palindromic if the mask has at most one bit set.
    ///
    /// # Approach
    /// 1. DFS with a bitmask tracking digit parity
    /// 2. At each node, toggle the bit for node value
    /// 3. At leaves, check `mask & (mask - 1) == 0` (power of two or zero)
    /// 4. Sum valid leaf counts
    ///
    /// # Complexity
    /// - Time: O(n) visiting every node once
    /// - Space: O(h) recursion depth
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mask: i32) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let mask = mask ^ (1 << n.val);
                    if n.left.is_none() && n.right.is_none() {
                        return if mask & (mask - 1) == 0 { 1 } else { 0 };
                    }
                    dfs(n.left.clone(), mask) + dfs(n.right.clone(), mask)
                }
                None => 0,
            }
        }

        dfs(root, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
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
    fn test_example_1() {
        // Tree: [2,3,1,3,1,null,1]
        //       2
        //      / \
        //     3   1
        //    / \   \
        //   3   1   1
        // Paths: [2,3,3], [2,3,1], [2,1,1]
        // Pseudo-palindromic: [2,3,3] (3,3,2), [2,1,1] (1,1,2)
        // Count: 2
        let root = build_tree(&[Some(2), Some(3), Some(1), Some(3), Some(1), None, Some(1)]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
    }

    #[test]
    fn test_example_2() {
        // Tree: [2,1,1,1,3,null,null,null,null,null,1]
        //       2
        //      / \
        //     1   1
        //    / \
        //   1   3
        //        \
        //         1
        // Paths: [2,1,1], [2,1,3,1]
        // Pseudo-palindromic: [2,1,1] (can be 1,2,1), [2,1,3,1] (can be 1,3,2,1)
        // Count: 1
        let root = build_tree(&[
            Some(2), Some(1), Some(1), Some(1), Some(3), None, None,
            None, None, None, Some(1)
        ]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }

    #[test]
    fn test_single_node() {
        // Tree: [9]
        // Single node is always pseudo-palindromic
        let root = build_tree(&[Some(9)]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }

    #[test]
    fn test_all_same_values() {
        // Tree: [1,1,1,1,1]
        //       1
        //      / \
        //     1   1
        //    / \
        //   1   1
        // All paths have only 1s, always pseudo-palindromic
        let root = build_tree(&[Some(1), Some(1), Some(1), Some(1), Some(1)]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
    }

    #[test]
    fn test_no_palindromic_paths() {
        // Tree: [1,2,3,4,5,6,7]
        //       1
        //      / \
        //     2   3
        //    / \ / \
        //   4  5 6  7
        // Paths: [1,2,4], [1,2,5], [1,3,6], [1,3,7]
        // None are pseudo-palindromic (all have 3 different values)
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 0);
    }
}
