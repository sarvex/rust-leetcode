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
    /// Flattens a binary tree to a linked list in-place using rightmost rewiring.
    ///
    /// # Intuition
    /// When a node has a left subtree, the rightmost node of that subtree is the
    /// predecessor in preorder. Move the current right subtree there, then promote
    /// the left subtree to the right.
    ///
    /// # Approach
    /// 1. Walk down the tree using right pointers.
    /// 2. If a node has a left subtree:
    ///    - Find the rightmost node of that left subtree.
    ///    - Attach the current right subtree to that rightmost node.
    ///    - Move the left subtree to the right and clear the left.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) extra
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut current = root.clone();
        while let Some(node) = current {
            let left_subtree = node.borrow_mut().left.take();
            if let Some(left) = left_subtree {
                let mut predecessor = left.clone();
                loop {
                    let next = predecessor.borrow().right.clone();
                    match next {
                        Some(next_node) => predecessor = next_node,
                        None => break,
                    }
                }

                let right_subtree = node.borrow_mut().right.take();
                predecessor.borrow_mut().right = right_subtree;
                node.borrow_mut().right = Some(left);
            }
            current = node.borrow().right.clone();
        }
    }
}

/// Binary tree node used by the LeetCode harness.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    /// Node value.
    pub val: i32,
    /// Left child pointer.
    pub left: Option<Rc<RefCell<TreeNode>>>,
    /// Right child pointer.
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Creates a new tree node with empty children.
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// LeetCode solution container.

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
            assert!(
                borrowed.left.is_none(),
                "Left child should be None after flattening"
            );
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
