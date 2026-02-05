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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Finds the lowest common ancestor of two nodes using recursive DFS.
    ///
    /// # Intuition
    /// If the current node is p or q, it is part of the answer. If both
    /// subtrees return non-None, the current node is the LCA.
    ///
    /// # Approach
    /// 1. Base case: return root if it is None, p, or q.
    /// 2. Recurse into left and right subtrees.
    /// 3. If both return a node, root is the LCA.
    /// 4. Otherwise, return whichever subtree found a match.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once in worst case
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let node = root.as_ref().unwrap().borrow();
        let left = Self::lowest_common_ancestor(node.left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(node.right.clone(), p, q);
        match (&left, &right) {
            (Some(_), Some(_)) => {
                drop(node);
                root
            }
            (None, _) => right,
            _ => left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_tree_with_map(
        vals: Vec<Option<i32>>,
    ) -> (
        Option<Rc<RefCell<TreeNode>>>,
        HashMap<i32, Rc<RefCell<TreeNode>>>,
    ) {
        let mut map = HashMap::new();
        if vals.is_empty() || vals[0].is_none() {
            return (None, map);
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        map.insert(vals[0].unwrap(), Rc::clone(&root));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    map.insert(val, Rc::clone(&left));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    map.insert(val, Rc::clone(&right));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        (Some(root), map)
    }

    #[test]
    fn test_lca_in_different_subtrees() {
        // Test tree: [3,5,1,6,2,0,8,null,null,7,4]
        //       3
        //      / \
        //     5   1
        //    / \ / \
        //   6  2 0  8
        //     / \
        //    7   4
        // LCA of 5 and 1 is 3
        let (root, map) = build_tree_with_map(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = Some(Rc::clone(&map[&5]));
        let q = Some(Rc::clone(&map[&1]));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_lca_ancestor_descendant() {
        // Test tree: [3,5,1,6,2,0,8,null,null,7,4]
        // LCA of 5 and 4 is 5 (5 is ancestor of 4)
        let (root, map) = build_tree_with_map(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = Some(Rc::clone(&map[&5]));
        let q = Some(Rc::clone(&map[&4]));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 5);
    }

    #[test]
    fn test_lca_simple_tree() {
        // Test tree: [1,2]
        //     1
        //    /
        //   2
        // LCA of 1 and 2 is 1
        let (root, map) = build_tree_with_map(vec![Some(1), Some(2)]);
        let p = Some(Rc::clone(&map[&1]));
        let q = Some(Rc::clone(&map[&2]));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_lca_leaf_nodes() {
        // Test tree: [6,2,8,0,4,7,9,null,null,3,5]
        //       6
        //      / \
        //     2   8
        //    / \ / \
        //   0  4 7  9
        //     / \
        //    3   5
        // LCA of 3 and 5 is 4
        let (root, map) = build_tree_with_map(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = Some(Rc::clone(&map[&3]));
        let q = Some(Rc::clone(&map[&5]));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 4);
    }

    #[test]
    fn test_lca_deep_tree() {
        // Test tree: [1,2,3,4,null,null,null,5]
        //       1
        //      / \
        //     2   3
        //    /
        //   4
        //  /
        // 5
        // LCA of 5 and 3 is 1
        let (root, map) = build_tree_with_map(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        let p = Some(Rc::clone(&map[&5]));
        let q = Some(Rc::clone(&map[&3]));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 1);
    }
}
