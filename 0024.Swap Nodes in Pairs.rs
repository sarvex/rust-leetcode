// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     /// Creates a new `ListNode` with the given value and no next node.
//     pub fn new(val: i32) -> Self {
//         Self { val, next: None }
//     }
// }

impl Solution {
    /// Swaps every two adjacent nodes in a linked list.
    ///
    /// # Intuition
    /// For each pair of nodes, swap their positions by adjusting pointers.
    /// Recursively process the rest of the list after each swap.
    ///
    /// # Approach
    /// Use recursion to swap pairs from the end of the list backward:
    /// 1. Base case: if fewer than two nodes remain, return as-is
    /// 2. Take the first two nodes and recursively swap the remainder
    /// 3. Point the first node to the result of the recursive call
    /// 4. Point the second node to the first node
    /// 5. Return the second node as the new head of this pair
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(n) due to recursion stack
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut first) => match first.next.take() {
                None => Some(first),
                Some(mut second) => {
                    first.next = Self::swap_pairs(second.next.take());
                    second.next = Some(first);
                    Some(second)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        vec.iter()
            .rev()
            .fold(None, |next, &val| {
                let mut node = Box::new(ListNode::new(val));
                node.next = next;
                Some(node)
            })
    }

    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        std::iter::successors(list.as_ref(), |node| node.next.as_ref())
            .map(|node| node.val)
            .collect()
    }

    #[test]
    fn test_swap_pairs_even_length() {
        let head = to_list(&[1, 2, 3, 4]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn test_swap_pairs_odd_length() {
        let head = to_list(&[1, 2, 3]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![2, 1, 3]);
    }

    #[test]
    fn test_swap_pairs_single_element() {
        let head = to_list(&[1]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![1]);
    }

    #[test]
    fn test_swap_pairs_empty() {
        let head = to_list(&[]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), Vec::<i32>::new());
    }

    #[test]
    fn test_swap_pairs_two_elements() {
        let head = to_list(&[1, 2]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![2, 1]);
    }
}
