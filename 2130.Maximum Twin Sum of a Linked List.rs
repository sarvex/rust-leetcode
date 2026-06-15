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
    /// Finds the maximum twin sum by collecting values and pairing from both ends.
    ///
    /// # Intuition
    /// In a linked list of even length n, node i and node n-1-i are twins.
    /// Collecting all values into a flat array gives O(1) indexed access,
    /// making the pairing loop straightforward and cache-friendly.
    ///
    /// # Approach
    /// 1. Traverse the list once, pushing each value into a `Vec`.
    /// 2. Walk indices 0..n/2, summing each element with its mirror at n-1-i.
    /// 3. Track and return the running maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut values = Vec::new();

        let mut cur = &head;
        while let Some(node) = cur {
            values.push(node.val);
            cur = &node.next;
        }

        let n = values.len();
        let mut res = 0;

        for i in 0..n / 2 {
            res = res.max(values[i] + values[n - 1 - i]);
        }

        res
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
    fn test_pair_sum_example1() {
        // Input: head = [5,4,2,1]
        // Output: 6
        // Explanation: Twins are (5,1)=6 and (4,2)=6. Maximum is 6.
        let head = vec_to_list(vec![5, 4, 2, 1]);
        assert_eq!(Solution::pair_sum(head), 6);
    }

    #[test]
    fn test_pair_sum_example2() {
        // Input: head = [4,2,2,3]
        // Output: 7
        // Explanation: Twins are (4,3)=7 and (2,2)=4. Maximum is 7.
        let head = vec_to_list(vec![4, 2, 2, 3]);
        assert_eq!(Solution::pair_sum(head), 7);
    }

    #[test]
    fn test_pair_sum_example3() {
        // Input: head = [1,100000]
        // Output: 100001
        // Explanation: Only one twin pair (1,100000)=100001
        let head = vec_to_list(vec![1, 100000]);
        assert_eq!(Solution::pair_sum(head), 100001);
    }

    #[test]
    fn test_pair_sum_all_same() {
        // Input: head = [5,5,5,5]
        // Output: 10
        // Explanation: All twin pairs sum to 10
        let head = vec_to_list(vec![5, 5, 5, 5]);
        assert_eq!(Solution::pair_sum(head), 10);
    }

    #[test]
    fn test_pair_sum_increasing() {
        // Input: head = [1,2,3,4,5,6]
        // Output: 7
        // Explanation: Twins are (1,6)=7, (2,5)=7, (3,4)=7. Maximum is 7.
        let head = vec_to_list(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(Solution::pair_sum(head), 7);
    }

    #[test]
    fn test_pair_sum_large_values() {
        // Input: head = [47,22,81,46,94,95,90,22,55,91,6,83,49,65,10,32,41,26,83,99,14,85,42,99]
        // Output: 182
        let head = vec_to_list(vec![
            47, 22, 81, 46, 94, 95, 90, 22, 55, 91, 6, 83, 49, 65, 10, 32, 41, 26, 83, 99, 14, 85,
            42, 99,
        ]);
        assert_eq!(Solution::pair_sum(head), 193);
    }
}
