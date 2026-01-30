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
    /// Checks if one tree is a subtree of another via recursive comparison.
    ///
    /// # Intuition
    /// A tree `sub_root` is a subtree of `root` if there exists a node in `root`
    /// where the entire subtree rooted at that node is identical to `sub_root`.
    ///
    /// # Approach
    /// 1. For each node in the main tree, check if it matches the sub-tree root
    /// 2. If a node matches, verify that all descendants match recursively
    /// 3. If no match at current node, recursively check left and right subtrees
    ///
    /// # Complexity
    /// - Time: O(m × n) worst case where m and n are the sizes of the trees
    /// - Space: O(h) where h is the height of the tree (recursion stack)
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.val == b.val && same(&a.left, &b.left) && same(&a.right, &b.right)
                }
                _ => false,
            }
        }

        match &root {
            None => false,
            Some(rc) => {
                let node = rc.borrow();
                same(&root, &sub_root)
                    || Self::is_subtree(node.left.clone(), sub_root.clone())
                    || Self::is_subtree(node.right.clone(), sub_root.clone())
            }
        }
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
        // root: [3, 4, 5, 1, 2]
        //         3
        //       /   \
        //      4     5
        //     / \
        //    1   2
        // subRoot: [4, 1, 2]
        //      4
        //     / \
        //    1   2
        // Result: true
        let root = create_tree(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = create_tree(vec![Some(4), Some(1), Some(2)]);
        assert!(Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_case_2() {
        // root: [3, 4, 5, 1, 2, None, None, None, None, 0]
        //         3
        //       /   \
        //      4     5
        //     / \
        //    1   2
        //       /
        //      0
        // subRoot: [4, 1, 2]
        //      4
        //     / \
        //    1   2
        // Result: false (root's subtree has extra node 0)
        let root = create_tree(vec![
            Some(3), 
            Some(4), Some(5), 
            Some(1), Some(2), None, None,
            None, None, Some(0)
        ]);
        let sub_root = create_tree(vec![Some(4), Some(1), Some(2)]);
        assert!(!Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_case_3() {
        // Both trees are identical single nodes
        let root = create_tree(vec![Some(1)]);
        let sub_root = create_tree(vec![Some(1)]);
        assert!(Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_case_4() {
        // Empty subtree (should return false)
        let root = create_tree(vec![Some(1), Some(2)]);
        let sub_root = None;
        assert!(!Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_case_5() {
        // Subtree is larger than main tree
        let root = create_tree(vec![Some(1)]);
        let sub_root = create_tree(vec![Some(1), Some(2)]);
        assert!(!Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_case_6() {
        // Values match but structure doesn't
        // root:     1
        //          /
        //         2
        // subRoot:  1
        //            \
        //             2
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        
        let sub_root = Rc::new(RefCell::new(TreeNode::new(1)));
        sub_root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        
        assert!(!Solution::is_subtree(Some(root), Some(sub_root)));
    }
}
