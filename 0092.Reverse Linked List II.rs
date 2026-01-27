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
    /// In-place reversal of a sublist between positions left and right.
    ///
    /// # Intuition
    /// Traverse to the node before the reversal start, then iteratively
    /// insert each subsequent node at the front of the reversed section.
    /// This avoids separate detach-reverse-reattach passes.
    ///
    /// # Approach
    /// Use a dummy head. Advance to the node before position `left`.
    /// For each of the `right - left + 1` nodes in the segment, detach
    /// the next node and insert it at the front of the reversed section.
    /// Then advance to the end of the reversed segment and reattach the
    /// remaining tail.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — pointer manipulation only
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut pre = &mut dummy;

        for _ in 1..left {
            pre = &mut pre.as_mut().unwrap().next;
        }

        let mut cur = pre.as_mut().unwrap().next.take();
        for _ in 0..right - left + 1 {
            let next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = pre.as_mut().unwrap().next.take();
            pre.as_mut().unwrap().next = cur.take();
            cur = next;
        }

        for _ in 0..right - left + 1 {
            pre = &mut pre.as_mut().unwrap().next;
        }
        pre.as_mut().unwrap().next = cur;

        dummy.unwrap().next
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
    fn reverse_middle() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(
            from_list(Solution::reverse_between(head, 2, 4)),
            vec![1, 4, 3, 2, 5]
        );
    }

    #[test]
    fn reverse_single() {
        let head = to_list(&[5]);
        assert_eq!(from_list(Solution::reverse_between(head, 1, 1)), vec![5]);
    }

    #[test]
    fn reverse_entire() {
        let head = to_list(&[1, 2, 3]);
        assert_eq!(
            from_list(Solution::reverse_between(head, 1, 3)),
            vec![3, 2, 1]
        );
    }
}
