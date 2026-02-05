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
    /// Sums all root-to-leaf numbers using DFS with accumulated value.
    ///
    /// # Intuition
    /// Each path from root to leaf forms a number by appending digits.
    /// Accumulate the number by multiplying by 10 at each level and adding
    /// the current node's value.
    ///
    /// # Approach
    /// 1. DFS with a running accumulated number.
    /// 2. At each node, compute `num * 10 + val`.
    /// 3. At leaf nodes, return the accumulated number.
    /// 4. For internal nodes, return the sum of left and right subtree results.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, accumulated: i32) -> i32 {
        node.as_ref().map_or(0, |n| {
            let n = n.borrow();
            let current = accumulated * 10 + n.val;
            match (&n.left, &n.right) {
                (None, None) => current,
                _ => Self::dfs(&n.left, current) + Self::dfs(&n.right, current),
            }
        })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(vals.len());
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
    fn test_sum_numbers_example1() {
        // Tree: [1,2,3]
        // Paths: 1->2 = 12, 1->3 = 13
        // Sum: 12 + 13 = 25
        let tree = build_tree(&[Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::sum_numbers(tree), 25);
    }

    #[test]
    fn test_sum_numbers_example2() {
        // Tree: [4,9,0,5,1]
        // Paths: 4->9->5 = 495, 4->9->1 = 491, 4->0 = 40
        // Sum: 495 + 491 + 40 = 1026
        let tree = build_tree(&[Some(4), Some(9), Some(0), Some(5), Some(1)]);
        assert_eq!(Solution::sum_numbers(tree), 1026);
    }

    #[test]
    fn test_sum_numbers_single_node() {
        // Tree: [5]
        // Path: 5
        // Sum: 5
        let tree = build_tree(&[Some(5)]);
        assert_eq!(Solution::sum_numbers(tree), 5);
    }
}
