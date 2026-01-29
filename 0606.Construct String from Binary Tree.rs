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

pub struct Solution;

impl Solution {
    /// Converts a binary tree to a string with parentheses via preorder DFS.
    ///
    /// # Intuition
    /// Use preorder traversal to build a string representation where each subtree
    /// is wrapped in parentheses. Empty nodes are omitted unless needed to distinguish
    /// tree structure (empty left child with non-empty right child).
    ///
    /// # Approach
    /// 1. Append the current node's value
    /// 2. If both children are absent, return
    /// 3. Always wrap left subtree in parentheses (even if empty when right exists)
    /// 4. Wrap right subtree in parentheses only if it exists
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(n) for the output string and O(h) for recursion stack
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
            if let Some(rc) = node {
                let inner = rc.borrow();
                result.push_str(&inner.val.to_string());
                if inner.left.is_none() && inner.right.is_none() {
                    return;
                }
                result.push('(');
                dfs(&inner.left, result);
                result.push(')');
                if inner.right.is_some() {
                    result.push('(');
                    dfs(&inner.right, result);
                    result.push(')');
                }
            }
        }

        let mut result = String::new();
        dfs(&root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            
            if i < vals.len() && vals[i].is_some() {
                let left = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            if i < vals.len() && vals[i].is_some() {
                let right = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_case_1() {
        // Tree: [1, 2, 3, 4]
        //       1
        //      / \
        //     2   3
        //    /
        //   4
        // Expected: "1(2(4))(3)"
        let root = create_tree(vec![Some(1), Some(2), Some(3), Some(4)]);
        assert_eq!(Solution::tree2str(root), "1(2(4))(3)");
    }

    #[test]
    fn test_case_2() {
        // Tree: [1, 2, 3, None, 4]
        //       1
        //      / \
        //     2   3
        //      \
        //       4
        // Expected: "1(2()(4))(3)"
        let root = create_tree(vec![Some(1), Some(2), Some(3), None, Some(4)]);
        assert_eq!(Solution::tree2str(root), "1(2()(4))(3)");
    }

    #[test]
    fn test_case_3() {
        // Single node tree
        let root = create_tree(vec![Some(5)]);
        assert_eq!(Solution::tree2str(root), "5");
    }

    #[test]
    fn test_case_4() {
        // Empty tree
        let root = None;
        assert_eq!(Solution::tree2str(root), "");
    }

    #[test]
    fn test_case_5() {
        // Tree with only left child
        //    1
        //   /
        //  2
        let root = create_tree(vec![Some(1), Some(2), None]);
        assert_eq!(Solution::tree2str(root), "1(2)");
    }

    #[test]
    fn test_case_6() {
        // Tree with only right child
        //    1
        //     \
        //      3
        let root = create_tree(vec![Some(1), None, Some(3)]);
        assert_eq!(Solution::tree2str(root), "1()(3)");
    }

    #[test]
    fn test_case_7() {
        // Tree with negative values
        //      -1
        //     /  \
        //   -2    -3
        let root = create_tree(vec![Some(-1), Some(-2), Some(-3)]);
        assert_eq!(Solution::tree2str(root), "-1(-2)(-3)");
    }
}
