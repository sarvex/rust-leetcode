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
use std::rc::Rc;

impl Solution {
    /// Recursive enumeration of all structurally unique BSTs for values [1..n].
    ///
    /// # Intuition
    /// Each value in the range can serve as the root. Left and right subtrees
    /// are recursively built from the values below and above the root. The
    /// cartesian product of left and right subtrees gives all unique trees.
    ///
    /// # Approach
    /// For range `[i, j]`, iterate each value `v` as root. Recursively
    /// generate all left subtrees from `[i, v-1]` and right subtrees from
    /// `[v+1, j]`. Combine every left-right pair with root `v`.
    ///
    /// # Complexity
    /// - Time: O(n × C(n)) — Catalan number of trees, each with n nodes
    /// - Space: O(n × C(n)) — storing all trees
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(lo: i32, hi: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if lo > hi {
                return vec![None];
            }
            let mut trees = Vec::new();
            for root_val in lo..=hi {
                let left_trees = dfs(lo, root_val - 1);
                let right_trees = dfs(root_val + 1, hi);
                for left in &left_trees {
                    for right in &right_trees {
                        trees.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: root_val,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }
            trees
        }

        dfs(1, n)
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

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        inorder(root, &mut result);
        result
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            inorder(&n.left, result);
            result.push(n.val);
            inorder(&n.right, result);
        }
    }

    #[test]
    fn test_generate_trees_n3() {
        let trees = Solution::generate_trees(3);
        // For n=3, there should be 5 unique BSTs (Catalan number C(3) = 5)
        assert_eq!(trees.len(), 5);
        
        // Verify all trees are valid BSTs with values 1, 2, 3
        for tree in &trees {
            let vals = tree_to_vec(tree);
            assert_eq!(vals, vec![1, 2, 3]); // In-order should always be [1, 2, 3]
        }
    }

    #[test]
    fn test_generate_trees_n1() {
        let trees = Solution::generate_trees(1);
        // For n=1, there should be exactly 1 tree
        assert_eq!(trees.len(), 1);
        
        // The single tree should have just node with value 1
        let vals = tree_to_vec(&trees[0]);
        assert_eq!(vals, vec![1]);
    }

    #[test]
    fn test_generate_trees_n2() {
        let trees = Solution::generate_trees(2);
        // For n=2, there should be 2 unique BSTs (Catalan number C(2) = 2)
        assert_eq!(trees.len(), 2);
        
        // Both trees should have values 1, 2 in order
        for tree in &trees {
            let vals = tree_to_vec(tree);
            assert_eq!(vals, vec![1, 2]);
        }
    }
}
