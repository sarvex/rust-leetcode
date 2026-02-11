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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    /// Reverses node values at every odd level of a perfect binary tree.
    ///
    /// # Intuition
    /// Perform a level-order traversal collecting values per level, reverse
    /// odd-level values, then reconstruct the tree from the collected levels.
    ///
    /// # Approach
    /// 1. BFS to collect values level by level, reversing odd levels
    /// 2. Reconstruct the tree recursively using the collected level values
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once during BFS and once during reconstruction
    /// - Space: O(n) — storage for all node values
    fn create_tree(vals: &[Vec<i32>], i: usize, j: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if i >= vals.len() {
            return None;
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[i][j],
            left: Self::create_tree(vals, i + 1, j * 2),
            right: Self::create_tree(vals, i + 1, j * 2 + 1),
        })))
    }

    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut depth = 0;
        let mut vals = Vec::new();

        while !queue.is_empty() {
            let mut level_vals = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let mut node = node.as_ref().unwrap().borrow_mut();
                level_vals.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left.take());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.take());
                }
            }
            if depth % 2 == 1 {
                level_vals.reverse();
            }
            vals.push(level_vals);
            depth += 1;
        }

        Self::create_tree(&vals, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        node(val, None, None)
    }

    fn to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r.clone());
        }
        while let Some(n) = queue.pop_front() {
            let n = n.borrow();
            result.push(n.val);
            if let Some(l) = &n.left {
                queue.push_back(l.clone());
            }
            if let Some(r) = &n.right {
                queue.push_back(r.clone());
            }
        }
        result
    }

    #[test]
    fn perfect_tree_depth_2() {
        // [2, 3, 5] -> reverse level 1 -> [2, 5, 3]
        let root = node(2, leaf(3), leaf(5));
        let result = Solution::reverse_odd_levels(root);
        assert_eq!(to_vec(&result), vec![2, 5, 3]);
    }

    #[test]
    fn perfect_tree_depth_3() {
        // [7, 13, 11, 1, 2, 3, 4]
        // Level 1 reversed: 13,11 -> 11,13
        // Level 3 stays: 1,2,3,4 (even level 2 not reversed, level 3 odd but leaf values)
        let root = node(7, node(13, leaf(1), leaf(2)), node(11, leaf(3), leaf(4)));
        let result = Solution::reverse_odd_levels(root);
        let vals = to_vec(&result);
        assert_eq!(vals[0], 7);
        assert_eq!(vals[1], 11);
        assert_eq!(vals[2], 13);
    }

    #[test]
    fn single_node() {
        let root = leaf(1);
        let result = Solution::reverse_odd_levels(root);
        assert_eq!(to_vec(&result), vec![1]);
    }
}
