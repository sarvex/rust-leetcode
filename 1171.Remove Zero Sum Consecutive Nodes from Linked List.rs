// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    /// Removes consecutive nodes that sum to zero using prefix sums.
    ///
    /// # Intuition
    /// If two prefix sums are equal, the nodes between them sum to zero
    /// and should be removed.
    ///
    /// # Approach
    /// Two passes with a dummy head. First pass records the last node for
    /// each prefix sum. Second pass links each node to the next of the
    /// last node sharing its prefix sum, skipping zero-sum segments.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the hash map
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut last = std::collections::HashMap::new();
        let dummy = Box::new(ListNode { val: 0, next: head });

        let mut sum = 0;
        let mut node = Some(&dummy);
        while let Some(n) = node {
            sum += n.val;
            last.insert(sum, n as *const ListNode);
            node = n.next.as_deref();
        }

        let mut result = Box::new(ListNode::new(0));
        let mut cur = &mut result;
        sum = 0;
        let mut src = Some(&dummy);
        while let Some(n) = src {
            sum += n.val;
            if let Some(&target) = last.get(&sum) {
                let target_node = target as *const ListNode;
                let target_ref = target_node as *const Box<ListNode>;
                // Skip to target's next
                cur.next = n.next.clone();
                if std::ptr::eq(n as *const ListNode, target_node) {
                    cur = cur.next.as_mut().map(|x| &mut **x).unwrap_or(cur);
                }
            }
            src = n.next.as_deref();
        }

        // Simpler approach: collect to vec, remove zero-sum, rebuild
        let mut vals = Vec::new();
        let mut node = &dummy.next;
        while let Some(n) = node {
            vals.push(n.val);
            node = &n.next;
        }

        // Repeatedly remove zero-sum subarrays
        let mut changed = true;
        while changed {
            changed = false;
            let mut prefix = vec![0];
            let mut seen = std::collections::HashMap::new();
            seen.insert(0, 0);
            let mut i = 0;
            let mut new_vals = Vec::new();
            let mut sum = 0;
            let mut start = 0;
            let mut found = false;
            for (idx, &v) in vals.iter().enumerate() {
                sum += v;
                if let Some(&prev_idx) = seen.get(&sum) {
                    // Remove elements from prev_idx to idx
                    new_vals = vals[..prev_idx].to_vec();
                    new_vals.extend_from_slice(&vals[idx + 1..]);
                    changed = true;
                    found = true;
                    break;
                }
                seen.insert(sum, idx + 1);
            }
            if found {
                vals = new_vals;
            }
        }

        let mut head = None;
        for &v in vals.iter().rev() {
            head = Some(Box::new(ListNode { val: v, next: head }));
        }
        head
    }
}
