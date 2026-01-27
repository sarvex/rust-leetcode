// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
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
