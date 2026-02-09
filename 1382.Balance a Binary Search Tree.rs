use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

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
    /// Inorder extraction then recursive midpoint build for a balanced BST.
    ///
    /// # Intuition
    /// BST inorder gives sorted values. A balanced BST with those values is
    /// built by repeatedly choosing the middle element as root so subtree
    /// sizes differ by at most one.
    ///
    /// # Approach
    /// 1. Inorder traverse the tree to collect values into a sorted slice.
    /// 2. Build a new tree from the sorted slice: root = mid, left = build(left
    ///    half), right = build(right half). Same idea as "Convert Sorted Array
    ///    to BST".
    ///
    /// # Complexity
    /// - Time: O(n) — one inorder pass, then one node per value.
    /// - Space: O(n) — values vector and O(log n) recursion for build.
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                inorder(&n.left, out);
                out.push(n.val);
                inorder(&n.right, out);
            }
        }

        fn build(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if slice.is_empty() {
                return None;
            }
            let mid = slice.len() / 2;
            let mut node = TreeNode::new(slice[mid]);
            node.left = build(&slice[..mid]);
            node.right = build(&slice[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }

        let mut values = Vec::new();
        inorder(&root, &mut values);
        build(&values)
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

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                1 + height(&n.left).max(height(&n.right))
            }
        }
    }

    fn is_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(n) => {
                let n = n.borrow();
                (height(&n.left) - height(&n.right)).abs() <= 1
                    && is_balanced(&n.left)
                    && is_balanced(&n.right)
            }
        }
    }

    fn is_bst(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            None => true,
            Some(n) => {
                let n = n.borrow();
                let v = n.val as i64;
                v > min && v < max && is_bst(&n.left, min, v) && is_bst(&n.right, v, max)
            }
        }
    }

    fn collect_inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = Vec::new();
        fn go(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                go(&n.left, out);
                out.push(n.val);
                go(&n.right, out);
            }
        }
        go(root, &mut out);
        out
    }

    #[test]
    fn test_example1() {
        // [1,null,2,null,3,null,4] -> balanced BST with same values
        let root = tree_from_vec(vec![Some(1), None, Some(2), None, Some(3), None, Some(4)]);
        let balanced = Solution::balance_bst(root);
        let values = collect_inorder(&balanced);
        assert_eq!(values, vec![1, 2, 3, 4]);
        assert!(is_bst(&balanced, i64::MIN, i64::MAX));
        assert!(is_balanced(&balanced));
    }

    #[test]
    fn test_example2() {
        // [2,1,3] already balanced
        let root = tree_from_vec(vec![Some(2), Some(1), Some(3)]);
        let balanced = Solution::balance_bst(root);
        let values = collect_inorder(&balanced);
        assert_eq!(values, vec![1, 2, 3]);
        assert!(is_bst(&balanced, i64::MIN, i64::MAX));
        assert!(is_balanced(&balanced));
    }

    #[test]
    fn test_single_node() {
        let root = tree_from_vec(vec![Some(1)]);
        let balanced = Solution::balance_bst(root);
        assert_eq!(collect_inorder(&balanced), vec![1]);
        assert!(is_balanced(&balanced));
    }

    #[test]
    fn test_two_nodes() {
        let root = tree_from_vec(vec![Some(1), None, Some(2)]);
        let balanced = Solution::balance_bst(root);
        assert_eq!(collect_inorder(&balanced), vec![1, 2]);
        assert!(is_bst(&balanced, i64::MIN, i64::MAX));
        assert!(is_balanced(&balanced));
    }
}
