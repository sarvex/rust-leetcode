// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    /// DFS matching linked list as a downward path in the binary tree.
    ///
    /// # Intuition
    /// For each tree node, attempt to match the linked list starting from that
    /// node downward. If the current node matches, continue matching the rest
    /// of the list in both subtrees. If not, try starting fresh from children.
    ///
    /// # Approach
    /// 1. `dfs`: check if list starting at `head` matches a downward path from `root`
    /// 2. `is_sub_path`: try `dfs` at every tree node via recursive traversal
    ///
    /// # Complexity
    /// - Time: O(n · m) where n = tree nodes, m = list length
    /// - Space: O(n + m) recursion depth
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(&head, &root)
    }

    fn check(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow();
                Self::matches(head, &Some(Rc::clone(&root.as_ref().unwrap())))
                    || Self::check(head, &node.left)
                    || Self::check(head, &node.right)
            }
        }
    }

    fn matches(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match head {
            None => true,
            Some(list_node) => match root {
                None => false,
                Some(tree_node) => {
                    let tree_node = tree_node.borrow();
                    list_node.val == tree_node.val
                        && (Self::matches(&list_node.next, &tree_node.left)
                            || Self::matches(&list_node.next, &tree_node.right))
                }
            },
        }
    }
}
