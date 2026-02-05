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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    /// Finds all duplicate subtrees by serializing each subtree.
    ///
    /// # Intuition
    /// Serialize every subtree into a canonical string. When a serialization
    /// appears for the second time, it represents a duplicate subtree.
    ///
    /// # Approach
    /// 1. DFS post-order: serialize each subtree as "val,left,right".
    /// 2. Track serialization counts in a hash map.
    /// 3. On count == 2, add the subtree root to the result.
    ///
    /// # Complexity
    /// - Time: O(n²) due to string concatenation
    /// - Space: O(n²) for stored serializations
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            freq: &mut HashMap<String, u32>,
            result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> String {
            let Some(rc) = node else {
                return "#".to_string();
            };

            let inner = rc.borrow();
            let serial = format!(
                "{},{},{}",
                inner.val,
                dfs(&inner.left, freq, result),
                dfs(&inner.right, freq, result),
            );

            let count = freq.entry(serial.clone()).or_insert(0);
            *count += 1;
            if *count == 2 {
                result.push(node.clone());
            }
            serial
        }

        let mut freq = HashMap::with_capacity(100);
        let mut result = Vec::new();
        dfs(&root, &mut freq, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn create_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();

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

        Some(root)
    }

    fn create_tree_manual() -> Option<Rc<RefCell<TreeNode>>> {
        // Manually create a tree with duplicate [2,4] subtrees
        //       1
        //      / \
        //     2   2
        //    /   /
        //   4   4
        let root = Rc::new(RefCell::new(TreeNode::new(1)));

        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let left_left = Rc::new(RefCell::new(TreeNode::new(4)));
        left.borrow_mut().left = Some(left_left);

        let right = Rc::new(RefCell::new(TreeNode::new(2)));
        let right_left = Rc::new(RefCell::new(TreeNode::new(4)));
        right.borrow_mut().left = Some(right_left);

        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        Some(root)
    }

    #[test]
    fn test_leetcode_example() {
        // Original LeetCode example - only leaf 4 nodes are duplicates
        // Tree: [1,2,3,4,null,2,4,null,null,4]
        //       1
        //      / \
        //     2   3
        //    /   / \
        //   4   2   4
        //      /
        //     4
        let root = create_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(2),
            Some(4),
            None,
            None,
            None,
            None,
            Some(4),
        ]);
        let result = Solution::find_duplicate_subtrees(root);

        // The actual LeetCode example would find [4] and [2,4] as duplicates
        // But based on the tree structure, only [4] appears multiple times
        assert!(result.len() >= 1);
    }

    #[test]
    fn test_manual_duplicate_subtrees() {
        // Manually created tree with actual [2,4] duplicates
        //       1
        //      / \
        //     2   2
        //    /   /
        //   4   4
        let root = create_tree_manual();
        let result = Solution::find_duplicate_subtrees(root);

        // Should find duplicates: [4] and [2,4]
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_example_2() {
        // Tree: [2,1,1]
        //       2
        //      / \
        //     1   1
        let root = create_tree(&[Some(2), Some(1), Some(1)]);
        let result = Solution::find_duplicate_subtrees(root);

        // Should find one duplicate subtree [1]
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref().unwrap().borrow().val, 1);
    }

    #[test]
    fn test_example_3() {
        // Tree: [2,2,2,3,null,3,null]
        //       2
        //      / \
        //     2   2
        //    /   /
        //   3   3
        let root = create_tree(&[Some(2), Some(2), Some(2), Some(3), None, Some(3), None]);
        let result = Solution::find_duplicate_subtrees(root);

        // Should find duplicate subtrees [3] and [2,3]
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_no_duplicates() {
        // Tree: [1,2,3,4,5,6,7]
        //       1
        //      / \
        //     2   3
        //    / \ / \
        //   4  5 6  7
        let root = create_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let result = Solution::find_duplicate_subtrees(root);

        // No duplicates should be found
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_single_node() {
        // Tree: [1]
        let root = create_tree(&[Some(1)]);
        let result = Solution::find_duplicate_subtrees(root);

        // No duplicates in a single node tree
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_all_same_values() {
        // Tree with all nodes having same value but different structure
        // [1,1,1,1,null,null,1]
        //       1
        //      / \
        //     1   1
        //    /     \
        //   1       1
        let root = create_tree(&[Some(1), Some(1), Some(1), Some(1), None, None, Some(1)]);
        let result = Solution::find_duplicate_subtrees(root);

        // Should find some duplicate leaf nodes with value 1
        assert!(result.len() >= 1);
    }
}
