impl Solution {
    /// Greedy with a min-heap on suffix capacity.
    ///
    /// # Intuition
    /// Scanning from right to left, let `c(i)` be the number of '1's in the suffix `[i..]`.
    /// In any valid final arrangement, at most `c(i)` positions can be chosen inside that
    /// suffix (only those '1's can end there). We want the maximum sum under these suffix
    /// capacity constraints.
    ///
    /// # Approach
    /// Maintain a min-heap of chosen values for the current suffix. When we see a '1', the
    /// capacity increases by one, so we must include one more value—push `nums[i]`. When we
    /// see a '0', the capacity stays the same; include `nums[i]` only if it improves the sum
    /// (replace the current minimum). This is the standard greedy for deadline/suffix limits.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let s = s.as_bytes();
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for i in (0..nums.len()).rev() {
            if s[i] == b'1' {
                heap.push(Reverse(nums[i]));
            } else if let Some(&Reverse(min_val)) = heap.peek() {
                if nums[i] > min_val {
                    heap.pop();
                    heap.push(Reverse(nums[i]));
                }
            }
        }

        heap.into_iter().map(|Reverse(v)| v as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::maximum_score(vec![2, 1, 5, 2, 3], "01010".into()),
            7
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::maximum_score(vec![4, 7, 2, 9], "0000".into()), 0);
    }

    #[test]
    fn test_single_one() {
        assert_eq!(Solution::maximum_score(vec![10, 1, 1], "001".into()), 10);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::maximum_score(vec![3, 1, 2], "111".into()), 6);
    }

    #[test]
    fn test_ones_at_start() {
        assert_eq!(
            Solution::maximum_score(vec![5, 4, 3, 2, 1], "11000".into()),
            9
        );
    }

    #[test]
    fn test_order_preserved() {
        // '1's at positions 0,1,3 claim values 1, 8, 8 (best available at each '1')
        assert_eq!(
            Solution::maximum_score(vec![1, 8, 8, 4, 6, 2], "110100".into()),
            17
        );
    }

    #[test]
    fn test_swap_to_left() {
        // '1's at positions 1,2 can claim any value from positions 0-1 and 0-2
        // Best: first '1' takes 100, second '1' takes 50
        assert_eq!(
            Solution::maximum_score(vec![50, 100, 1, 2], "0110".into()),
            150
        );
    }

    #[test]
    fn test_greedy_needs_suffix_view() {
        // Left-to-right max-heap fails here; right-to-left suffix greedy succeeds.
        assert_eq!(
            Solution::maximum_score(vec![1, 1, 1, 1, 90, 100, 1], "0000011".into()),
            190
        );
    }
}
