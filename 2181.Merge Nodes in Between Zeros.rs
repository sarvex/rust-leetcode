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
    /// Merge linked list nodes between consecutive zeros into their sums.
    ///
    /// # Intuition
    /// The list starts and ends with zeros, with non-zero values in between.
    /// We need to sum the values between each pair of zeros and create a new
    /// node for each sum.
    ///
    /// # Approach
    /// 1. Skip the initial zero node
    /// 2. Accumulate values until we hit a zero
    /// 3. Create a new node with the accumulated sum
    /// 4. Reset the accumulator and continue
    /// 5. Return the new list without any zeros
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(k) where k is the number of segments between zeros
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut sum = 0;
        let mut cur = head.unwrap().next;

        while let Some(mut node) = cur {
            match node.val {
                0 => {
                    tail.next = Some(Box::new(ListNode::new(sum)));
                    tail = tail.next.as_mut().unwrap();
                    sum = 0;
                }
                v => sum += v,
            }
            cur = node.next.take();
        }

        dummy.next
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

    fn list_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = list {
            vec.push(node.val);
            list = node.next;
        }
        vec
    }

    #[test]
    fn test_merge_nodes_example1() {
        // Input: head = [0,3,1,0,4,5,2,0]
        // Output: [4,11]
        // Explanation: Sum between first zeros: 3+1=4, between second zeros: 4+5+2=11
        let head = vec_to_list(vec![0, 3, 1, 0, 4, 5, 2, 0]);
        let result = Solution::merge_nodes(head);
        assert_eq!(list_to_vec(result), vec![4, 11]);
    }

    #[test]
    fn test_merge_nodes_example2() {
        // Input: head = [0,1,0,3,0,2,2,0]
        // Output: [1,3,4]
        // Explanation: Three segments: 1, 3, and 2+2=4
        let head = vec_to_list(vec![0, 1, 0, 3, 0, 2, 2, 0]);
        let result = Solution::merge_nodes(head);
        assert_eq!(list_to_vec(result), vec![1, 3, 4]);
    }

    #[test]
    fn test_merge_nodes_single_segment() {
        // Input: head = [0,5,0]
        // Output: [5]
        // Explanation: Only one segment with value 5
        let head = vec_to_list(vec![0, 5, 0]);
        let result = Solution::merge_nodes(head);
        assert_eq!(list_to_vec(result), vec![5]);
    }

    #[test]
    fn test_merge_nodes_multiple_single_values() {
        // Input: head = [0,1,0,2,0,3,0]
        // Output: [1,2,3]
        // Explanation: Three segments each with single value
        let head = vec_to_list(vec![0, 1, 0, 2, 0, 3, 0]);
        let result = Solution::merge_nodes(head);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_nodes_large_sum() {
        // Input: head = [0,100,200,300,0,50,50,0]
        // Output: [600,100]
        // Explanation: First segment: 100+200+300=600, second: 50+50=100
        let head = vec_to_list(vec![0, 100, 200, 300, 0, 50, 50, 0]);
        let result = Solution::merge_nodes(head);
        assert_eq!(list_to_vec(result), vec![600, 100]);
    }

    #[test]
    fn test_merge_nodes_many_segments() {
        // Input: head = [0,1,2,0,3,4,0,5,6,0,7,8,0]
        // Output: [3,7,11,15]
        let head = vec_to_list(vec![0, 1, 2, 0, 3, 4, 0, 5, 6, 0, 7, 8, 0]);
        let result = Solution::merge_nodes(head);
        assert_eq!(list_to_vec(result), vec![3, 7, 11, 15]);
    }
}
