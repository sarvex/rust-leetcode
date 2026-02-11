use std::cmp::{max, min};

const MAXV: usize = 100_000;

struct SegTree {
    n: usize,
    mn: Vec<i32>,
    mx: Vec<i32>,
    lz: Vec<i32>,
}

impl SegTree {
    #[inline(always)]
    fn new(n: usize) -> Self {
        let size = 4 * n + 5;
        Self {
            n,
            mn: vec![0; size],
            mx: vec![0; size],
            lz: vec![0; size],
        }
    }

    fn build(&mut self, arr: &[i32]) {
        self.build_rec(1, 0, self.n - 1, arr);
    }

    fn build_rec(&mut self, idx: usize, left: usize, right: usize, arr: &[i32]) {
        if left == right {
            let value = arr[left];
            self.mn[idx] = value;
            self.mx[idx] = value;
            return;
        }
        let mid = (left + right) >> 1;
        let left_child = idx << 1;
        let right_child = left_child | 1;
        self.build_rec(left_child, left, mid, arr);
        self.build_rec(right_child, mid + 1, right, arr);
        self.mn[idx] = min(self.mn[left_child], self.mn[right_child]);
        self.mx[idx] = max(self.mx[left_child], self.mx[right_child]);
    }

    #[inline(always)]
    fn apply(&mut self, idx: usize, delta: i32) {
        self.mn[idx] += delta;
        self.mx[idx] += delta;
        self.lz[idx] += delta;
    }

    #[inline(always)]
    fn push(&mut self, idx: usize) {
        let delta = self.lz[idx];
        if delta != 0 {
            let left_child = idx << 1;
            let right_child = left_child | 1;
            self.apply(left_child, delta);
            self.apply(right_child, delta);
            self.lz[idx] = 0;
        }
    }

    #[inline(always)]
    fn pull(&mut self, idx: usize) {
        let left_child = idx << 1;
        let right_child = left_child | 1;
        self.mn[idx] = min(self.mn[left_child], self.mn[right_child]);
        self.mx[idx] = max(self.mx[left_child], self.mx[right_child]);
    }

    #[inline(always)]
    fn add(&mut self, left: usize, right: usize, delta: i32) {
        if left > right {
            return;
        }
        self.add_rec(1, 0, self.n - 1, left, right, delta);
    }

    fn add_rec(
        &mut self,
        idx: usize,
        tree_left: usize,
        tree_right: usize,
        query_left: usize,
        query_right: usize,
        delta: i32,
    ) {
        if query_left <= tree_left && tree_right <= query_right {
            self.apply(idx, delta);
            return;
        }
        self.push(idx);
        let tree_mid = (tree_left + tree_right) >> 1;

        if query_left <= tree_mid {
            self.add_rec(
                idx << 1,
                tree_left,
                tree_mid,
                query_left,
                query_right.min(tree_mid),
                delta,
            );
        }
        if query_right > tree_mid {
            self.add_rec(
                idx << 1 | 1,
                tree_mid + 1,
                tree_right,
                query_left.max(tree_mid + 1),
                query_right,
                delta,
            );
        }
        self.pull(idx);
    }

    #[inline(always)]
    fn rightmost_zero_from(&mut self, query_left: usize) -> Option<usize> {
        self.find_rec(1, 0, self.n - 1, query_left)
    }

    fn find_rec(
        &mut self,
        idx: usize,
        tree_left: usize,
        tree_right: usize,
        query_left: usize,
    ) -> Option<usize> {
        if tree_right < query_left {
            return None;
        }
        if self.mn[idx] > 0 || self.mx[idx] < 0 {
            return None;
        }
        if tree_left == tree_right {
            return Some(tree_left);
        }

        self.push(idx);
        let tree_mid = (tree_left + tree_right) >> 1;

        if let Some(result) = self.find_rec(idx << 1 | 1, tree_mid + 1, tree_right, query_left) {
            return Some(result);
        }
        self.find_rec(idx << 1, tree_left, tree_mid, query_left)
    }
}

impl Solution {
    /// Longest balanced subarray via lazy segment tree.
    ///
    /// # Intuition
    /// For fixed left boundary `l`, let `diff[r]` be:
    /// `#distinct_odd(nums[l..=r]) - #distinct_even(nums[l..=r])`.
    /// Then `[l, r]` is balanced iff `diff[r] == 0`.
    ///
    /// Sliding `l` to `l + 1` only changes a contiguous suffix of `r` values until the next
    /// occurrence of `nums[l]`, which is a range add on `diff`.
    ///
    /// # Approach
    /// 1. Build `diff[r]` for `l = 0`.
    /// 2. Segment tree supports range add and rightmost index with value `0`.
    /// 3. Iterate left boundary `l`:
    ///    - query rightmost `r >= l` where `diff[r] == 0`
    ///    - apply one range update reflecting removal of `nums[l]`
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let n_u32 = n as u32;

        let mut next_occ: Vec<u32> = vec![n_u32; n];
        let mut last: Vec<u32> = vec![n_u32; MAXV + 1];
        for i in (0..n).rev() {
            let value = nums[i] as usize;
            next_occ[i] = last[value];
            last[value] = i as u32;
        }

        let mut seen: Vec<u8> = vec![0; MAXV + 1];
        let mut diff: Vec<i32> = vec![0; n];
        let mut balance = 0_i32;
        let mut odd_count = 0_i32;
        let mut even_count = 0_i32;

        for i in 0..n {
            let value = nums[i] as usize;
            if seen[value] == 0 {
                seen[value] = 1;
                if (value & 1) == 1 {
                    balance += 1;
                    odd_count += 1;
                } else {
                    balance -= 1;
                    even_count += 1;
                }
            }
            diff[i] = balance;
        }

        if odd_count == 0 || even_count == 0 {
            return 0;
        }
        if balance == 0 {
            return n as i32;
        }

        let mut seg_tree = SegTree::new(n);
        seg_tree.build(&diff);

        let mut best: usize = 0;
        for left in 0..n {
            if let Some(right) = seg_tree.rightmost_zero_from(left) {
                let length = right + 1 - left;
                if length > best {
                    best = length;
                }
            }

            let next = next_occ[left] as usize;
            if next > left + 1 {
                let delta = if (nums[left] & 1) != 0 { -1 } else { 1 };
                seg_tree.add(left + 1, next - 1, delta);
            }
        }

        best as i32
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
