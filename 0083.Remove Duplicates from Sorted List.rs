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
    /// Dummy-head deduplication keeping one copy of each value in a sorted list.
    ///
    /// # Intuition
    /// Since the list is sorted, duplicates are adjacent. Comparing each
    /// node's value to the last appended value skips duplicates naturally.
    ///
    /// # Approach
    /// Use a dummy head with a sentinel value (`i32::MAX`). For each node,
    /// attach it only if its value differs from the tail's value. This
    /// preserves the first occurrence and drops subsequent duplicates.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — pointer manipulation only
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(i32::MAX));
        let mut tail = &mut dummy;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();
            if node.val != tail.val {
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }

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
    fn with_duplicates() {
        let head = to_list(&[1, 1, 2]);
        assert_eq!(from_list(Solution::delete_duplicates(head)), vec![1, 2]);
    }

    #[test]
    fn multiple_groups() {
        let head = to_list(&[1, 1, 2, 3, 3]);
        assert_eq!(from_list(Solution::delete_duplicates(head)), vec![1, 2, 3]);
    }

    #[test]
    fn no_duplicates() {
        let head = to_list(&[1, 2, 3]);
        assert_eq!(from_list(Solution::delete_duplicates(head)), vec![1, 2, 3]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(from_list(Solution::delete_duplicates(None)), vec![]);
    }
}
