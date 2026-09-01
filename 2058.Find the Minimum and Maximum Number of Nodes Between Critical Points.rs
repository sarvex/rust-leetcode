// Uncomment when needed for local testing:
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { val, next: None }
//     }
// }

impl Solution {
    /// Single-pass scan tracking critical-point positions.
    ///
    /// # Intuition
    /// A critical point is a node whose value is a strict local maximum or
    /// minimum relative to its immediate neighbours. To answer the query we
    /// only need three facts about the sequence of critical-point positions:
    /// the first position, the previous position, and the last position seen.
    /// The minimum distance is the smallest gap between two consecutive
    /// critical points, and the maximum distance is always between the first
    /// and last critical points.
    ///
    /// # Approach
    /// Walk the list keeping the previous value, the current index, and the
    /// value one step ahead so we can classify the current node. Whenever the
    /// current node is a strict local extremum, record its index:
    /// - Track `first` critical index (set once).
    /// - Track `prev` critical index to compute consecutive gaps for the
    ///   running minimum.
    /// - Track `last` critical index to compute the overall span.
    ///
    /// After the scan, if fewer than two critical points exist return
    /// `[-1, -1]`; otherwise return `[min_gap, last - first]`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut node = head.as_deref();
        let Some(first_node) = node else {
            return vec![-1, -1];
        };

        let mut prev_val = first_node.val;
        node = first_node.next.as_deref();

        let mut index = 1_i32;
        let mut first = -1_i32;
        let mut prev_critical = -1_i32;
        let mut last = -1_i32;
        let mut min_gap = i32::MAX;

        while let Some(cur_node) = node {
            let Some(next_node) = cur_node.next.as_deref() else {
                break;
            };

            let cur_val = cur_node.val;
            let next_val = next_node.val;
            let is_critical = (cur_val > prev_val && cur_val > next_val)
                || (cur_val < prev_val && cur_val < next_val);

            if is_critical {
                if first == -1 {
                    first = index;
                } else {
                    min_gap = min_gap.min(index - prev_critical);
                }
                prev_critical = index;
                last = index;
            }

            prev_val = cur_val;
            node = cur_node.next.as_deref();
            index += 1;
        }

        if first == -1 || first == last {
            vec![-1, -1]
        } else {
            vec![min_gap, last - first]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { val, next: None }
        }
    }

    struct Solution;

    impl Solution {
        fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
            let mut node = head.as_deref();
            if node.is_none() {
                return vec![-1, -1];
            }

            let first_node = node.unwrap();
            let mut prev_val = first_node.val;
            node = first_node.next.as_deref();

            let mut index = 1_i32;
            let mut first = -1_i32;
            let mut prev_critical = -1_i32;
            let mut last = -1_i32;
            let mut min_gap = i32::MAX;

            while let Some(cur_node) = node {
                let Some(next_node) = cur_node.next.as_deref() else {
                    break;
                };

                let cur_val = cur_node.val;
                let next_val = next_node.val;
                let is_critical = (cur_val > prev_val && cur_val > next_val)
                    || (cur_val < prev_val && cur_val < next_val);

                if is_critical {
                    if first == -1 {
                        first = index;
                    } else {
                        min_gap = min_gap.min(index - prev_critical);
                    }
                    prev_critical = index;
                    last = index;
                }

                prev_val = cur_val;
                node = cur_node.next.as_deref();
                index += 1;
            }

            if first == -1 || first == last {
                vec![-1, -1]
            } else {
                vec![min_gap, last - first]
            }
        }
    }

    fn to_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in vals.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_no_critical_points() {
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[3, 1])),
            vec![-1, -1]
        );
    }

    #[test]
    fn test_three_critical_points() {
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[5, 3, 1, 2, 5, 1, 2])),
            vec![1, 3]
        );
    }

    #[test]
    fn test_two_critical_points_equal_distances() {
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[1, 3, 2, 2, 3, 2, 2, 2, 7])),
            vec![3, 3]
        );
    }

    #[test]
    fn test_single_critical_point() {
        // Only one local maxima at index 1.
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[1, 5, 1])),
            vec![-1, -1]
        );
    }

    #[test]
    fn test_minimum_length_list() {
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[1, 1])),
            vec![-1, -1]
        );
    }

    #[test]
    fn test_plateau_no_strict_extrema() {
        // Equal neighbours are never strict extrema.
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[2, 2, 2, 2, 2])),
            vec![-1, -1]
        );
    }

    #[test]
    fn test_alternating_pattern() {
        // 1,3,1,3,1 -> critical at indices 1,2,3 -> min 1, max 2.
        assert_eq!(
            Solution::nodes_between_critical_points(to_list(&[1, 3, 1, 3, 1])),
            vec![1, 2]
        );
    }
}
