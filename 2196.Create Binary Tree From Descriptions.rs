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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

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
        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut children = HashSet::new();

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
