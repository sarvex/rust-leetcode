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
impl Solution {
    /// Reverses a singly linked list iteratively using pointer manipulation.
    ///
    /// # Intuition
    /// Walk through the list, reversing each node's next pointer to point
    /// to the previous node instead of the next.
    ///
    /// # Approach
    /// 1. Maintain a `prev` pointer (initially None).
    /// 2. For each node, save its next, point it to prev, then advance.
    /// 3. Return prev as the new head.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
