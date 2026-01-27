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
    /// Two-list partitioning around a pivot value for linked list.
    ///
    /// # Intuition
    /// Nodes with values less than `x` belong before nodes with values
    /// greater than or equal to `x`. Collecting them into two separate
    /// lists and concatenating preserves relative order.
    ///
    /// # Approach
    /// Maintain two dummy-headed lists: `less` and `greater_eq`. Iterate
    /// through the original list, appending each node to the appropriate
    /// list. Terminate the `greater_eq` list and link it after the `less` list.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — pointer manipulation only
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less = ListNode::new(0);
        let mut greater_eq = ListNode::new(0);
        let mut less_tail = &mut less;
        let mut greater_tail = &mut greater_eq;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();
            if node.val < x {
                less_tail.next = Some(node);
                less_tail = less_tail.next.as_mut().unwrap();
            } else {
                greater_tail.next = Some(node);
                greater_tail = greater_tail.next.as_mut().unwrap();
            }
        }

        greater_tail.next = None;
        less_tail.next = greater_eq.next;
        less.next
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
    fn standard_case() {
        let head = to_list(&[1, 4, 3, 2, 5, 2]);
        assert_eq!(
            from_list(Solution::partition(head, 3)),
            vec![1, 2, 2, 4, 3, 5]
        );
    }

    #[test]
    fn all_less() {
        let head = to_list(&[1, 2]);
        assert_eq!(from_list(Solution::partition(head, 3)), vec![1, 2]);
    }

    #[test]
    fn all_greater() {
        let head = to_list(&[3, 4]);
        assert_eq!(from_list(Solution::partition(head, 2)), vec![3, 4]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(from_list(Solution::partition(None, 1)), vec![]);
    }
}
