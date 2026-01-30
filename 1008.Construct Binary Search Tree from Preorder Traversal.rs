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
    /// Constructs a BST from preorder traversal using upper-bound constraint.
    ///
    /// # Intuition
    /// In preorder traversal, the first element is the root. For a BST, all elements
    /// smaller than root go left, larger go right. Using an upper bound constraint
    /// during construction allows O(n) time complexity.
    ///
    /// # Approach
    /// 1. Use a recursive approach with an index pointer and upper bound
    /// 2. The current element becomes a node if it's within the bound
    /// 3. Recursively build left subtree with current value as upper bound
    /// 4. Recursively build right subtree with inherited upper bound
    ///
    /// # Complexity
    /// - Time: O(n) - each element is processed exactly once
    /// - Space: O(h) - recursion stack depth equals tree height
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], idx: &mut usize, bound: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if *idx >= preorder.len() || preorder[*idx] > bound {
                return None;
            }

            let val = preorder[*idx];
            *idx += 1;

            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = build(preorder, idx, val);
            node.borrow_mut().right = build(preorder, idx, bound);

            Some(node)
        }

        build(&preorder, &mut 0, i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_to_preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                result.push(n.val);
                dfs(&n.left, result);
                dfs(&n.right, result);
            }
        }
        let mut result = Vec::with_capacity(16); // Typical tree size
        dfs(root, &mut result);
        result
    }

    fn validate_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let n = n.borrow();
                    if let Some(min_val) = min {
                        if n.val <= min_val {
                            return false;
                        }
                    }
                    if let Some(max_val) = max {
                        if n.val >= max_val {
                            return false;
                        }
                    }
                    dfs(&n.left, min, Some(n.val)) && dfs(&n.right, Some(n.val), max)
                }
            }
        }
        dfs(root, None, None)
    }

    #[test]
    fn test_basic_bst() {
        let preorder = vec![8, 5, 1, 7, 10, 12];
        let result = Solution::bst_from_preorder(preorder.clone());
        assert!(validate_bst(&result));
        assert_eq!(tree_to_preorder(&result), preorder);
    }

    #[test]
    fn test_single_node() {
        let preorder = vec![1];
        let result = Solution::bst_from_preorder(preorder.clone());
        assert!(validate_bst(&result));
        assert_eq!(tree_to_preorder(&result), preorder);
    }

    #[test]
    fn test_empty_tree() {
        let preorder = vec![];
        let result = Solution::bst_from_preorder(preorder);
        assert!(result.is_none());
    }

    #[test]
    fn test_left_skewed_tree() {
        let preorder = vec![5, 4, 3, 2, 1];
        let result = Solution::bst_from_preorder(preorder.clone());
        assert!(validate_bst(&result));
        assert_eq!(tree_to_preorder(&result), preorder);
    }

    #[test]
    fn test_right_skewed_tree() {
        let preorder = vec![1, 2, 3, 4, 5];
        let result = Solution::bst_from_preorder(preorder.clone());
        assert!(validate_bst(&result));
        assert_eq!(tree_to_preorder(&result), preorder);
    }

    #[test]
    fn test_balanced_bst() {
        let preorder = vec![4, 2, 1, 3, 6, 5, 7];
        let result = Solution::bst_from_preorder(preorder.clone());
        assert!(validate_bst(&result));
        assert_eq!(tree_to_preorder(&result), preorder);
    }
}
