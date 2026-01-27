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
    /// Finds the next greater value for each node using a monotonic stack.
    ///
    /// # Intuition
    /// Collect values into an array, then use a right-to-left monotonic
    /// stack to find the next greater element for each position.
    ///
    /// # Approach
    /// Traverse the list to collect values. Iterate in reverse, maintaining
    /// a decreasing stack. The stack top after popping smaller elements is
    /// the next greater value.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            vals.push(node.val);
            cur = &node.next;
        }

        let n = vals.len();
        let mut result = vec![0; n];
        let mut stack: Vec<i32> = Vec::new();

        for i in (0..n).rev() {
            while stack.last().is_some_and(|&top| top <= vals[i]) {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                result[i] = top;
            }
            stack.push(vals[i]);
        }

        result
    }
}
