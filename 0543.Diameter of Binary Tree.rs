use std::cell::RefCell;
use std::rc::Rc;

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


impl Solution {
    /// Computes the diameter of a binary tree via DFS depth calculation.
    ///
    /// # Intuition
    /// The diameter through any node is the sum of left and right subtree
    /// depths. Track the global maximum while computing depths recursively.
    ///
    /// # Approach
    /// 1. DFS returns the depth of each subtree.
    /// 2. At each node, update the diameter as left_depth + right_depth.
    /// 3. Return the maximum diameter seen.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    let left = dfs(&inner.left, diameter);
                    let right = dfs(&inner.right, diameter);
                    *diameter = (*diameter).max(left + right);
                    1 + left.max(right)
                }
            }
        }
        let mut diameter = 0;
        dfs(&root, &mut diameter);
        diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;

        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();
            
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            
            if i < values.len() {
                if let Some(val) = values[i] {
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
    fn test_diameter_balanced_tree() {
        // Tree: [1,2,3,4,5]
        //       1
        //      / \
        //     2   3
        //    / \
        //   4   5
        // Diameter: 3 (path: 4->2->1->3 or 5->2->1->3)
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_skewed_tree() {
        // Tree: [1,2]
        //       1
        //      /
        //     2
        // Diameter: 1 (path: 2->1)
        let root = build_tree(&[Some(1), Some(2)]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 1);
    }

    #[test]
    fn test_diameter_single_node() {
        // Tree: [1]
        // Diameter: 0 (no path with two nodes)
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 0);
    }

    #[test]
    fn test_diameter_empty_tree() {
        // Empty tree
        // Diameter: 0
        let root = build_tree(&[]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 0);
    }

    #[test]
    fn test_diameter_complex_tree() {
        // Tree: [1,2,3,4,5]
        //         1
        //        / \
        //       2   3
        //      / \
        //     4   5
        // The maximum diameter is the longest path, which is 3 edges (4->2->1->3 or 5->2->1->3)
        // Note: This tree has depth 2 on left (through node 2) and depth 1 on right (node 3)
        // The diameter through root 1 is 2 + 1 = 3
        let root = build_tree(&[
            Some(1), Some(2), Some(3), Some(4), Some(5)
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_path_not_through_root() {
        // Tree: [1,2,null,3,4,5,6]
        //       1
        //      /
        //     2
        //    / \
        //   3   4
        //  / \
        // 5   6
        // Diameter: 3 (path: 5->3->2->4)
        let root = build_tree(&[
            Some(1), Some(2), None, Some(3), Some(4), Some(5), Some(6)
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_deeper_tree() {
        // Tree with a deeper left subtree
        //         1
        //        / \
        //       2   3
        //      / \
        //     4   5
        //    / \
        //   6   7
        // Diameter: 4 (path: 6->4->2->1->3 or 6->4->2->5 etc.)
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        
        root.borrow_mut().left = Some(node2.clone());
        root.borrow_mut().right = Some(node3.clone());
        node2.borrow_mut().left = Some(node4.clone());
        node2.borrow_mut().right = Some(node5.clone());
        node4.borrow_mut().left = Some(node6.clone());
        node4.borrow_mut().right = Some(node7.clone());
        
        assert_eq!(Solution::diameter_of_binary_tree(Some(root)), 4);
    }
}
