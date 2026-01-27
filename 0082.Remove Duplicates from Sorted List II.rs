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
    /// Dummy-head linear scan removing all nodes with duplicate values.
    ///
    /// # Intuition
    /// Nodes whose value appeared more than once must be entirely removed.
    /// Tracking the previous value lets us detect both the first and
    /// subsequent duplicates in a single pass.
    ///
    /// # Approach
    /// Use a dummy head with a sentinel value. Iterate through nodes,
    /// tracking the previous value. If a node's value equals the previous
    /// or the next node's value, it is a duplicate — skip it. Otherwise,
    /// attach it to the result list.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — pointer manipulation only
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(i32::MIN));
        let mut tail = &mut dummy;
        let mut current = head;
        let mut prev_val = i32::MIN;

        while let Some(mut node) = current {
            current = node.next.take();
            let is_dup =
                node.val == prev_val || current.as_ref().map_or(false, |next| next.val == node.val);
            prev_val = node.val;

            if !is_dup {
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
    fn remove_duplicates() {
        let head = to_list(&[1, 2, 3, 3, 4, 4, 5]);
        assert_eq!(from_list(Solution::delete_duplicates(head)), vec![1, 2, 5]);
    }

    #[test]
    fn all_duplicates() {
        let head = to_list(&[1, 1, 1, 2, 3]);
        assert_eq!(from_list(Solution::delete_duplicates(head)), vec![2, 3]);
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
