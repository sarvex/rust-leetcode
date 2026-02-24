use std::cell::RefCell;
use std::rc::Rc;

// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

impl Solution {
    /// Recursive midpoint selection for height-balanced BST construction.
    ///
    /// # Intuition
    /// Choosing the middle element as root ensures equal-sized subtrees,
    /// producing a height-balanced BST. Recursing on left and right halves
    /// builds the entire tree.
    ///
    /// # Approach
    /// Binary-divide the array. The middle element becomes the root. Left
    /// half builds the left subtree, right half builds the right subtree.
    /// Use index ranges to avoid copying slices.
    ///
    /// # Complexity
    /// - Time: O(n) - each element becomes a node exactly once
    /// - Space: O(log n) - recursion depth for a balanced tree
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &[i32], left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if left > right || left >= nums.len() {
                return None;
            }
            let mid = left + (right - left) / 2;
            let mut node = TreeNode::new(nums[mid]);
            if mid > 0 {
                node.left = dfs(nums, left, mid - 1);
            }
            node.right = dfs(nums, mid + 1, right);
            Some(Rc::new(RefCell::new(node)))
        }

        if nums.is_empty() {
            return None;
        }
        dfs(&nums, 0, nums.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_bst(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let val = node.val as i64;
                val > min
                    && val < max
                    && is_valid_bst(&node.left, min, val)
                    && is_valid_bst(&node.right, val, max)
            }
        }
    }

    fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                1 + get_height(&node.left).max(get_height(&node.right))
            }
        }
    }

    fn is_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let left_height = get_height(&node.left);
                let right_height = get_height(&node.right);
                (left_height - right_height).abs() <= 1
                    && is_balanced(&node.left)
                    && is_balanced(&node.right)
            }
        }
    }

    fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                1 + count_nodes(&node.left) + count_nodes(&node.right)
            }
        }
    }

    #[test]
    fn test_example() {
        let nums = vec![-10, -3, 0, 5, 9];
        let tree = Solution::sorted_array_to_bst(nums.clone());
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), nums.len());
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        let tree = Solution::sorted_array_to_bst(nums);
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), 1);
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, 3];
        let tree = Solution::sorted_array_to_bst(nums.clone());
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), nums.len());
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        let tree = Solution::sorted_array_to_bst(nums);
        assert!(tree.is_none());
    }

    #[test]
    fn test_larger_array() {
        let nums: Vec<i32> = (1..=15).collect();
        let tree = Solution::sorted_array_to_bst(nums.clone());
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), nums.len());
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-5, -3, -1, 0, 2, 4, 6];
        let tree = Solution::sorted_array_to_bst(nums.clone());
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), nums.len());
    }
}
