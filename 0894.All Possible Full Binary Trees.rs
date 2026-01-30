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
    /// Generates all possible full binary trees with n nodes using memoized recursion.
    ///
    /// # Intuition
    /// A full binary tree has 0 or 2 children at every node. For n nodes,
    /// split n-1 nodes between left and right subtrees in all valid ways.
    ///
    /// # Approach
    /// Recursively generate all full binary trees. For each split `(i, n-1-i)`,
    /// combine all left and right subtree combinations. Memoize results.
    ///
    /// # Complexity
    /// - Time: O(2^(n/2)) Catalan number growth
    /// - Space: O(n * 2^(n/2)) for memoization
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut memo: Vec<Option<Vec<Option<Rc<RefCell<TreeNode>>>>>> =
            vec![None; (n + 1) as usize];
        Self::build(n, &mut memo)
    }

    fn build(
        n: i32,
        memo: &mut [Option<Vec<Option<Rc<RefCell<TreeNode>>>>>],
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(ref cached) = memo[n as usize] {
            return cached.clone();
        }

        // Estimate capacity based on Catalan number growth pattern
        let capacity = if n <= 7 { 5 } else { 20 };
        let mut result = Vec::with_capacity(capacity);
        if n == 1 {
            result.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        } else {
            for i in (1..n - 1).step_by(2) {
                let j = n - 1 - i;
                for left in Self::build(i, memo) {
                    for right in Self::build(j, memo) {
                        result.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }
        }

        memo[n as usize] = Some(result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                1 + count_nodes(&n.left) + count_nodes(&n.right)
            }
        }
    }

    fn is_full_binary_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let n = node.borrow();
                let has_left = n.left.is_some();
                let has_right = n.right.is_some();

                // Must have either 0 or 2 children
                if has_left != has_right {
                    return false;
                }

                is_full_binary_tree(&n.left) && is_full_binary_tree(&n.right)
            }
        }
    }

    #[test]
    fn test_n_equals_1() {
        // Only one possible tree: single node
        let trees = Solution::all_possible_fbt(1);
        assert_eq!(trees.len(), 1);
        assert_eq!(count_nodes(&trees[0]), 1);
    }

    #[test]
    fn test_n_equals_3() {
        // Only one possible tree: root with two children
        //     0
        //    / \
        //   0   0
        let trees = Solution::all_possible_fbt(3);
        assert_eq!(trees.len(), 1);
        assert_eq!(count_nodes(&trees[0]), 3);
        assert!(is_full_binary_tree(&trees[0]));
    }

    #[test]
    fn test_n_equals_5() {
        // Two possible trees
        let trees = Solution::all_possible_fbt(5);
        assert_eq!(trees.len(), 2);
        for tree in &trees {
            assert_eq!(count_nodes(tree), 5);
            assert!(is_full_binary_tree(tree));
        }
    }

    #[test]
    fn test_n_equals_7() {
        // Five possible trees (Catalan number C_3 = 5)
        let trees = Solution::all_possible_fbt(7);
        assert_eq!(trees.len(), 5);
        for tree in &trees {
            assert_eq!(count_nodes(tree), 7);
            assert!(is_full_binary_tree(tree));
        }
    }

    #[test]
    fn test_even_n() {
        // Even n cannot form a full binary tree
        let trees = Solution::all_possible_fbt(2);
        assert_eq!(trees.len(), 0);

        let trees = Solution::all_possible_fbt(4);
        assert_eq!(trees.len(), 0);
    }
}
