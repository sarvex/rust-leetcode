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
use std::rc::Rc;

impl Solution {
    /// Constructs a maximum binary tree from the array recursively.
    ///
    /// # Intuition
    /// The root is the maximum element. Left subtree is built from elements
    /// before the max, right subtree from elements after.
    ///
    /// # Approach
    /// 1. Find the index of the maximum in the range.
    /// 2. Create a node with that value.
    /// 3. Recurse on left and right sub-ranges.
    ///
    /// # Complexity
    /// - Time: O(n²) worst case, O(n log n) average
    /// - Space: O(n) recursion depth
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }
            let (idx, &max_val) = nums.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: max_val,
                left: build(&nums[..idx]),
                right: build(&nums[idx + 1..]),
            })))
        }
        build(&nums)
    }
}
