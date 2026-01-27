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
    /// Sorts a linked list in O(n log n) time using merge sort.
    ///
    /// # Intuition
    /// Merge sort naturally fits linked lists: splitting at the midpoint
    /// and merging two sorted halves requires no random access.
    ///
    /// # Approach
    /// 1. Count the list length, split at the midpoint.
    /// 2. Recursively sort both halves.
    /// 3. Merge the two sorted halves by comparing head elements.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(log n) recursion stack
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (None, Some(node)) | (Some(node), None) => Some(node),
                (Some(mut n1), Some(mut n2)) => {
                    if n1.val < n2.val {
                        n1.next = merge(n1.next.take(), Some(n2));
                        Some(n1)
                    } else {
                        n2.next = merge(Some(n1), n2.next.take());
                        Some(n2)
                    }
                }
                _ => None,
            }
        }

        fn sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_ref().unwrap().next.is_none() {
                return head;
            }
            let mut head = head;
            let mut length = 0;
            let mut cur = &head;
            while cur.is_some() {
                length += 1;
                cur = &cur.as_ref().unwrap().next;
            }
            let mut cur = &mut head;
            for _ in 0..length / 2 - 1 {
                cur = &mut cur.as_mut().unwrap().next;
            }
            let right = cur.as_mut().unwrap().next.take();
            merge(sort(head), sort(right))
        }

        sort(head)
    }
}
