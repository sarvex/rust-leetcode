use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Bottom-up height check with early termination for balanced tree validation.
    ///
    /// # Intuition
    /// A balanced tree has subtree heights differing by at most 1 at every node.
    /// Computing height bottom-up and returning -1 on imbalance propagates
    /// failure without redundant traversals.
    ///
    /// # Approach
    /// DFS returns the height of each subtree, or -1 if any subtree is
    /// unbalanced. At each node, if either child returns -1 or the height
    /// difference exceeds 1, return -1. Otherwise return `1 + max(left, right)`.
    ///
    /// # Complexity
    /// - Time: O(n) - each node visited once
    /// - Space: O(h) - recursion depth equals tree height
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left = height(&n.left);
                    let right = height(&n.right);
                    if left == -1 || right == -1 || (left - right).abs() > 1 {
                        -1
                    } else {
                        1 + left.max(right)
                    }
                }
            }
        }

        height(&root) != -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn tree_from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = VecDeque::with_capacity(values.len());
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();

            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_balanced_tree() {
        // Tree: [3,9,20,null,null,15,7]
        //       3
        //      / \
        //     9   20
        //        /  \
        //       15   7
        let root = tree_from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test_unbalanced_tree() {
        // Tree: [1,2,2,3,3,null,null,4,4]
        //         1
        //        / \
        //       2   2
        //      / \
        //     3   3
        //    / \
        //   4   4
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn test_empty_tree() {
        let root = tree_from_vec(vec![]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test_single_node() {
        let root = tree_from_vec(vec![Some(1)]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test_perfect_tree() {
        // Tree: [1,2,3,4,5,6,7]
        //         1
        //       /   \
        //      2     3
        //     / \   / \
        //    4   5 6   7
        let root = tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test_left_heavy() {
        // Tree: [1,2,null,3]
        //       1
        //      /
        //     2
        //    /
        //   3
        let root = tree_from_vec(vec![Some(1), Some(2), None, Some(3)]);
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn test_right_heavy() {
        // Tree: [1,null,2,null,3]
        //   1
        //    \
        //     2
        //      \
        //       3
        let root = tree_from_vec(vec![Some(1), None, Some(2), None, Some(3)]);
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn test_almost_balanced() {
        // Tree: [1,2,3,4]
        //       1
        //      / \
        //     2   3
        //    /
        //   4
        // This is balanced (height diff is 1 at root)
        let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4)]);
        assert!(Solution::is_balanced(root));
    }
}
