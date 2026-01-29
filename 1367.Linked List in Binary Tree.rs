use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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

pub struct Solution;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
        if vals.is_empty() {
            return None;
        }
        
        let mut head = Box::new(ListNode::new(vals[0]));
        let mut current = &mut head;
        
        for &val in &vals[1..] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        
        Some(head)
    }

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        
        let mut i = 1;
        
        while !queue.is_empty() && i < vals.len() {
            if let Some(node) = queue.pop_front() {
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
        }
        
        Some(root)
    }

    #[test]
    fn test_example_1() {
        // List: [4,2,8]
        // Tree: [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
        //         1
        //        / \
        //       4   4
        //        \   \
        //         2   2
        //        /   /
        //       1   6
        //            \
        //             8
        // Path exists: 4->2->8
        let head = build_list(&[4, 2, 8]);
        let root = build_tree(&[
            Some(1), Some(4), Some(4), None, Some(2), Some(2), None,
            Some(1), None, Some(6), Some(8), None, None, None, None,
            Some(1), Some(3)
        ]);
        assert_eq!(Solution::is_sub_path(head, root), true);
    }

    #[test]
    fn test_example_2() {
        // List: [1,4,2,6]
        // Tree: [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
        // Path exists: 1->4->2->6
        let head = build_list(&[1, 4, 2, 6]);
        let root = build_tree(&[
            Some(1), Some(4), Some(4), None, Some(2), Some(2), None,
            Some(1), None, Some(6), Some(8), None, None, None, None,
            Some(1), Some(3)
        ]);
        assert_eq!(Solution::is_sub_path(head, root), true);
    }

    #[test]
    fn test_no_path() {
        // List: [1,4,2,6,8]
        // Tree: [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
        // No path exists for the full list
        let head = build_list(&[1, 4, 2, 6, 8]);
        let root = build_tree(&[
            Some(1), Some(4), Some(4), None, Some(2), Some(2), None,
            Some(1), None, Some(6), Some(8), None, None, None, None,
            Some(1), Some(3)
        ]);
        assert_eq!(Solution::is_sub_path(head, root), false);
    }

    #[test]
    fn test_single_node_match() {
        // List: [1]
        // Tree: [1]
        let head = build_list(&[1]);
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::is_sub_path(head, root), true);
    }

    #[test]
    fn test_single_node_no_match() {
        // List: [2]
        // Tree: [1]
        let head = build_list(&[2]);
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::is_sub_path(head, root), false);
    }

    #[test]
    fn test_empty_list() {
        // Empty list should always return true
        let head = build_list(&[]);
        let root = build_tree(&[Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::is_sub_path(head, root), true);
    }
}
