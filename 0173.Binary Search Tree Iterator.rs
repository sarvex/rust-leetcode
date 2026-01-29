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

/// BST iterator using flattened inorder traversal.
///
/// # Intuition
/// Pre-compute the inorder traversal and serve elements sequentially via an index.
///
/// # Approach
/// 1. Perform a full inorder traversal at construction, storing values in a vector.
/// 2. `next()` returns the current element and advances the index.
/// 3. `has_next()` checks if the index is within bounds.
///
/// # Complexity
/// - Time: O(n) construction, O(1) per `next`/`has_next`
/// - Space: O(n) for the values vector
struct BSTIterator {
    vals: Vec<i32>,
    index: usize,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut vals = Vec::new();
        Self::inorder(&root, &mut vals);
        Self { vals, index: 0 }
    }

    fn next(&mut self) -> i32 {
        let val = self.vals[self.index];
        self.index += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.index < self.vals.len()
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder(&node.left, result);
            result.push(node.val);
            Self::inorder(&node.right, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();
            
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_basic_bst_iteration() {
        // Tree: [7, 3, 15, null, null, 9, 20]
        //       7
        //      / \
        //     3  15
        //        / \
        //       9  20
        let tree = build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)]);
        let mut iterator = BSTIterator::new(tree);
        
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 3);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 7);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 9);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 15);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 20);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_single_node_tree() {
        let tree = build_tree(&[Some(42)]);
        let mut iterator = BSTIterator::new(tree);
        
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 42);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_empty_tree() {
        let iterator = BSTIterator::new(None);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_left_skewed_tree() {
        // Tree: [4, 3, null, 2, null, 1]
        //     4
        //    /
        //   3
        //  /
        // 2
        // /
        // 1
        let root = Rc::new(RefCell::new(TreeNode::new(4)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        
        root.borrow_mut().left = Some(Rc::clone(&node3));
        node3.borrow_mut().left = Some(Rc::clone(&node2));
        node2.borrow_mut().left = Some(node1);
        
        let mut iterator = BSTIterator::new(Some(root));
        
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.next(), 2);
        assert_eq!(iterator.next(), 3);
        assert_eq!(iterator.next(), 4);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_right_skewed_tree() {
        // Tree: [1, null, 2, null, 3, null, 4]
        // 1
        //  \
        //   2
        //    \
        //     3
        //      \
        //       4
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        
        root.borrow_mut().right = Some(Rc::clone(&node2));
        node2.borrow_mut().right = Some(Rc::clone(&node3));
        node3.borrow_mut().right = Some(node4);
        
        let mut iterator = BSTIterator::new(Some(root));
        
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.next(), 2);
        assert_eq!(iterator.next(), 3);
        assert_eq!(iterator.next(), 4);
        assert!(!iterator.has_next());
    }
}
