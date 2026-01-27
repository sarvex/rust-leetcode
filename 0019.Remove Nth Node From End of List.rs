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
//     ListNode { next: None, val }
//   }
// }

impl Solution {
    /// Two-pass approach to remove the nth node from the end of a linked list.
    ///
    /// # Intuition
    /// Knowing the total length lets us compute the 0-based index of the node
    /// to remove. A dummy head simplifies edge cases where the head itself
    /// is removed.
    ///
    /// # Approach
    /// First pass: count the total number of nodes. Compute the target index
    /// as `length - n`. Second pass: traverse to the node just before the
    /// target and bypass it by relinking.
    ///
    /// # Complexity
    /// - Time: O(L) — two passes through the list
    /// - Space: O(1) — pointer manipulation only
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        let mut length = 0;
        let mut cursor = &dummy.next;
        while let Some(node) = cursor {
            length += 1;
            cursor = &node.next;
        }

        let target = length - n as usize;
        let mut current = &mut dummy;
        for _ in 0..target {
            current = current.next.as_mut().unwrap();
        }
        let next_next = current.next.as_mut().unwrap().next.take();
        current.next = next_next;

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        vec.iter().rev().fold(None, |next, &val| {
            let mut node = Box::new(ListNode::new(val));
            node.next = next;
            Some(node)
        })
    }

    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        std::iter::successors(list.as_ref(), |n| n.next.as_ref())
            .map(|n| n.val)
            .collect()
    }

    #[test]
    fn remove_from_middle() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(
            from_list(Solution::remove_nth_from_end(head, 2)),
            vec![1, 2, 3, 5]
        );
    }

    #[test]
    fn remove_only_element() {
        let head = to_list(&[1]);
        assert_eq!(from_list(Solution::remove_nth_from_end(head, 1)), vec![]);
    }

    #[test]
    fn remove_head() {
        let head = to_list(&[1, 2]);
        assert_eq!(from_list(Solution::remove_nth_from_end(head, 2)), vec![2]);
    }

    #[test]
    fn remove_tail() {
        let head = to_list(&[1, 2]);
        assert_eq!(from_list(Solution::remove_nth_from_end(head, 1)), vec![1]);
    }
}
