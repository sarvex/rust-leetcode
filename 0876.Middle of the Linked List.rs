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
    /// Finds the middle node using the slow/fast pointer technique.
    ///
    /// # Intuition
    /// When the fast pointer reaches the end, the slow pointer is at the middle.
    ///
    /// # Approach
    /// Move slow one step and fast two steps per iteration. When fast reaches
    /// the end, return slow.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
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
    fn test_middle_node_example1() {
        // Input: [1,2,3,4,5]
        // Expected: [3,4,5] (middle node is 3)
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![3, 4, 5]);
    }

    #[test]
    fn test_middle_node_example2() {
        // Input: [1,2,3,4,5,6]
        // Expected: [4,5,6] (second middle node is 4)
        let head = vec_to_list(vec![1, 2, 3, 4, 5, 6]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![4, 5, 6]);
    }

    #[test]
    fn test_middle_node_single() {
        // Input: [1]
        // Expected: [1]
        let head = vec_to_list(vec![1]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![1]);
    }

    #[test]
    fn test_middle_node_two_elements() {
        // Input: [1,2]
        // Expected: [2] (second middle node)
        let head = vec_to_list(vec![1, 2]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![2]);
    }

    #[test]
    fn test_middle_node_three_elements() {
        // Input: [1,2,3]
        // Expected: [2,3] (middle node is 2)
        let head = vec_to_list(vec![1, 2, 3]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![2, 3]);
    }

    #[test]
    fn test_middle_node_four_elements() {
        // Input: [1,2,3,4]
        // Expected: [3,4] (second middle node is 3)
        let head = vec_to_list(vec![1, 2, 3, 4]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![3, 4]);
    }

    #[test]
    fn test_middle_node_large_list() {
        // Input: [1,2,3,4,5,6,7,8,9]
        // Expected: [5,6,7,8,9] (middle node is 5)
        let head = vec_to_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let middle = Solution::middle_node(head);
        assert_eq!(list_to_vec(middle), vec![5, 6, 7, 8, 9]);
    }
}
