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
    /// - Time: O(n^2) worst case, O(n log n) average
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
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
    fn test_example_1() {
        // Input: [3,2,1,6,0,5]
        // Output: [6,3,5,null,2,0,null,null,1]
        //         6
        //       /   \
        //      3     5
        //       \   /
        //        2 0
        //         \
        //          1
        let nums = vec![3, 2, 1, 6, 0, 5];
        let result = Solution::construct_maximum_binary_tree(nums);
        let output = tree_to_vec(&result);
        assert_eq!(
            output,
            vec![
                Some(6),
                Some(3),
                Some(5),
                None,
                Some(2),
                Some(0),
                None,
                None,
                Some(1)
            ]
        );
    }

    #[test]
    fn test_example_2() {
        // Input: [3,2,1]
        // Output: [3,null,2,null,1]
        //         3
        //          \
        //           2
        //            \
        //             1
        let nums = vec![3, 2, 1];
        let result = Solution::construct_maximum_binary_tree(nums);
        let output = tree_to_vec(&result);
        assert_eq!(output, vec![Some(3), None, Some(2), None, Some(1)]);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        let result = Solution::construct_maximum_binary_tree(nums);
        assert_eq!(result.as_ref().unwrap().borrow().val, 42);
        assert!(result.as_ref().unwrap().borrow().left.is_none());
        assert!(result.as_ref().unwrap().borrow().right.is_none());
    }

    #[test]
    fn test_sorted_ascending() {
        // Sorted ascending creates a right-skewed tree
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::construct_maximum_binary_tree(nums);
        let output = tree_to_vec(&result);
        assert_eq!(
            output,
            vec![
                Some(5),
                Some(4),
                None,
                Some(3),
                None,
                Some(2),
                None,
                Some(1)
            ]
        );
    }

    #[test]
    fn test_sorted_descending() {
        // Sorted descending creates a left-skewed tree
        let nums = vec![5, 4, 3, 2, 1];
        let result = Solution::construct_maximum_binary_tree(nums);
        let output = tree_to_vec(&result);
        assert_eq!(
            output,
            vec![
                Some(5),
                None,
                Some(4),
                None,
                Some(3),
                None,
                Some(2),
                None,
                Some(1)
            ]
        );
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, 2];
        let result = Solution::construct_maximum_binary_tree(nums);
        assert_eq!(result.as_ref().unwrap().borrow().val, 2);
        assert_eq!(
            result
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            1
        );
        assert!(result.as_ref().unwrap().borrow().right.is_none());
    }

    #[test]
    fn test_max_at_beginning() {
        let nums = vec![9, 1, 2, 3];
        let result = Solution::construct_maximum_binary_tree(nums);
        let output = tree_to_vec(&result);
        assert_eq!(output, vec![Some(9), None, Some(3), Some(2), None, Some(1)]);
    }
}
