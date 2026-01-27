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
    /// Finds the maximum twin sum by collecting values and pairing from ends.
    ///
    /// # Intuition
    /// Twin nodes are at positions `i` and `n - 1 - i`. By collecting all
    /// values into a vector, we can pair elements from both ends and find
    /// the maximum sum.
    ///
    /// # Approach
    /// 1. Traverse the linked list, collecting values into a vector.
    /// 2. Pair the first element with the last, second with second-to-last, etc.
    /// 3. Return the maximum sum among all pairs.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut values = Vec::new();
        let mut current = &head;
        while let Some(node) = current {
            values.push(node.val);
            current = &node.next;
        }

        let n = values.len();
        (0..n / 2)
            .map(|i| values[i] + values[n - 1 - i])
            .max()
            .unwrap_or(0)
    }
}
