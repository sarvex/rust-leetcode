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
    /// Sums all left leaves in a binary tree via recursive DFS.
    ///
    /// # Intuition
    /// A left leaf is a node with no children that is the left child of its
    /// parent. Track the "is left child" flag through recursion.
    ///
    /// # Approach
    /// 1. DFS with an `is_left` flag.
    /// 2. If the node is a leaf and `is_left`, return its value.
    /// 3. Otherwise recurse on both children, passing the appropriate flag.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion depth where h is tree height
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            node.as_ref().map_or(0, |rc| {
                let inner = rc.borrow();
                match (&inner.left, &inner.right) {
                    (None, None) => {
                        if is_left {
                            inner.val
                        } else {
                            0
                        }
                    }
                    _ => dfs(&inner.left, true) + dfs(&inner.right, false),
                }
            })
        }
        dfs(&root, false)
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

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
    fn test_sum_of_left_leaves_example1() {
        // Tree: [3,9,20,null,null,15,7]
        // Left leaves: 9, 15
        // Sum: 9 + 15 = 24
        let tree = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::sum_of_left_leaves(tree), 24);
    }

    #[test]
    fn test_sum_of_left_leaves_single_node() {
        // Tree: [1]
        // No left leaves
        // Sum: 0
        let tree = build_tree(&[Some(1)]);
        assert_eq!(Solution::sum_of_left_leaves(tree), 0);
    }

    #[test]
    fn test_sum_of_left_leaves_only_left_child() {
        // Tree: [1,2]
        // Left leaf: 2
        // Sum: 2
        let tree = build_tree(&[Some(1), Some(2)]);
        assert_eq!(Solution::sum_of_left_leaves(tree), 2);
    }
}
