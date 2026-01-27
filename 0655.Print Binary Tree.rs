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
    /// - Space: O(2^h × h) for the grid
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
