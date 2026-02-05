#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Creates a new `ListNode` with the given value and no next node.
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

impl Solution {
    /// Adds two numbers represented as linked lists in reverse order.
    ///
    /// # Intuition
    /// Simulate digit-by-digit addition with carry, as in grade school math.
    ///
    /// # Approach
    /// Traverse both lists, summing digits and carry. Append each digit to the result list. Continue until both lists and carry are processed.
    ///
    /// # Complexity
    /// - Time: O(max(n, m))
    /// - Space: O(max(n, m))
    ///
    /// # Panics
    /// Never panics. Always returns a valid list or None.
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = l1.as_ref().map_or(0, |n| n.val) + l2.as_ref().map_or(0, |n| n.val) + carry;
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = current;
            current = Some(node);
        }
        current
    }

    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vals = Vec::new();
        let mut node = list;
        while let Some(n) = node {
            vals.push(n.val);
            node = n.next;
        }
        vals
    }

    #[test]
    fn test_add_two_numbers_basic() {
        let l1 = to_list(&[2, 4, 3]);
        let l2 = to_list(&[5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_carry() {
        let l1 = to_list(&[9, 9, 9]);
        let l2 = to_list(&[1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_diff_lengths() {
        let l1 = to_list(&[2, 4]);
        let l2 = to_list(&[5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![7, 0, 5]);
    }

    #[test]
    fn test_add_two_numbers_both_none() {
        let result = Solution::add_two_numbers(None, None);
        assert_eq!(from_list(result), Vec::<i32>::new());
    }
}
