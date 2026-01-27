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
    /// Iterative group reversal of linked list nodes in k-sized segments.
    ///
    /// # Intuition
    /// Reverse each group of k nodes independently, then link the reversed
    /// groups together. If fewer than k nodes remain, keep them in original
    /// order.
    ///
    /// # Approach
    /// Use a dummy head for uniform edge handling. For each group, advance
    /// a probe pointer k-1 steps to verify the group is complete. Detach
    /// the group, reverse it in-place, then attach the reversed segment.
    /// Advance the tail pointer to the end of the reversed group and repeat.
    ///
    /// # Complexity
    /// - Time: O(n) — each node reversed at most once
    /// - Space: O(1) — in-place pointer manipulation
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            let mut current = head;
            while let Some(mut node) = current {
                current = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
            prev
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut tail = &mut dummy;
        let mut remaining = tail.as_mut().unwrap().next.take();

        loop {
            let mut probe = &remaining;
            for _ in 0..k {
                match probe {
                    Some(node) => probe = &node.next,
                    None => {
                        tail.as_mut().unwrap().next = remaining;
                        return dummy.unwrap().next;
                    }
                }
            }

            let mut group_head = remaining;
            let mut next_group = &mut group_head;
            for _ in 0..k - 1 {
                next_group = &mut next_group.as_mut().unwrap().next;
            }
            let rest = next_group.as_mut().unwrap().next.take();

            tail.as_mut().unwrap().next = reverse(group_head);
            while tail.as_ref().unwrap().next.is_some() {
                tail = &mut tail.as_mut().unwrap().next;
            }
            remaining = rest;
        }
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
    fn reverse_by_two() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(
            from_list(Solution::reverse_k_group(head, 2)),
            vec![2, 1, 4, 3, 5]
        );
    }

    #[test]
    fn reverse_by_three() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(
            from_list(Solution::reverse_k_group(head, 3)),
            vec![3, 2, 1, 4, 5]
        );
    }

    #[test]
    fn k_equals_one() {
        let head = to_list(&[1, 2, 3]);
        assert_eq!(from_list(Solution::reverse_k_group(head, 1)), vec![1, 2, 3]);
    }

    #[test]
    fn k_equals_length() {
        let head = to_list(&[1, 2, 3]);
        assert_eq!(from_list(Solution::reverse_k_group(head, 3)), vec![3, 2, 1]);
    }
}
