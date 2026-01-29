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

pub struct Solution;

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
            freq: &mut HashMap<String, i32>,
            result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> String {
            match node {
                None => "#".to_string(),
                Some(rc) => {
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
            }
        }

        let mut freq = HashMap::new();
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

    fn tree_to_values(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Helper to extract values from a tree for comparison
        let mut result = Vec::new();
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node.clone());
            while let Some(current) = queue.pop_front() {
                let inner = current.borrow();
                result.push(inner.val);
                if let Some(left) = &inner.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &inner.right {
                    queue.push_back(right.clone());
                }
            }
        }
        result
    }

    #[test]
    fn test_example_1() {
        // Tree: [1,2,3,4,null,2,4,null,null,4]
        //       1
        //      / \
        //     2   3
        //    /   / \
        //   4   2   4
        //      /
        //     4
        let root = create_tree(&[
            Some(1), Some(2), Some(3), Some(4), None, Some(2), Some(4), 
            None, None, None, None, Some(4)
        ]);
        let result = Solution::find_duplicate_subtrees(root);
        
        // Should find duplicate subtrees with values [4] and [2,4]
        assert_eq!(result.len(), 2);
        
        let values: Vec<Vec<i32>> = result.iter()
            .map(|node| tree_to_values(node))
            .collect();
        
        // One duplicate should be a single node [4]
        assert!(values.iter().any(|v| v == &vec![4]));
        // Another should be subtree [2,4]
        assert!(values.iter().any(|v| v.len() == 2 && v[0] == 2 && v[1] == 4));
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
        let values = tree_to_values(&result[0]);
        assert_eq!(values, vec![1]);
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
        let root = create_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
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
}
