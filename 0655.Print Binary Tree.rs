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
    /// Prints a binary tree in a 2-D grid layout.
    ///
    /// # Intuition
    /// The grid has height+1 rows and 2^(height+1) - 1 columns. Each node
    /// is placed at the midpoint of its column range, splitting the range
    /// for its children.
    ///
    /// # Approach
    /// 1. Compute the tree height.
    /// 2. Create the grid of empty strings.
    /// 3. DFS to place each node's value at its computed column.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(2^h * h) for the grid
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> u32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    1 + height(&inner.left).max(height(&inner.right))
                }
            }
        }

        fn fill(
            node: &Option<Rc<RefCell<TreeNode>>>,
            row: usize,
            col: usize,
            offset: usize,
            grid: &mut Vec<Vec<String>>,
        ) {
            if let Some(rc) = node {
                let inner = rc.borrow();
                grid[row][col] = inner.val.to_string();
                if offset > 0 {
                    fill(&inner.left, row + 1, col - offset, offset / 2, grid);
                    fill(&inner.right, row + 1, col + offset, offset / 2, grid);
                }
            }
        }

        let h = height(&root);
        let rows = h as usize;
        let cols = (1usize << h) - 1;
        let mut grid = vec![vec![String::new(); cols]; rows];
        fill(&root, 0, (cols - 1) / 2, (cols + 1) / 4, &mut grid);
        grid
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

    #[test]
    fn test_example_1() {
        // Tree: [1,2]
        //       1
        //      /
        //     2
        // Expected:
        // [["", "1", ""],
        //  ["2", "", ""]]
        let root = create_tree(&[Some(1), Some(2)]);
        let result = Solution::print_tree(root);
        assert_eq!(
            result,
            vec![
                vec!["".to_string(), "1".to_string(), "".to_string()],
                vec!["2".to_string(), "".to_string(), "".to_string()]
            ]
        );
    }

    #[test]
    fn test_example_2() {
        // Tree: [1,2,3,null,4]
        //       1
        //      / \
        //     2   3
        //      \
        //       4
        // Expected: 7 columns wide, 3 rows
        let root = create_tree(&[Some(1), Some(2), Some(3), None, Some(4)]);
        let result = Solution::print_tree(root);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 7);
        assert_eq!(result[0][3], "1");
        assert_eq!(result[1][1], "2");
        assert_eq!(result[1][5], "3");
        assert_eq!(result[2][2], "4");
    }

    #[test]
    fn test_single_node() {
        let root = create_tree(&[Some(1)]);
        let result = Solution::print_tree(root);
        assert_eq!(result, vec![vec!["1".to_string()]]);
    }

    #[test]
    fn test_full_binary_tree() {
        // Tree: [1,2,3]
        //       1
        //      / \
        //     2   3
        let root = create_tree(&[Some(1), Some(2), Some(3)]);
        let result = Solution::print_tree(root);
        assert_eq!(
            result,
            vec![
                vec!["".to_string(), "1".to_string(), "".to_string()],
                vec!["2".to_string(), "".to_string(), "3".to_string()]
            ]
        );
    }

    #[test]
    fn test_right_skewed() {
        // Tree: [1,null,2,null,3]
        //       1
        //        \
        //         2
        //          \
        //           3
        let root = create_tree(&[Some(1), None, Some(2), None, Some(3)]);
        let result = Solution::print_tree(root);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 7);
        // Root at center (index 3)
        assert_eq!(result[0][3], "1");
        // Right child at index 5
        assert_eq!(result[1][5], "2");
        // Right-right child at index 6
        assert_eq!(result[2][6], "3");
    }

    #[test]
    fn test_negative_values() {
        // Tree: [-1,-2,-3]
        let root = create_tree(&[Some(-1), Some(-2), Some(-3)]);
        let result = Solution::print_tree(root);
        assert_eq!(result[0][1], "-1");
        assert_eq!(result[1][0], "-2");
        assert_eq!(result[1][2], "-3");
    }

    #[test]
    fn test_empty_tree() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result = Solution::print_tree(root);
        assert!(result.is_empty() || (result.len() == 0));
    }
}
