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
    /// Finds the middle node using the slow/fast pointer technique.
    ///
    /// # Intuition
    /// When the fast pointer reaches the end, the slow pointer is at the middle.
    ///
    /// # Approach
    /// Move slow one step and fast two steps per iteration. When fast reaches
    /// the end, return slow.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}
