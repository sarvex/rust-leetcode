use std::cell::RefCell;
use std::rc::Rc;

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
    /// Deletes a node with the given key from a BST.
    ///
    /// # Intuition
    /// Recursively locate the key, then handle three cases: leaf node, single
    /// child, or two children (replace with in-order successor).
    ///
    /// # Approach
    /// 1. Search left or right subtree based on key comparison.
    /// 2. When found with two children, find the leftmost node in the right
    ///    subtree (in-order successor), swap values, and recursively delete.
    /// 3. With zero or one child, return the existing child directly.
    ///
    /// # Complexity
    /// - Time: O(h) where h is tree height
    /// - Space: O(h) recursion depth
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn min_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let n = node.as_ref().unwrap().borrow();
            if n.left.is_none() {
                n.val
            } else {
                min_val(&n.left)
            }
        }

        match root {
            None => None,
            Some(rc) => {
                let val = rc.borrow().val;
                match val.cmp(&key) {
                    std::cmp::Ordering::Greater => {
                        let left = rc.borrow_mut().left.take();
                        rc.borrow_mut().left = Self::delete_node(left, key);
                        Some(rc)
                    }
                    std::cmp::Ordering::Less => {
                        let right = rc.borrow_mut().right.take();
                        rc.borrow_mut().right = Self::delete_node(right, key);
                        Some(rc)
                    }
                    std::cmp::Ordering::Equal => {
                        let left = rc.borrow_mut().left.take();
                        let right = rc.borrow_mut().right.take();
                        match (left.is_some(), right.is_some()) {
                            (false, false) => None,
                            (true, false) => left,
                            (false, true) => right,
                            (true, true) => {
                                let successor = min_val(&right);
                                rc.borrow_mut().val = successor;
                                rc.borrow_mut().left = left;
                                rc.borrow_mut().right = Self::delete_node(right, successor);
                                Some(rc)
                            }
                        }
                    }
                }
            }
        }
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

    fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while let Some(node_opt) = queue.pop_front() {
            match node_opt {
                Some(node) => {
                    let n = node.borrow();
                    result.push(Some(n.val));
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
                None => result.push(None),
            }
        }

        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }

    #[test]
    fn test_delete_leaf_node() {
        let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        let result = Solution::delete_node(tree, 7);
        let vals = tree_to_vec(result);
        assert!(!vals.contains(&Some(7)));
    }

    #[test]
    fn test_delete_node_with_one_child() {
        let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        let result = Solution::delete_node(tree, 6);
        let vals = tree_to_vec(result);
        assert!(!vals.contains(&Some(6)));
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        let result = Solution::delete_node(tree, 3);
        let vals = tree_to_vec(result);
        assert!(!vals.contains(&Some(3)));
    }

    #[test]
    fn test_delete_root() {
        let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        let result = Solution::delete_node(tree, 5);
        let vals = tree_to_vec(result);
        assert!(!vals.contains(&Some(5)));
    }

    #[test]
    fn test_key_not_found() {
        let tree = build_tree(&[Some(5), Some(3), Some(6)]);
        let result = Solution::delete_node(tree.clone(), 10);
        assert_eq!(tree_to_vec(result), vec![Some(5), Some(3), Some(6)]);
    }

    #[test]
    fn test_empty_tree() {
        let result = Solution::delete_node(None, 5);
        assert!(result.is_none());
    }
}
