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
    /// Adds two numbers represented as linked lists (most significant digit first).
    ///
    /// # Intuition
    /// Since the numbers are stored with the most significant digit first, we need to
    /// reverse both lists to process from least significant digit, perform digit-by-digit
    /// addition with carry, then reverse the result.
    ///
    /// # Approach
    /// 1. Reverse both input lists to get least significant digit first
    /// 2. Add corresponding digits with carry propagation
    /// 3. Handle remaining carry if any
    /// 4. Reverse the result list to restore MSB-first order
    ///
    /// # Complexity
    /// - Time: O(m + n) where m and n are lengths of the two lists
    /// - Space: O(max(m, n)) for the result list
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                head = next;
            }
            prev
        }

        let mut a = reverse(l1);
        let mut b = reverse(l2);
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while a.is_some() || b.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = a {
                sum += node.val;
                a = node.next;
            }
            if let Some(node) = b {
                sum += node.val;
                b = node.next;
            }
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        reverse(dummy.next)
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
    fn test_add_two_numbers_example1() {
        // Input: l1 = [7,2,4,3], l2 = [5,6,4]
        // Output: [7,8,0,7]
        // Explanation: 7243 + 564 = 7807
        let l1 = vec_to_list(vec![7, 2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![7, 8, 0, 7]);
    }

    #[test]
    fn test_add_two_numbers_example2() {
        // Input: l1 = [2,4,3], l2 = [5,6,4]
        // Output: [8,0,7]
        // Explanation: 243 + 564 = 807
        let l1 = vec_to_list(vec![2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![8, 0, 7]);
    }

    #[test]
    fn test_add_two_numbers_with_carry() {
        // Input: l1 = [9,9,9], l2 = [1]
        // Output: [1,0,0,0]
        // Explanation: 999 + 1 = 1000
        let l1 = vec_to_list(vec![9, 9, 9]);
        let l2 = vec_to_list(vec![1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_add_two_numbers_zeros() {
        // Input: l1 = [0], l2 = [0]
        // Output: [0]
        // Explanation: 0 + 0 = 0
        let l1 = vec_to_list(vec![0]);
        let l2 = vec_to_list(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        // Input: l1 = [1,0,0,0,0], l2 = [9,9]
        // Output: [1,0,0,9,9]
        // Explanation: 10000 + 99 = 10099
        let l1 = vec_to_list(vec![1, 0, 0, 0, 0]);
        let l2 = vec_to_list(vec![9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![1, 0, 0, 9, 9]);
    }
}
