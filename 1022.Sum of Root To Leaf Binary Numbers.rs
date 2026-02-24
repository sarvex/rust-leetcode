use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    /// Sums all root-to-leaf binary numbers via DFS.
    ///
    /// # Intuition
    /// Each path from root to leaf represents a binary number built by
    /// shifting left and OR-ing the current node's bit.
    ///
    /// # Approach
    /// DFS passing the accumulated binary value. At each node, shift left
    /// and OR the value. At leaves, return the accumulated number.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, accumulated: i32) -> i32 {
            node.as_ref().map_or(0, |n| {
                let n = n.borrow();
                let current = (accumulated << 1) | n.val;
                match (&n.left, &n.right) {
                    (None, None) => current,
                    _ => dfs(&n.left, current) + dfs(&n.right, current),
                }
            })
        }
        dfs(&root, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::with_capacity(values.len());
        queue.push_back(root.clone());

        let mut i = 1;
        while i < values.len() && !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if i < values.len() && values[i].is_some() {
                let left = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            if i < values.len() && values[i].is_some() {
                let right = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_basic_tree() {
        // Tree: [1,0,1,0,1,0,1]
        // Paths: 100=4, 101=5, 110=6, 111=7
        // Sum: 4+5+6+7=22
        let root = build_tree(vec![
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(1),
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root), 22);
    }

    #[test]
    fn test_single_node() {
        // Tree: [0]
        // Path: 0
        // Sum: 0
        let root = build_tree(vec![Some(0)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 0);
    }

    #[test]
    fn test_single_node_one() {
        // Tree: [1]
        // Path: 1
        // Sum: 1
        let root = build_tree(vec![Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 1);
    }

    #[test]
    fn test_two_levels() {
        // Tree: [1,1,0]
        // Paths: 11=3, 10=2
        // Sum: 3+2=5
        let root = build_tree(vec![Some(1), Some(1), Some(0)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 5);
    }

    #[test]
    fn test_left_skewed() {
        // Tree: [1,0,null,1]
        // Path: 101=5
        // Sum: 5
        let root = build_tree(vec![Some(1), Some(0), None, Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 5);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(Solution::sum_root_to_leaf(None), 0);
    }
}
