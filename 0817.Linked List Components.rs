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

use std::collections::HashSet;


impl Solution {
    /// Counts connected components in a linked list subset.
    ///
    /// # Intuition
    /// A connected component is a maximal set of connected nodes. We need to count
    /// how many such groups exist where nodes are in the given subset and adjacent
    /// to each other in the linked list.
    ///
    /// # Approach
    /// 1. Store the subset in a HashSet for O(1) lookup
    /// 2. Traverse the list and track when we enter/exit components
    /// 3. A new component starts when we encounter a node in the subset after
    ///    being outside of it
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of the linked list
    /// - Space: O(m) where m is the size of the nums array
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut count = 0;
        let mut in_component = false;
        let mut cur = &head;
        
        while let Some(node) = cur {
            if set.contains(&node.val) {
                if !in_component {
                    in_component = true;
                    count += 1;
                }
            } else {
                in_component = false;
            }
            cur = &node.next;
        }
        count
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
    fn test_num_components_example1() {
        // Input: head = [0,1,2,3], nums = [0,1,3]
        // Output: 2
        // Explanation: 0 and 1 are connected, 3 is a separate component
        let head = vec_to_list(vec![0, 1, 2, 3]);
        let nums = vec![0, 1, 3];
        assert_eq!(Solution::num_components(head, nums), 2);
    }

    #[test]
    fn test_num_components_example2() {
        // Input: head = [0,1,2,3,4], nums = [0,3,1,4]
        // Output: 2
        // Explanation: 0 and 1 are connected, 3 and 4 are connected
        let head = vec_to_list(vec![0, 1, 2, 3, 4]);
        let nums = vec![0, 3, 1, 4];
        assert_eq!(Solution::num_components(head, nums), 2);
    }

    #[test]
    fn test_num_components_all_nodes() {
        // Input: head = [0,1,2,3], nums = [0,1,2,3]
        // Output: 1
        // Explanation: All nodes form one component
        let head = vec_to_list(vec![0, 1, 2, 3]);
        let nums = vec![0, 1, 2, 3];
        assert_eq!(Solution::num_components(head, nums), 1);
    }

    #[test]
    fn test_num_components_no_adjacent() {
        // Input: head = [0,1,2,3,4], nums = [0,2,4]
        // Output: 3
        // Explanation: Each node is a separate component
        let head = vec_to_list(vec![0, 1, 2, 3, 4]);
        let nums = vec![0, 2, 4];
        assert_eq!(Solution::num_components(head, nums), 3);
    }

    #[test]
    fn test_num_components_single_node() {
        // Input: head = [0], nums = [0]
        // Output: 1
        let head = vec_to_list(vec![0]);
        let nums = vec![0];
        assert_eq!(Solution::num_components(head, nums), 1);
    }
}
