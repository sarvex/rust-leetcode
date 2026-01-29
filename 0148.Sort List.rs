#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a linked list from a vector
    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in vec.into_iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    /// Helper function to convert a linked list to a vector
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_sort_list_example1() {
        // Input: [4,2,1,3]
        // Expected: [1,2,3,4]
        let head = vec_to_list(vec![4, 2, 1, 3]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sort_list_example2() {
        // Input: [-1,5,3,4,0]
        // Expected: [-1,0,3,4,5]
        let head = vec_to_list(vec![-1, 5, 3, 4, 0]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_empty() {
        // Input: []
        // Expected: []
        let head = None;
        let sorted = Solution::sort_list(head);
        assert_eq!(sorted, None);
    }

    #[test]
    fn test_sort_list_single_node() {
        // Input: [1]
        // Expected: [1]
        let head = vec_to_list(vec![1]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![1]);
    }

    #[test]
    fn test_sort_list_two_nodes() {
        // Input: [2,1]
        // Expected: [1,2]
        let head = vec_to_list(vec![2, 1]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![1, 2]);
    }

    #[test]
    fn test_sort_list_duplicates() {
        // Input: [3,1,2,1,3,2]
        // Expected: [1,1,2,2,3,3]
        let head = vec_to_list(vec![3, 1, 2, 1, 3, 2]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_sort_list_already_sorted() {
        // Input: [1,2,3,4,5]
        // Expected: [1,2,3,4,5]
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_reverse_sorted() {
        // Input: [5,4,3,2,1]
        // Expected: [1,2,3,4,5]
        let head = vec_to_list(vec![5, 4, 3, 2, 1]);
        let sorted = Solution::sort_list(head);
        assert_eq!(list_to_vec(sorted), vec![1, 2, 3, 4, 5]);
    }
}
