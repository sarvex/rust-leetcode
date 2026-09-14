// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Counts nodes whose value equals the floored average of their subtree.
    ///
    /// # Intuition
    /// The average of a subtree needs only two aggregates: the sum of its
    /// values and the number of nodes. Both compose bottom-up, so a single
    /// post-order traversal can evaluate every node's condition while it is
    /// being unwound. Carrying the running match count in the same return
    /// value avoids any shared mutable state.
    ///
    /// # Approach
    /// 1. Post-order DFS returning the triple `(sum, count, matches)` for the
    ///    subtree rooted at the visited node.
    /// 2. Combine the children's aggregates with the current node:
    ///    `sum = left_sum + right_sum + val`, `count = left_count + right_count + 1`.
    /// 3. Integer division `sum / count` is already the floor because all
    ///    values are non-negative, so add one match when it equals `val`.
    /// 4. The root call's third component is the answer.
    ///
    /// With `n <= 1000` nodes and `val <= 1000`, the maximum subtree sum is
    /// `10^6`, so `i32` arithmetic never overflows.
    ///
    /// # Complexity
    /// - Time: O(n) — each node is visited exactly once
    /// - Space: O(h) — recursion depth, where `h` is the tree height
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Returns (subtree_sum, subtree_node_count, matching_node_count).
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            node.as_ref().map_or((0, 0, 0), |rc| {
                let inner = rc.borrow();
                let (left_sum, left_count, left_matches) = dfs(&inner.left);
                let (right_sum, right_count, right_matches) = dfs(&inner.right);

                let sum = left_sum + right_sum + inner.val;
                let count = left_count + right_count + 1;
                let matches = left_matches + right_matches + i32::from(sum / count == inner.val);

                (sum, count, matches)
            })
        }

        dfs(&root).2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // Builds a tree from a LeetCode level-order listing where `None` marks an
    // absent child and the children of absent nodes are omitted.
    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let root_val = vals.first().copied().flatten()?;
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue = VecDeque::with_capacity(vals.len());
        queue.push_back(Rc::clone(&root));
        let mut i = 1;

        while let Some(node) = queue.pop_front() {
            for slot in 0..2 {
                if i >= vals.len() {
                    return Some(root);
                }
                if let Some(val) = vals[i] {
                    let child = Rc::new(RefCell::new(TreeNode::new(val)));
                    if slot == 0 {
                        node.borrow_mut().left = Some(Rc::clone(&child));
                    } else {
                        node.borrow_mut().right = Some(Rc::clone(&child));
                    }
                    queue.push_back(child);
                }
                i += 1;
            }
        }

        Some(root)
    }

    // Builds a left-leaning chain of `n` nodes all holding `val`.
    fn build_left_chain(n: usize, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        (0..n).fold(None, |child, _| {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = child;
            Some(node)
        })
    }

    #[test]
    fn test_example_1() {
        // [4,8,5,0,1,null,6] -> nodes 4, 5, 0, 1, 6 match.
        let tree = build_tree(&[Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)]);
        assert_eq!(Solution::average_of_subtree(tree), 5);
    }

    #[test]
    fn test_example_2() {
        // [1] -> the single node always matches.
        let tree = build_tree(&[Some(1)]);
        assert_eq!(Solution::average_of_subtree(tree), 1);
    }

    #[test]
    fn test_leaves_always_match() {
        // [1,2,3]: leaves 2 and 3 match; root averages (1+2+3)/3 = 2 != 1.
        let tree = build_tree(&[Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::average_of_subtree(tree), 2);
    }

    #[test]
    fn test_floor_division() {
        // [5,null,6]: root averages (5+6)/2 = 5 (floored), leaf 6 matches.
        let tree = build_tree(&[Some(5), None, Some(6)]);
        assert_eq!(Solution::average_of_subtree(tree), 2);
    }

    #[test]
    fn test_all_zero_values() {
        // Minimum node values: every subtree averages 0.
        let tree = build_tree(&[Some(0), Some(0), Some(0), Some(0)]);
        assert_eq!(Solution::average_of_subtree(tree), 4);
    }

    #[test]
    fn test_no_internal_match() {
        // [1,0,0]: only the two leaves match, root averages (1+0+0)/3 = 0.
        let tree = build_tree(&[Some(1), Some(0), Some(0)]);
        assert_eq!(Solution::average_of_subtree(tree), 2);
    }

    #[test]
    fn test_max_constraints_skewed_chain() {
        // Deepest allowed shape with maximum values: all 1000 nodes match.
        let tree = build_left_chain(1000, 1000);
        assert_eq!(Solution::average_of_subtree(tree), 1000);
    }
}
