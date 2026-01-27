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
    /// Bit-shift accumulation traversing the linked list.
    ///
    /// # Intuition
    /// The linked list represents a binary number from MSB to LSB. Shifting
    /// the accumulator left by one and OR-ing each node's value builds the
    /// integer in a single traversal.
    ///
    /// # Approach
    /// 1. Traverse the list from head to tail
    /// 2. At each node, shift accumulator left by 1 and OR with node value
    /// 3. Return the final accumulator
    ///
    /// # Complexity
    /// - Time: O(n) single traversal
    /// - Space: O(1)
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut current = &head;
        while let Some(node) = current {
            result = (result << 1) | node.val;
            current = &node.next;
        }
        result
    }
}
