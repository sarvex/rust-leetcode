// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    /// Deletes the middle node at index ⌊n/2⌋ using a two-pass, O(1)-space approach.
    ///
    /// # Intuition
    /// We need to reach the node just before the middle so we can splice it out.
    /// A single length count gives us the exact target index; we then walk the
    /// list a second time stopping at `mid - 1` and rewire `next` in place.
    ///
    /// # Approach
    /// 1. Count `n` by traversing once.
    /// 2. If `n == 1` the whole list is removed; return `None`.
    /// 3. Compute `mid = n / 2`. Walk `mid - 1` steps from the head to land on
    ///    the predecessor node.
    /// 4. Set `predecessor.next = predecessor.next.next`, dropping the middle node.
    ///
    /// # Complexity
    /// - Time: O(n) — two passes, second terminates at ⌊n/2⌋
    /// - Space: O(1) — no auxiliary allocation
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Count nodes.
        let mut n = 0usize;
        let mut cur = &head;
        while let Some(node) = cur {
            n += 1;
            cur = &node.next;
        }

        // Single node: the only node is the middle.
        if n <= 1 {
            return None;
        }

        let mid = n / 2;

        // Walk to the predecessor (index mid - 1).
        let mut predecessor = head.as_deref_mut().unwrap();
        for _ in 0..mid - 1 {
            predecessor = predecessor.next.as_deref_mut().unwrap();
        }

        // Splice out the middle node.
        let middle = predecessor.next.take().unwrap();
        predecessor.next = middle.next;

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

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_example1_seven_nodes() {
        // [1,3,4,7,1,2,6] → remove index 3 (value 7) → [1,3,4,1,2,6]
        let head = vec_to_list(vec![1, 3, 4, 7, 1, 2, 6]);
        assert_eq!(
            list_to_vec(Solution::delete_middle(head)),
            vec![1, 3, 4, 1, 2, 6]
        );
    }

    #[test]
    fn test_example2_four_nodes() {
        // [1,2,3,4] → remove index 2 (value 3) → [1,2,4]
        let head = vec_to_list(vec![1, 2, 3, 4]);
        assert_eq!(list_to_vec(Solution::delete_middle(head)), vec![1, 2, 4]);
    }

    #[test]
    fn test_example3_two_nodes() {
        // [2,1] → remove index 1 (value 1) → [2]
        let head = vec_to_list(vec![2, 1]);
        assert_eq!(list_to_vec(Solution::delete_middle(head)), vec![2]);
    }

    #[test]
    fn test_single_node() {
        // n=1: the only node is the middle; result is empty.
        let head = vec_to_list(vec![5]);
        assert_eq!(list_to_vec(Solution::delete_middle(head)), vec![]);
    }

    #[test]
    fn test_three_nodes() {
        // [1,2,3] → remove index 1 (value 2) → [1,3]
        let head = vec_to_list(vec![1, 2, 3]);
        assert_eq!(list_to_vec(Solution::delete_middle(head)), vec![1, 3]);
    }

    #[test]
    fn test_five_nodes() {
        // [1,2,3,4,5] → remove index 2 (value 3) → [1,2,4,5]
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(list_to_vec(Solution::delete_middle(head)), vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_six_nodes() {
        // [1,2,3,4,5,6] → remove index 3 (value 4) → [1,2,3,5,6]
        let head = vec_to_list(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            list_to_vec(Solution::delete_middle(head)),
            vec![1, 2, 3, 5, 6]
        );
    }
}
