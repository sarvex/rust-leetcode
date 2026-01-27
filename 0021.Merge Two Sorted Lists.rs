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
    /// Recursive merge of two sorted linked lists.
    ///
    /// # Intuition
    /// At each step the smaller head becomes the next node in the merged list.
    /// Recursion naturally handles the remaining tails until one list is exhausted.
    ///
    /// # Approach
    /// Pattern match on both heads. When both are present, compare values and
    /// recurse on the chosen node's tail merged with the other list. Base cases
    /// return the non-empty list directly.
    ///
    /// # Complexity
    /// - Time: O(n + m) — each node processed exactly once
    /// - Space: O(n + m) — recursion stack depth
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, other) | (other, None) => other,
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
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
    fn both_non_empty() {
        let l1 = to_list(&[1, 2, 4]);
        let l2 = to_list(&[1, 3, 4]);
        assert_eq!(
            from_list(Solution::merge_two_lists(l1, l2)),
            vec![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    fn both_empty() {
        assert_eq!(from_list(Solution::merge_two_lists(None, None)), vec![]);
    }

    #[test]
    fn one_empty() {
        let l1 = to_list(&[0]);
        assert_eq!(from_list(Solution::merge_two_lists(None, l1)), vec![0]);
    }
}
