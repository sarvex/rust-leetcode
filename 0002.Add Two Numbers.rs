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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut carry = 0;
        
        let (mut p1, mut p2) = (l1, l2);
        
        // Process both lists and handle carry
        while p1.is_some() || p2.is_some() || carry > 0 {
            // Calculate sum of current digits and carry
            let mut sum = carry;
            
            // Add value from first list if available
            if let Some(node) = p1 {
                sum += node.val;
                p1 = node.next;
            }
            
            // Add value from second list if available
            if let Some(node) = p2 {
                sum += node.val;
                p2 = node.next;
            }
            
            // Update carry for next calculation
            carry = sum / 10;
            
            // Create new node with current digit
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }
        
        // Return the result list (skip the dummy head)
        dummy_head.next
    }
}
