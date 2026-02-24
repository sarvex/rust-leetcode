use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    /// Converts a sorted linked list to a height-balanced BST via array conversion and recursive midpoint splitting.
    ///
    /// # Intuition
    /// A sorted list maps directly to a balanced BST by choosing the middle element as root.
    /// Collecting values into an array enables O(1) midpoint access.
    ///
    /// # Approach
    /// 1. Traverse the linked list and collect all values into a vector.
    /// 2. Recursively select the midpoint of each subarray as root.
    /// 3. Build left and right subtrees from the left and right halves.
    ///
    /// # Complexity
    /// - Time: O(n) for list traversal plus O(n) for tree construction
    /// - Space: O(n) for the values vector plus O(log n) recursion stack
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Count nodes first for optimal capacity
        let mut count = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            count += 1;
            cur = &node.next;
        }

        let mut vals = Vec::with_capacity(count);
        let mut cur = &head;
        while let Some(node) = cur {
            vals.push(node.val);
            cur = &node.next;
        }
        Self::build(&vals, 0, vals.len())
    }

    fn build(vals: &[i32], start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return None;
        }
        let mid = (start + end) >> 1;
        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[mid],
            left: Self::build(vals, start, mid),
            right: Self::build(vals, mid + 1, end),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in values.into_iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

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
        let list = vec_to_list(vec![-10, -3, 0, 5, 9]);
        let tree = Solution::sorted_list_to_bst(list);
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), 5);
    }

    #[test]
    fn test_single_element() {
        let list = vec_to_list(vec![1]);
        let tree = Solution::sorted_list_to_bst(list);
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), 1);
    }

    #[test]
    fn test_two_elements() {
        let list = vec_to_list(vec![1, 3]);
        let tree = Solution::sorted_list_to_bst(list);
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), 2);
    }

    #[test]
    fn test_empty() {
        let list = vec_to_list(vec![]);
        let tree = Solution::sorted_list_to_bst(list);
        assert!(tree.is_none());
    }

    #[test]
    fn test_longer_list() {
        let list = vec_to_list(vec![1, 2, 3, 4, 5, 6, 7]);
        let tree = Solution::sorted_list_to_bst(list);
        assert!(is_valid_bst(&tree, i64::MIN, i64::MAX));
        assert!(is_balanced(&tree));
        assert_eq!(count_nodes(&tree), 7);
    }
}
