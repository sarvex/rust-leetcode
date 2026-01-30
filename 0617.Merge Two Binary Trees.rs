use std::cell::RefCell;
use std::rc::Rc;

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
    /// Merges two binary trees by summing overlapping nodes.
    ///
    /// # Intuition
    /// Recursively merge corresponding nodes. Where only one tree has a node,
    /// use that subtree directly.
    ///
    /// # Approach
    /// 1. Match on both roots.
    /// 2. If both present, sum values and recurse on children.
    /// 3. If only one present, return that subtree.
    ///
    /// # Complexity
    /// - Time: O(min(m, n)) where m, n are node counts
    /// - Space: O(min(h1, h2)) recursion depth
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(r), None) | (None, Some(r)) => Some(r),
            (Some(r1), Some(r2)) => {
                {
                    let mut n1 = r1.borrow_mut();
                    let mut n2 = r2.borrow_mut();
                    n1.val += n2.val;
                    n1.left = Self::merge_trees(n1.left.take(), n2.left.take());
                    n1.right = Self::merge_trees(n1.right.take(), n2.right.take());
                }
                Some(r1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();
            
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(n) = node {
                let n = n.borrow();
                result.push(Some(n.val));
                queue.push_back(n.left.clone());
                queue.push_back(n.right.clone());
            } else {
                result.push(None);
            }
        }

        // Remove trailing None values
        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }

    #[test]
    fn test_merge_trees_full_overlap() {
        // Tree1: [1,3,2,5]
        //        1
        //       / \
        //      3   2
        //     /
        //    5
        // Tree2: [2,1,3,null,4,null,7]
        //        2
        //       / \
        //      1   3
        //       \   \
        //        4   7
        // Merged: [3,4,5,5,4,null,7]
        //        3
        //       / \
        //      4   5
        //     / \   \
        //    5   4   7
        let root1 = build_tree(&[Some(1), Some(3), Some(2), Some(5)]);
        let root2 = build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(7)]);
        let merged = Solution::merge_trees(root1, root2);
        let expected = build_tree(&[Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)]);
        assert_eq!(tree_to_vec(&merged), tree_to_vec(&expected));
    }

    #[test]
    fn test_merge_trees_one_empty() {
        // Tree1: [1]
        // Tree2: []
        // Merged: [1]
        let root1 = build_tree(&[Some(1)]);
        let root2 = build_tree(&[]);
        let merged = Solution::merge_trees(root1, root2);
        let expected = build_tree(&[Some(1)]);
        assert_eq!(tree_to_vec(&merged), tree_to_vec(&expected));
    }

    #[test]
    fn test_merge_trees_both_empty() {
        // Tree1: []
        // Tree2: []
        // Merged: []
        let root1 = build_tree(&[]);
        let root2 = build_tree(&[]);
        let merged = Solution::merge_trees(root1, root2);
        assert!(merged.is_none());
    }

    #[test]
    fn test_merge_trees_partial_overlap() {
        // Tree1: [1,2,null,3]
        //        1
        //       /
        //      2
        //     /
        //    3
        // Tree2: [1,null,2,null,3]
        //        1
        //         \
        //          2
        //           \
        //            3
        // Merged: [2,2,2,3,null,null,3]
        //        2
        //       / \
        //      2   2
        //     /     \
        //    3       3
        let root1 = build_tree(&[Some(1), Some(2), None, Some(3)]);
        let root2 = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
        let merged = Solution::merge_trees(root1, root2);
        let expected = build_tree(&[Some(2), Some(2), Some(2), Some(3), None, None, Some(3)]);
        assert_eq!(tree_to_vec(&merged), tree_to_vec(&expected));
    }

    #[test]
    fn test_merge_trees_single_nodes() {
        // Tree1: [5]
        // Tree2: [10]
        // Merged: [15]
        let root1 = build_tree(&[Some(5)]);
        let root2 = build_tree(&[Some(10)]);
        let merged = Solution::merge_trees(root1, root2);
        let expected = build_tree(&[Some(15)]);
        assert_eq!(tree_to_vec(&merged), tree_to_vec(&expected));
    }
}
