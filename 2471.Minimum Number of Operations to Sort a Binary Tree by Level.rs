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
    /// Counts minimum swaps to sort each level of a binary tree.
    ///
    /// # Intuition
    /// Perform level-order traversal, then count the minimum number of swaps
    /// (selection sort swaps) needed to sort each level independently.
    ///
    /// # Approach
    /// 1. BFS to collect values level by level
    /// 2. For each level, count minimum swaps via selection sort
    /// 3. Sum swap counts across all levels
    ///
    /// # Complexity
    /// - Time: O(n * w) where w is the maximum level width (selection sort per level)
    /// - Space: O(n) for the queue and level arrays
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut ans = 0;

        while !queue.is_empty() {
            let n = queue.len();
            let mut row = Vec::with_capacity(n);

            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let mut node = node.as_ref().unwrap().borrow_mut();
                row.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left.take());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.take());
                }
            }

            for i in 0..n.saturating_sub(1) {
                let min_idx = (i..n).min_by_key(|&j| row[j]).unwrap_or(i);
                if i != min_idx {
                    row.swap(i, min_idx);
                    ans += 1;
                }
            }
        }

        ans
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

    #[test]
    fn example_one() {
        // [1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10]
        let root = node(
            1,
            node(4, leaf(7), leaf(6)),
            node(3, node(8, leaf(9), None), node(5, leaf(10), None)),
        );
        assert_eq!(Solution::minimum_operations(root), 3);
    }

    #[test]
    fn already_sorted() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::minimum_operations(root), 0);
    }

    #[test]
    fn single_node() {
        let root = leaf(1);
        assert_eq!(Solution::minimum_operations(root), 0);
    }

    #[test]
    fn one_swap_needed() {
        // Level 1: [3, 2] needs one swap
        let root = node(1, leaf(3), leaf(2));
        assert_eq!(Solution::minimum_operations(root), 1);
    }
}
