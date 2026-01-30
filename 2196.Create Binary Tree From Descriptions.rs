use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

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


impl Solution {
    /// Build a binary tree from parent-child-direction descriptions.
    ///
    /// # Intuition
    /// Store all nodes in a map, track which values appear as children, and
    /// the root is the node that never appears as a child.
    ///
    /// # Approach
    /// 1. Create or retrieve tree nodes from a HashMap keyed by value.
    /// 2. Attach each child to its parent's left or right based on the flag.
    /// 3. The root is the only node whose value is not in the children set.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of descriptions
    /// - Space: O(n) for the node map and children set
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> =
            HashMap::with_capacity(descriptions.len() * 2);
        let mut children = HashSet::with_capacity(descriptions.len());

        for d in &descriptions {
            let (parent_val, child_val, is_left) = (d[0], d[1], d[2]);

            nodes
                .entry(parent_val)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent_val))));
            nodes
                .entry(child_val)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_val))));

            let child_node = Rc::clone(nodes.get(&child_val).unwrap());
            let parent_node = nodes.get(&parent_val).unwrap();

            match is_left {
                1 => parent_node.borrow_mut().left = Some(child_node),
                _ => parent_node.borrow_mut().right = Some(child_node),
            }

            children.insert(child_val);
        }

        nodes
            .iter()
            .find(|(key, _)| !children.contains(key))
            .map(|(_, node)| Rc::clone(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_tree_structure(
        root: &Option<Rc<RefCell<TreeNode>>>,
        expected_val: i32,
        expected_left: Option<i32>,
        expected_right: Option<i32>,
    ) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node_borrow = node.borrow();
                if node_borrow.val != expected_val {
                    return false;
                }

                let left_ok = match (expected_left, &node_borrow.left) {
                    (None, None) => true,
                    (Some(val), Some(left)) => left.borrow().val == val,
                    _ => false,
                };

                let right_ok = match (expected_right, &node_borrow.right) {
                    (None, None) => true,
                    (Some(val), Some(right)) => right.borrow().val == val,
                    _ => false,
                };

                left_ok && right_ok
            }
        }
    }

    #[test]
    fn test_example_1() {
        // Descriptions: [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
        // Tree:
        //       50
        //      /  \
        //    20    80
        //   /  \   /
        //  15  17 19
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        let root = Solution::create_binary_tree(descriptions);

        // Verify root is 50
        assert!(verify_tree_structure(&root, 50, Some(20), Some(80)));

        // Verify structure
        if let Some(node) = root {
            let node_borrow = node.borrow();
            // Check left subtree (20)
            assert!(verify_tree_structure(
                &node_borrow.left,
                20,
                Some(15),
                Some(17)
            ));
            // Check right subtree (80)
            assert!(verify_tree_structure(
                &node_borrow.right,
                80,
                Some(19),
                None
            ));
        }
    }

    #[test]
    fn test_example_2() {
        // Descriptions: [[1,2,1],[2,3,0],[3,4,1]]
        // Tree:
        //     1
        //    /
        //   2
        //    \
        //     3
        //    /
        //   4
        let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
        let root = Solution::create_binary_tree(descriptions);

        // Verify root is 1
        assert!(verify_tree_structure(&root, 1, Some(2), None));

        // Verify the chain structure
        if let Some(node) = root {
            let left = &node.borrow().left;
            assert!(verify_tree_structure(left, 2, None, Some(3)));

            if let Some(left_node) = left {
                let right = &left_node.borrow().right;
                assert!(verify_tree_structure(right, 3, Some(4), None));
            }
        }
    }

    #[test]
    fn test_single_edge() {
        // Descriptions: [[10,5,1]]
        // Tree:
        //   10
        //  /
        // 5
        let descriptions = vec![vec![10, 5, 1]];
        let root = Solution::create_binary_tree(descriptions);

        assert!(verify_tree_structure(&root, 10, Some(5), None));
    }

    #[test]
    fn test_right_only_tree() {
        // Descriptions: [[1,2,0],[2,3,0],[3,4,0]]
        // Tree:
        //     1
        //      \
        //       2
        //        \
        //         3
        //          \
        //           4
        let descriptions = vec![vec![1, 2, 0], vec![2, 3, 0], vec![3, 4, 0]];
        let root = Solution::create_binary_tree(descriptions);

        assert!(verify_tree_structure(&root, 1, None, Some(2)));

        if let Some(node) = root {
            let right = &node.borrow().right;
            assert!(verify_tree_structure(right, 2, None, Some(3)));

            if let Some(right_node) = right {
                let right_right = &right_node.borrow().right;
                assert!(verify_tree_structure(right_right, 3, None, Some(4)));
            }
        }
    }

    #[test]
    fn test_complete_binary_tree() {
        // Descriptions: [[1,2,1],[1,3,0],[2,4,1],[2,5,0],[3,6,1],[3,7,0]]
        // Tree:
        //       1
        //      / \
        //     2   3
        //    / \ / \
        //   4  5 6  7
        let descriptions = vec![
            vec![1, 2, 1],
            vec![1, 3, 0],
            vec![2, 4, 1],
            vec![2, 5, 0],
            vec![3, 6, 1],
            vec![3, 7, 0],
        ];
        let root = Solution::create_binary_tree(descriptions);

        assert!(verify_tree_structure(&root, 1, Some(2), Some(3)));

        if let Some(node) = root {
            let node_borrow = node.borrow();
            assert!(verify_tree_structure(
                &node_borrow.left,
                2,
                Some(4),
                Some(5)
            ));
            assert!(verify_tree_structure(
                &node_borrow.right,
                3,
                Some(6),
                Some(7)
            ));
        }
    }
}
