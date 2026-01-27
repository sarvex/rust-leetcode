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
    /// Merge linked list nodes between consecutive zeros into their sums.
    ///
    /// # Intuition
    /// Walk through the list, accumulating values between zero-valued nodes
    /// and creating a new node for each accumulated segment.
    ///
    /// # Approach
    /// 1. Skip the leading zero node.
    /// 2. Accumulate values until the next zero node is found.
    /// 3. Append the accumulated sum as a new node and reset.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(k) where k is the number of segments
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut sum = 0;
        let mut cur = head.unwrap().next;

        while let Some(mut node) = cur {
            match node.val {
                0 => {
                    tail.next = Some(Box::new(ListNode::new(sum)));
                    tail = tail.next.as_mut().unwrap();
                    sum = 0;
                }
                v => sum += v,
            }
            cur = node.next.take();
        }

        dummy.next
    }
}
