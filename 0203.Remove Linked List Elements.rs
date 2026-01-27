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
    /// Removes all nodes with a given value using a dummy head technique.
    ///
    /// # Intuition
    /// A dummy head simplifies removal of the first node. Walk the list,
    /// skipping nodes that match the target value.
    ///
    /// # Approach
    /// 1. Create a dummy node pointing to the head.
    /// 2. For each next node, either skip it (if value matches) or advance.
    /// 3. Return dummy's next as the new head.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        while let Some(mut node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next.take();
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
