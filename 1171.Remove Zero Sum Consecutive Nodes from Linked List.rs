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

pub struct Solution;

impl Solution {
    /// Removes consecutive nodes that sum to zero using prefix sums.
    ///
    /// # Intuition
    /// If two positions have the same prefix sum, the nodes between them sum to zero.
    /// We can use this property to identify and remove zero-sum segments.
    ///
    /// # Approach
    /// 1. Convert linked list to vector for easier manipulation
    /// 2. Repeatedly find and remove zero-sum subarrays using prefix sums
    /// 3. Use a hashmap to track the first occurrence of each prefix sum
    /// 4. When we see a prefix sum again, remove elements between occurrences
    /// 5. Rebuild the linked list from the remaining values
    ///
    /// # Complexity
    /// - Time: O(n²) in worst case due to repeated removal passes
    /// - Space: O(n) for storing values and prefix sum map
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Collect values from linked list
        let mut vals = Vec::new();
        let mut node = &head;
        while let Some(n) = node {
            vals.push(n.val);
            node = &n.next;
        }

        // Repeatedly remove zero-sum subarrays
        let mut changed = true;
        while changed {
            changed = false;
            let mut seen = std::collections::HashMap::new();
            seen.insert(0, 0);
            let mut sum = 0;
            
            for (idx, &v) in vals.iter().enumerate() {
                sum += v;
                if let Some(&prev_idx) = seen.get(&sum) {
                    // Found zero-sum subarray from prev_idx to idx
                    let mut new_vals = vals[..prev_idx].to_vec();
                    new_vals.extend_from_slice(&vals[idx + 1..]);
                    vals = new_vals;
                    changed = true;
                    break;
                }
                seen.insert(sum, idx + 1);
            }
        }

        // Rebuild linked list from remaining values
        let mut head = None;
        for &v in vals.iter().rev() {
            head = Some(Box::new(ListNode { val: v, next: head }));
        }
        head
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
    fn test_remove_zero_sum_example1() {
        // Input: head = [1,2,-3,3,1]
        // Output: [3,1]
        // Explanation: 1+2-3=0, so remove those nodes
        let head = vec_to_list(vec![1, 2, -3, 3, 1]);
        let result = Solution::remove_zero_sum_sublists(head);
        assert_eq!(list_to_vec(result), vec![3, 1]);
    }

    #[test]
    fn test_remove_zero_sum_example2() {
        // Input: head = [1,2,3,-3,4]
        // Output: [1,2,4]
        // Explanation: 3-3=0, so remove those nodes
        let head = vec_to_list(vec![1, 2, 3, -3, 4]);
        let result = Solution::remove_zero_sum_sublists(head);
        assert_eq!(list_to_vec(result), vec![1, 2, 4]);
    }

    #[test]
    fn test_remove_zero_sum_example3() {
        // Input: head = [1,2,3,-3,-2]
        // Output: [1]
        // Explanation: 2+3-3-2=0, so remove those nodes
        let head = vec_to_list(vec![1, 2, 3, -3, -2]);
        let result = Solution::remove_zero_sum_sublists(head);
        assert_eq!(list_to_vec(result), vec![1]);
    }

    #[test]
    fn test_remove_zero_sum_all_zeros() {
        // Input: head = [0,0,0]
        // Output: []
        // Explanation: All nodes sum to zero
        let head = vec_to_list(vec![0, 0, 0]);
        let result = Solution::remove_zero_sum_sublists(head);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_remove_zero_sum_no_removal() {
        // Input: head = [1,2,3,4]
        // Output: [1,2,3,4]
        // Explanation: No zero-sum subarrays
        let head = vec_to_list(vec![1, 2, 3, 4]);
        let result = Solution::remove_zero_sum_sublists(head);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_zero_sum_entire_list() {
        // Input: head = [1,-1]
        // Output: []
        // Explanation: Entire list sums to zero
        let head = vec_to_list(vec![1, -1]);
        let result = Solution::remove_zero_sum_sublists(head);
        assert_eq!(list_to_vec(result), vec![]);
    }
}
