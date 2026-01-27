use std::cell::RefCell;
use std::rc::Rc;

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

        fn subtree_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    n.val as i64 + subtree_sum(&n.left) + subtree_sum(&n.right)
                }
                None => 0,
            }
        }

        fn find_max(node: &Option<Rc<RefCell<TreeNode>>>, total: i64, best: &mut i64) -> i64 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let sum = n.val as i64
                        + find_max(&n.left, total, best)
                        + find_max(&n.right, total, best);
                    *best = (*best).max(sum * (total - sum));
                    sum
                }
                None => 0,
            }
        }

        let total = subtree_sum(&root);
        let mut best = 0i64;
        find_max(&root, total, &mut best);
        (best % MOD) as i32
    }
}
