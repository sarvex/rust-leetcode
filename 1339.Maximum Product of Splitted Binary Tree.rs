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


impl Solution {
    /// Two-pass DFS to maximize the product of split subtree sums.
    ///
    /// # Intuition
    /// Removing any edge splits the tree into two parts with sums `s` and
    /// `total - s`. The product `s * (total - s)` is maximized when `s`
    /// is closest to `total / 2`. A first DFS computes the total, and a
    /// second DFS evaluates every possible split.
    ///
    /// # Approach
    /// 1. First DFS: compute total sum of all nodes
    /// 2. Second DFS: for each subtree sum `s`, update max product
    /// 3. Return `max_product % 10^9+7`
    ///
    /// # Complexity
    /// - Time: O(n) two tree traversals
    /// - Space: O(h) recursion stack depth
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn compute_total(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
            node.as_ref().map_or(0, |n| {
                let borrowed = n.borrow();
                i64::from(borrowed.val)
                    + compute_total(&borrowed.left)
                    + compute_total(&borrowed.right)
            })
        }

        fn find_max_product(
            node: &Option<Rc<RefCell<TreeNode>>>,
            total: i64,
            best: &mut i64,
        ) -> i64 {
            node.as_ref().map_or(0, |n| {
                let borrowed = n.borrow();
                let subtree_sum = i64::from(borrowed.val)
                    + find_max_product(&borrowed.left, total, best)
                    + find_max_product(&borrowed.right, total, best);
                *best = (*best).max(subtree_sum * (total - subtree_sum));
                subtree_sum
            })
        }

        let total = compute_total(&root);
        let mut best = 0_i64;
        find_max_product(&root, total, &mut best);
        (best % MOD) as i32
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

    #[test]
    fn test_example_1() {
        // Tree: [1,2,3,4,5,6]
        //       1
        //      / \
        //     2   3
        //    / \   \
        //   4   5   6
        // Total sum = 21
        // Best split: remove edge between 3 and 6 -> 15 * 6 = 90
        // Or remove edge between 1 and 3 -> 12 * 9 = 108
        // Or remove edge between 1 and 2 -> 10 * 11 = 110
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6)]);
        assert_eq!(Solution::max_product(root), 110);
    }

    #[test]
    fn test_example_2() {
        // Tree: [1,null,2,3,4,null,null,5,6]
        //       1
        //        \
        //         2
        //        / \
        //       3   4
        //          / \
        //         5   6
        // Total sum = 21
        // Best split: remove edge between 2 and 4 -> 6 * 15 = 90
        let root = build_tree(&[Some(1), None, Some(2), Some(3), Some(4), None, None, Some(5), Some(6)]);
        assert_eq!(Solution::max_product(root), 90);
    }

    #[test]
    fn test_balanced_tree() {
        // Tree: [2,1,3]
        //       2
        //      / \
        //     1   3
        // Total sum = 6
        // Split at left: 1 * 5 = 5
        // Split at right: 3 * 3 = 9
        let root = build_tree(&[Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::max_product(root), 9);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        // No edge to remove, product = 0
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::max_product(root), 0);
    }

    #[test]
    fn test_large_values() {
        // Tree with large values to test modulo operation
        // Tree: [50000,30000,40000]
        //       50000
        //      /     \
        //   30000   40000
        // Total = 120000
        // Split at left: 30000 * 90000 = 2700000000
        // Split at right: 40000 * 80000 = 3200000000
        let root = build_tree(&[Some(50000), Some(30000), Some(40000)]);
        // 3200000000 % 1000000007 = 199999986
        assert_eq!(Solution::max_product(root), 199999986);
    }
}
