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
    /// Removes all nodes with a given value using a dummy head technique.
    ///
    /// # Intuition
    /// A dummy head simplifies removal of the first node. Walk the list,
    /// skipping nodes that match the target value.
    ///
    /// # Approach
    /// 1. Create a dummy node pointing to the head.
    /// 2. For each next node, either skip it (if value matches) or advance.
    /// 3. Return dummy's next as the new head.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        while let Some(mut node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next.take();
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy.next
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
    fn test_remove_elements_example1() {
        // Input: [1,2,6,3,4,5,6], val = 6
        // Expected: [1,2,3,4,5]
        let head = vec_to_list(vec![1, 2, 6, 3, 4, 5, 6]);
        let result = Solution::remove_elements(head, 6);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_elements_empty() {
        // Input: [], val = 1
        // Expected: []
        let head = None;
        let result = Solution::remove_elements(head, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_remove_elements_all_same() {
        // Input: [7,7,7,7], val = 7
        // Expected: []
        let head = vec_to_list(vec![7, 7, 7, 7]);
        let result = Solution::remove_elements(head, 7);
        assert_eq!(result, None);
    }

    #[test]
    fn test_remove_elements_none_match() {
        // Input: [1,2,3,4], val = 5
        // Expected: [1,2,3,4]
        let head = vec_to_list(vec![1, 2, 3, 4]);
        let result = Solution::remove_elements(head, 5);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_elements_single_node_match() {
        // Input: [1], val = 1
        // Expected: []
        let head = vec_to_list(vec![1]);
        let result = Solution::remove_elements(head, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_remove_elements_single_node_no_match() {
        // Input: [1], val = 2
        // Expected: [1]
        let head = vec_to_list(vec![1]);
        let result = Solution::remove_elements(head, 2);
        assert_eq!(list_to_vec(result), vec![1]);
    }

    #[test]
    fn test_remove_elements_head_removal() {
        // Input: [1,1,2,3], val = 1
        // Expected: [2,3]
        let head = vec_to_list(vec![1, 1, 2, 3]);
        let result = Solution::remove_elements(head, 1);
        assert_eq!(list_to_vec(result), vec![2, 3]);
    }

    #[test]
    fn test_remove_elements_tail_removal() {
        // Input: [1,2,3,3], val = 3
        // Expected: [1,2]
        let head = vec_to_list(vec![1, 2, 3, 3]);
        let result = Solution::remove_elements(head, 3);
        assert_eq!(list_to_vec(result), vec![1, 2]);
    }
}
