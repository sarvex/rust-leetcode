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
        std::iter::successors(head.as_ref(), |node| node.next.as_ref())
            .fold(0, |acc, node| (acc << 1) | node.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    fn from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
        vals.iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
    }

    fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        std::iter::successors(head.as_ref(), |node| node.next.as_ref())
            .fold(0, |acc, node| (acc << 1) | node.val)
    }

    #[test]
    fn binary_101() {
        assert_eq!(get_decimal_value(from_vec(&[1, 0, 1])), 5);
    }

    #[test]
    fn single_zero() {
        assert_eq!(get_decimal_value(from_vec(&[0])), 0);
    }

    #[test]
    fn single_one() {
        assert_eq!(get_decimal_value(from_vec(&[1])), 1);
    }

    #[test]
    fn all_ones() {
        assert_eq!(get_decimal_value(from_vec(&[1, 1, 1, 1])), 15);
    }
}
