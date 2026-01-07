use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Maximum Product of Splitted Binary Tree
    ///
    /// # Intuition
    /// When removing an edge, we split the tree into two subtrees with sums S and (total - S).
    /// The product S * (total - S) is maximized when S is as close to total/2 as possible.
    ///
    /// # Approach
    /// 1. First DFS pass: Calculate the total sum of all nodes
    /// 2. Second DFS pass: Compute each subtree sum and track the maximum product
    ///    - For each subtree with sum S, the product would be S * (total - S)
    ///    - Update max_product whenever we find a larger product
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes (two tree traversals)
    /// - Space: O(h) where h is the height of the tree (recursion stack)
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let subtree_sum = |node: &Option<Rc<RefCell<TreeNode>>>| -> i64 {
            fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
                match node {
                    Some(n) => {
                        let n = n.borrow();
                        n.val as i64 + dfs(&n.left) + dfs(&n.right)
                    }
                    None => 0,
                }
            }
            dfs(node)
        };

        let total = subtree_sum(&root);
        let mut max_product: i64 = 0;

        let mut find_max = |node: &Option<Rc<RefCell<TreeNode>>>| -> i64 {
            fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, total: i64, max_product: &mut i64) -> i64 {
                match node {
                    Some(n) => {
                        let n = n.borrow();
                        let sum = n.val as i64
                            + dfs(&n.left, total, max_product)
                            + dfs(&n.right, total, max_product);
                        *max_product = (*max_product).max(sum * (total - sum));
                        sum
                    }
                    None => 0,
                }
            }
            dfs(node, total, &mut max_product)
        };

        find_max(&root);

        (max_product % MOD) as i32
    }
}
