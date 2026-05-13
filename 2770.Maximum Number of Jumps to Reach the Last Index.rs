/// Iterative segment tree for range-max queries and point updates.
/// Nodes 1..2n store values; leaves are at indices n..2n.
struct SegTree {
    n: usize,
    t: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            t: vec![i32::MIN; 2 * n],
        }
    }

    /// Point update: set position p to max(current, v).
    fn update(&mut self, mut p: usize, v: i32) {
        p += self.n;
        self.t[p] = self.t[p].max(v);
        while p > 1 {
            p >>= 1;
            self.t[p] = self.t[2 * p].max(self.t[2 * p + 1]);
        }
    }

    /// Range max query over [l, r] inclusive.
    fn query(&self, mut l: usize, mut r: usize) -> i32 {
        let mut res = i32::MIN;
        l += self.n;
        r += self.n + 1;
        while l < r {
            if l & 1 == 1 {
                res = res.max(self.t[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = res.max(self.t[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}

impl Solution {
    /// Iterative segment tree with coordinate compression for O(n log n).
    ///
    /// # Intuition
    /// Map values to compressed coordinates, then use a segment tree to answer
    /// "what is the max dp value among all predecessors whose value is within
    /// [nums[i] - target, nums[i] + target]?" in O(log n) per index.
    ///
    /// # Approach
    /// 1. Coordinate compress: sort + dedup all values → indices 0..m
    /// 2. Build an iterative segment tree over m leaves (2m nodes, no recursion)
    /// 3. For each index i left-to-right:
    ///    - Binary search for the valid value range [nums[i]-target, nums[i]+target]
    ///    - Query tree for max dp in that range
    ///    - Update tree at nums[i]'s compressed position with dp[i]
    /// The iterative tree halves constant factors vs recursive: no call stack,
    /// 2n nodes instead of 4n, and sequential memory access patterns.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        sorted.dedup();
        let m = sorted.len();
        let pos = |v: i32| sorted.binary_search(&v).unwrap();

        let mut tree = SegTree::new(m);
        tree.update(pos(nums[0]), 0);

        let t = target as i64;
        let mut last = if n == 1 { 0 } else { -1 };

        for i in 1..n {
            let lo = nums[i] as i64 - t;
            let hi = nums[i] as i64 + t;
            let l = sorted.partition_point(|&x| (x as i64) < lo);
            let r = sorted.partition_point(|&x| (x as i64) <= hi);

            let dp_i = if l < r {
                let mx = tree.query(l, r - 1);
                if mx >= 0 {
                    mx + 1
                } else {
                    -1
                }
            } else {
                -1
            };

            if i == n - 1 {
                last = dp_i;
            }
            if dp_i >= 0 {
                tree.update(pos(nums[i]), dp_i);
            }
        }
        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
    }

    #[test]
    fn test_two_elements_reachable() {
        assert_eq!(Solution::maximum_jumps(vec![0, 1], 1), 1);
    }

    #[test]
    fn test_two_elements_unreachable() {
        assert_eq!(Solution::maximum_jumps(vec![0, 5], 1), -1);
    }

    #[test]
    fn test_all_same_values_zero_target() {
        assert_eq!(Solution::maximum_jumps(vec![3, 3, 3, 3], 0), 3);
    }

    #[test]
    fn test_large_target() {
        assert_eq!(
            Solution::maximum_jumps(vec![1, 1000000000, -1000000000, 0], 2000000000),
            3
        );
    }

    #[test]
    fn test_extreme_values() {
        assert_eq!(
            Solution::maximum_jumps(vec![i32::MIN, i32::MAX], 2000000000),
            -1
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_jumps(vec![42], 100), 0);
    }

    #[test]
    fn test_descending_sequence() {
        assert_eq!(Solution::maximum_jumps(vec![5, 4, 3, 2, 1], 1), 4);
    }
}
