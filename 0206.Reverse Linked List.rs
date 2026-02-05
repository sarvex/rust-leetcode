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

impl Solution {
    /// Reverses a singly linked list iteratively using pointer manipulation.
    ///
    /// # Intuition
    /// Walk through the list, reversing each node's next pointer to point
    /// to the previous node instead of the next.
    ///
    /// # Approach
    /// 1. Maintain a `prev` pointer (initially None).
    /// 2. For each node, save its next, point it to prev, then advance.
    /// 3. Return prev as the new head.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
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
    fn test_reverse_list_example1() {
        // Input: [1,2,3,4,5]
        // Expected: [5,4,3,2,1]
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(list_to_vec(reversed), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_example2() {
        // Input: [1,2]
        // Expected: [2,1]
        let head = vec_to_list(vec![1, 2]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(list_to_vec(reversed), vec![2, 1]);
    }

    #[test]
    fn test_reverse_list_empty() {
        // Input: []
        // Expected: []
        let head = None;
        let reversed = Solution::reverse_list(head);
        assert_eq!(reversed, None);
    }

    #[test]
    fn test_reverse_list_single_node() {
        // Input: [1]
        // Expected: [1]
        let head = vec_to_list(vec![1]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(list_to_vec(reversed), vec![1]);
    }

    #[test]
    fn test_reverse_list_three_nodes() {
        // Input: [1,2,3]
        // Expected: [3,2,1]
        let head = vec_to_list(vec![1, 2, 3]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(list_to_vec(reversed), vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_negative_values() {
        // Input: [-1,-2,-3]
        // Expected: [-3,-2,-1]
        let head = vec_to_list(vec![-1, -2, -3]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(list_to_vec(reversed), vec![-3, -2, -1]);
    }

    #[test]
    fn test_reverse_list_large_list() {
        // Input: [1,2,3,4,5,6,7,8,9,10]
        // Expected: [10,9,8,7,6,5,4,3,2,1]
        let head = vec_to_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(list_to_vec(reversed), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
