// Definition for singly-linked list.
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
    /// Finds the next greater value for each node using a monotonic stack.
    ///
    /// # Intuition
    /// For each node, we need to find the next node with a greater value. This is
    /// a classic next greater element problem that can be efficiently solved using
    /// a monotonic decreasing stack.
    ///
    /// # Approach
    /// 1. Collect all values from the linked list into a vector
    /// 2. Use a monotonic decreasing stack traversing from right to left
    /// 3. For each position, pop elements from stack that are not greater
    /// 4. The top of stack (if exists) is the next greater element
    /// 5. Push current element to maintain the monotonic property
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(n) for storing values and stack
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            vals.push(node.val);
            cur = &node.next;
        }

        let n = vals.len();
        let mut result = vec![0; n];
        let mut stack: Vec<i32> = Vec::new();

        for i in (0..n).rev() {
            while stack.last().is_some_and(|&top| top <= vals[i]) {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                result[i] = top;
            }
            stack.push(vals[i]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }

    #[test]
    fn test_next_larger_nodes_example1() {
        // Input: head = [2,1,5]
        // Output: [5,5,0]
        // Explanation: For node 2, next greater is 5. For node 1, next greater is 5. For node 5, no greater element.
        let head = vec_to_list(vec![2, 1, 5]);
        assert_eq!(Solution::next_larger_nodes(head), vec![5, 5, 0]);
    }

    #[test]
    fn test_next_larger_nodes_example2() {
        // Input: head = [2,7,4,3,5]
        // Output: [7,0,5,5,0]
        let head = vec_to_list(vec![2, 7, 4, 3, 5]);
        assert_eq!(Solution::next_larger_nodes(head), vec![7, 0, 5, 5, 0]);
    }

    #[test]
    fn test_next_larger_nodes_decreasing() {
        // Input: head = [5,4,3,2,1]
        // Output: [0,0,0,0,0]
        // Explanation: No next greater element for any node
        let head = vec_to_list(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::next_larger_nodes(head), vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_next_larger_nodes_increasing() {
        // Input: head = [1,2,3,4,5]
        // Output: [2,3,4,5,0]
        // Explanation: Each element's next greater is the next element
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::next_larger_nodes(head), vec![2, 3, 4, 5, 0]);
    }

    #[test]
    fn test_next_larger_nodes_single() {
        // Input: head = [1]
        // Output: [0]
        let head = vec_to_list(vec![1]);
        assert_eq!(Solution::next_larger_nodes(head), vec![0]);
    }

    #[test]
    fn test_next_larger_nodes_duplicates() {
        // Input: head = [1,7,5,1,9,2,5,1]
        // Output: [7,9,9,9,0,5,0,0]
        let head = vec_to_list(vec![1, 7, 5, 1, 9, 2, 5, 1]);
        assert_eq!(
            Solution::next_larger_nodes(head),
            vec![7, 9, 9, 9, 0, 5, 0, 0]
        );
    }
}
