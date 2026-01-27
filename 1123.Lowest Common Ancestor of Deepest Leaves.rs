use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Finds LCA of deepest leaves using DFS with depth tracking.
    ///
    /// # Intuition
    /// The LCA of deepest leaves is the node where both subtrees reach the
    /// maximum depth. If one subtree is deeper, propagate that result upward.
    ///
    /// # Approach
    /// DFS returning `(depth, lca_candidate)`. If left and right depths are
    /// equal, the current node is the LCA. Otherwise return the deeper side.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(n) => {
                let borrowed = n.borrow();
                let (ld, ll) = Self::dfs(&borrowed.left);
                let (rd, rl) = Self::dfs(&borrowed.right);
                match ld.cmp(&rd) {
                    std::cmp::Ordering::Equal => (ld + 1, Some(n.clone())),
                    std::cmp::Ordering::Greater => (ld + 1, ll),
                    std::cmp::Ordering::Less => (rd + 1, rl),
                }
            }
        }
    }
}
