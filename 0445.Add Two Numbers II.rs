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
    /// Adds two numbers represented as linked lists (most significant digit first).
    ///
    /// # Intuition
    /// Reverse both lists to process from least significant digit, perform
    /// digit-by-digit addition with carry, then reverse the result.
    ///
    /// # Approach
    /// 1. Reverse both input lists.
    /// 2. Add corresponding digits with carry propagation.
    /// 3. Reverse the result list to restore MSB-first order.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(max(m, n))
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                head = next;
            }
            prev
        }

        let mut a = reverse(l1);
        let mut b = reverse(l2);
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while a.is_some() || b.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = a {
                sum += node.val;
                a = node.next;
            }
            if let Some(node) = b {
                sum += node.val;
                b = node.next;
            }
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        reverse(dummy.next)
    }
}
