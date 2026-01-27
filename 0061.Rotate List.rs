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
    /// Length-based split and reattach for linked list rotation.
    ///
    /// # Intuition
    /// Rotating right by k places is equivalent to moving the last k nodes
    /// to the front. After computing the list length, reduce k modulo n to
    /// handle large rotations, then split at position `n - k` and reattach.
    ///
    /// # Approach
    /// Count the list length. Compute effective rotation `k % n`. Traverse
    /// to the `(n - k - 1)`th node, detach the tail segment, and append
    /// the original head to the end of the detached segment.
    ///
    /// # Complexity
    /// - Time: O(n) — two passes at most
    /// - Space: O(1) — pointer manipulation only
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }

        let length = {
            let mut count = 0;
            let mut cursor = &head;
            while let Some(node) = cursor {
                count += 1;
                cursor = &node.next;
            }
            count
        };

        let k = k as usize % length;
        if k == 0 {
            return head;
        }

        let mut current = &mut head;
        for _ in 0..length - k - 1 {
            current = &mut current.as_mut().unwrap().next;
        }

        let mut new_head = current.as_mut().unwrap().next.take();
        let mut tail = &mut new_head;
        while tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = head;

        new_head
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
    fn rotate_by_two() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(
            from_list(Solution::rotate_right(head, 2)),
            vec![4, 5, 1, 2, 3]
        );
    }

    #[test]
    fn rotate_by_four() {
        let head = to_list(&[0, 1, 2]);
        assert_eq!(from_list(Solution::rotate_right(head, 4)), vec![2, 0, 1]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(from_list(Solution::rotate_right(None, 1)), vec![]);
    }

    #[test]
    fn rotate_by_length() {
        let head = to_list(&[1, 2, 3]);
        assert_eq!(from_list(Solution::rotate_right(head, 3)), vec![1, 2, 3]);
    }
}
