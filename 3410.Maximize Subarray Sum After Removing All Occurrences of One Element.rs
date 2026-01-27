use std::collections::HashMap;

trait SegmentTreeHelper<T> {
    fn combine(&self, left: T, right: T) -> T;
}

struct SegmentTree<T: Copy, H: SegmentTreeHelper<T>> {
    data: Vec<T>,
    len: usize,
    helper: H,
}

impl<T: Copy, H: SegmentTreeHelper<T>> SegmentTree<T, H> {
    fn new(nums: &[T], helper: H) -> Self {
        let mut result = Self {
            data: vec![nums[0]; nums.len() * 4],
            len: nums.len(),
            helper,
        };
        result.build(0, 0, nums.len() - 1, nums);
        result
    }

    fn build(&mut self, node: usize, start: usize, end: usize, nums: &[T]) {
        if start == end {
            self.data[node] = nums[start];
        } else {
            let mid = (start + end) / 2;
            self.build(node * 2 + 1, start, mid, nums);
            self.build(node * 2 + 2, mid + 1, end, nums);
            self.data[node] = self
                .helper
                .combine(self.data[node * 2 + 1], self.data[node * 2 + 2]);
        }
    }

    fn update(&mut self, index: usize, val: T) {
        self.update_recursive(0, 0, self.len - 1, index, val);
    }

    fn update_recursive(&mut self, node: usize, start: usize, end: usize, index: usize, val: T) {
        if start == end {
            self.data[node] = val;
        } else {
            let mid = (start + end) / 2;
            if index <= mid {
                self.update_recursive(node * 2 + 1, start, mid, index, val);
            } else {
                self.update_recursive(node * 2 + 2, mid + 1, end, index, val);
            }
            self.data[node] = self
                .helper
                .combine(self.data[node * 2 + 1], self.data[node * 2 + 2]);
        }
    }

    fn query(&self, left: usize, right: usize) -> T {
        self.query_recursive(0, 0, self.len - 1, left, right)
    }

    fn query_recursive(
        &self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
    ) -> T {
        if start == left && end == right {
            self.data[node]
        } else {
            let mid = (start + end) / 2;
            if right <= mid {
                self.query_recursive(node * 2 + 1, start, mid, left, right)
            } else if left > mid {
                self.query_recursive(node * 2 + 2, mid + 1, end, left, right)
            } else {
                let l = self.query_recursive(node * 2 + 1, start, mid, left, mid);
                let r = self.query_recursive(node * 2 + 2, mid + 1, end, mid + 1, right);
                self.helper.combine(l, r)
            }
        }
    }
}

struct SubarrayHelper;

impl SegmentTreeHelper<(i64, i64, i64, i64)> for SubarrayHelper {
    fn combine(
        &self,
        left: (i64, i64, i64, i64),
        right: (i64, i64, i64, i64),
    ) -> (i64, i64, i64, i64) {
        (
            left.0.max(right.0).max(right.1 + left.2),
            left.1.max(left.3 + right.1),
            right.2.max(right.3 + left.2),
            left.3 + right.3,
        )
    }
}

impl Solution {
    /// Maximizes subarray sum after optionally removing all occurrences of one element.
    ///
    /// # Intuition
    /// A segment tree storing (max_subarray, prefix_max, suffix_max, total)
    /// supports efficient "what if we remove value v?" queries by temporarily
    /// zeroing out all positions of v.
    ///
    /// # Approach
    /// 1. Build segment tree where negative values can be "skipped" (max clamped to 0)
    ///    but their total carries the actual value for aggregation.
    /// 2. Query the base case (no removal).
    /// 3. For each unique negative value, zero out its positions, query, restore.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn max_subarray_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let max_val = *nums.iter().max().unwrap() as i64;
        if max_val < 0 {
            return max_val;
        }

        let nodes: Vec<_> = nums
            .iter()
            .map(|&x| x as i64)
            .map(|x| (x.max(0), x.max(0), x.max(0), x))
            .collect();

        let mut tree = SegmentTree::new(&nodes, SubarrayHelper);

        let mut neg_positions: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            if x < 0 {
                neg_positions.entry(x).or_default().push(i);
            }
        }

        let mut result = tree.query(0, n - 1).0;

        for (&val, positions) in &neg_positions {
            for &i in positions {
                tree.update(i, (0, 0, 0, 0));
            }

            result = result.max(tree.query(0, n - 1).0);

            for &i in positions {
                tree.update(i, (0, 0, 0, val as i64));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_negative_bridges_segments() {
        assert_eq!(Solution::max_subarray_sum(vec![-3, 2, -2, -1, 3, -2, 3]), 7);
    }

    #[test]
    fn all_positive_uses_entire_array() {
        assert_eq!(Solution::max_subarray_sum(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn all_negative_returns_max_element() {
        assert_eq!(Solution::max_subarray_sum(vec![-5, -3, -2]), -2);
    }

    #[test]
    fn single_positive_element() {
        assert_eq!(Solution::max_subarray_sum(vec![5]), 5);
    }

    #[test]
    fn single_negative_element() {
        assert_eq!(Solution::max_subarray_sum(vec![-5]), -5);
    }

    #[test]
    fn removing_repeated_negative_unifies_segments() {
        assert_eq!(Solution::max_subarray_sum(vec![5, -1, 5, -1, 5]), 15);
    }

    #[test]
    fn removing_large_negative_bridges_far_segments() {
        assert_eq!(Solution::max_subarray_sum(vec![10, -5, 10, -5, 10]), 30);
    }

    #[test]
    fn mixed_negatives_choose_best_removal() {
        assert_eq!(Solution::max_subarray_sum(vec![1, -2, 3, -4, 5]), 8);
    }

    #[test]
    fn two_elements_sum() {
        assert_eq!(Solution::max_subarray_sum(vec![1, 2]), 3);
    }

    #[test]
    fn all_same_negative_returns_single() {
        assert_eq!(Solution::max_subarray_sum(vec![-2, -2, -2]), -2);
    }
}
