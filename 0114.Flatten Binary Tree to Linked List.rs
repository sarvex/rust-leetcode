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
    /// Flattens a binary tree to a linked list in-place using preorder collection.
    ///
    /// # Intuition
    /// Collect all nodes in preorder, then relink each node's right to the next
    /// node in the sequence while clearing left pointers.
    ///
    /// # Approach
    /// 1. Perform a preorder traversal collecting all node references.
    /// 2. Iterate through collected nodes, setting each node's right to the next
    ///    and left to None.
    ///
    /// # Complexity
    /// - Time: O(n) — two passes over all nodes
    /// - Space: O(n) — storage for node references
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        Self::preorder(&mut nodes, root);
        for i in 0..nodes.len() - 1 {
            let node = nodes[i].as_ref().unwrap();
            node.borrow_mut().left = None;
            node.borrow_mut().right = nodes[i + 1].clone();
        }
    }

    fn preorder(
        nodes: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root {
            nodes.push(root.clone());
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            Self::preorder(nodes, &left);
            Self::preorder(nodes, &right);
        }
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

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
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

    fn tree_to_list(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = root.clone();
        
        while let Some(node) = current {
            let borrowed = node.borrow();
            result.push(borrowed.val);
            assert!(borrowed.left.is_none(), "Left child should be None after flattening");
            current = borrowed.right.clone();
        }
        
        result
    }

    #[test]
    fn test_flatten_example1() {
        // Tree: [1,2,5,3,4,null,6]
        // After flattening: 1->2->3->4->5->6
        let mut tree = build_tree(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
        Solution::flatten(&mut tree);
        let result = tree_to_list(&tree);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_flatten_empty_tree() {
        let mut tree = None;
        Solution::flatten(&mut tree);
        assert_eq!(tree, None);
    }

    #[test]
    fn test_flatten_single_node() {
        let mut tree = build_tree(&[Some(1)]);
        Solution::flatten(&mut tree);
        let result = tree_to_list(&tree);
        assert_eq!(result, vec![1]);
    }
}
