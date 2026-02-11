use std::collections::HashMap;

impl Solution {
    /// Longest contiguous subarray where #distinct evens equals #distinct odds.
    ///
    /// # Intuition
    /// Track `diff[i] = #distinct_evens - #distinct_odds` for every potential start index `i`
    /// of subarrays ending at the current position. When value `v` appears at position `j`,
    /// only subarrays starting after the previous occurrence of `v` gain a new distinct element,
    /// translating to a range addition on a segment tree.
    ///
    /// # Approach
    /// Maintain a segment tree with lazy propagation over start indices. For each position `j`:
    /// 1. Let `prev` be the last occurrence of `nums[j]` (or before the array start).
    /// 2. Add +1 (even) or -1 (odd) to range `[prev+1, j]`.
    /// 3. Query for the leftmost index in `[0, j]` where value == 0 (balanced).
    /// 4. Update the answer with `j - leftmost + 1`.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut tree = SegTree::new(n);
        let mut last_occ: HashMap<i32, usize> = HashMap::with_capacity(n);
        let mut result = 0;

        for (j, &v) in nums.iter().enumerate() {
            let left = last_occ.get(&v).map_or(0, |&p| p + 1);
            let delta = if v % 2 == 0 { 1 } else { -1 };
            tree.update(1, 0, n - 1, left, j, delta);
            last_occ.insert(v, j);

            if let Some(i) = tree.query(1, 0, n - 1, 0, j) {
                result = result.max((j - i + 1) as i32);
            }
        }

        result
    }
}

struct SegTree {
    min_val: Vec<i32>,
    max_val: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let size = 4 * n;
        Self {
            min_val: vec![0; size],
            max_val: vec![0; size],
            lazy: vec![0; size],
        }
    }

    fn push_down(&mut self, node: usize) {
        if self.lazy[node] != 0 {
            let val = self.lazy[node];
            let left = 2 * node;
            let right = 2 * node + 1;
            self.min_val[left] += val;
            self.max_val[left] += val;
            self.lazy[left] += val;
            self.min_val[right] += val;
            self.max_val[right] += val;
            self.lazy[right] += val;
            self.lazy[node] = 0;
        }
    }

    fn pull_up(&mut self, node: usize) {
        let left = 2 * node;
        let right = 2 * node + 1;
        self.min_val[node] = self.min_val[left].min(self.min_val[right]);
        self.max_val[node] = self.max_val[left].max(self.max_val[right]);
    }

    fn update(&mut self, node: usize, lo: usize, hi: usize, l: usize, r: usize, delta: i32) {
        if l > hi || r < lo {
            return;
        }
        if l <= lo && hi <= r {
            self.min_val[node] += delta;
            self.max_val[node] += delta;
            self.lazy[node] += delta;
            return;
        }
        self.push_down(node);
        let mid = lo + (hi - lo) / 2;
        self.update(2 * node, lo, mid, l, r, delta);
        self.update(2 * node + 1, mid + 1, hi, l, r, delta);
        self.pull_up(node);
    }

    fn query(&mut self, node: usize, lo: usize, hi: usize, ql: usize, qr: usize) -> Option<usize> {
        if lo > qr || hi < ql {
            return None;
        }
        if self.min_val[node] > 0 || self.max_val[node] < 0 {
            return None;
        }
        if lo == hi {
            return Some(lo);
        }
        self.push_down(node);
        let mid = lo + (hi - lo) / 2;
        self.query(2 * node, lo, mid, ql, qr)
            .or_else(|| self.query(2 * node + 1, mid + 1, hi, ql, qr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::longest_balanced(vec![1]), 0);
        assert_eq!(Solution::longest_balanced(vec![2]), 0);
    }

    #[test]
    fn test_two_elements_balanced() {
        assert_eq!(Solution::longest_balanced(vec![1, 2]), 2);
        assert_eq!(Solution::longest_balanced(vec![2, 1]), 2);
    }

    #[test]
    fn test_all_same_parity() {
        assert_eq!(Solution::longest_balanced(vec![1, 3, 5, 7]), 0);
        assert_eq!(Solution::longest_balanced(vec![2, 4, 6, 8]), 0);
    }

    #[test]
    fn test_with_duplicates() {
        assert_eq!(Solution::longest_balanced(vec![1, 1, 2, 2]), 4);
        assert_eq!(Solution::longest_balanced(vec![1, 2, 1, 2]), 4);
    }
}
