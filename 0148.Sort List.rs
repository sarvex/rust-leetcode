/// Singly linked list node used by `sort_list`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    /// Node value.
    pub val: i32,
    /// Next node in the list.
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}


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
        fn split_at_mid(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut slow = head.as_mut();
            let mut fast = head.as_ref();

            while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
                let fast_next = fast_node.next.as_ref();
                let fast_next_next = fast_next.and_then(|node| node.next.as_ref());
                if fast_next_next.is_none() {
                    break;
                }
                slow = slow_node.next.as_mut();
                fast = fast_next_next;
            }

            slow.and_then(|node| node.next.take())
        }

        fn merge(
            mut left: Option<Box<ListNode>>,
            mut right: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut dummy = Box::new(ListNode::new(0));
            let mut tail = &mut dummy;

            while let (Some(left_node), Some(right_node)) = (left.as_ref(), right.as_ref()) {
                let take_left = left_node.val <= right_node.val;
                let node = if take_left {
                    let mut node = left.take().unwrap();
                    left = node.next.take();
                    node
                } else {
                    let mut node = right.take().unwrap();
                    right = node.next.take();
                    node
                };
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }

            tail.next = left.or(right);
            dummy.next
        }

        fn sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head.as_ref() {
                None => return None,
                Some(node) if node.next.is_none() => return head,
                _ => {}
            }

            let right = split_at_mid(&mut head);
            let left_sorted = sort(head);
            let right_sorted = sort(right);
            merge(left_sorted, right_sorted)
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
