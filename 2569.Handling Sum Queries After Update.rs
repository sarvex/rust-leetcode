impl Solution {
    /// Track ones in `nums1` with a lazy segment tree and aggregate `nums2` with a single sum.
    ///
    /// # Intuition
    /// Flipping a range only changes the count of ones in `nums1`, and each type 2 query adds the
    /// same value to every position where `nums1[i] == 1`. Keeping the total ones is enough to
    /// update the sum of `nums2` directly.
    ///
    /// # Approach
    /// 1. Build a segment tree that stores the number of ones per segment.
    /// 2. For type 1, lazily flip the range (ones become `len - ones`).
    /// 3. For type 2, add `p * total_ones` to the running sum of `nums2`.
    /// 4. For type 3, record the running sum.
    ///
    /// # Complexity
    /// - Time: O((n + q) log n)
    /// - Space: O(n)
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut segment_tree = SegmentTree::new(&nums1);
        let mut sum_nums2 = nums2.iter().map(|&value| value as i64).sum::<i64>();
        let mut answers = Vec::new();

        for query in queries {
            match query[0] {
                1 => {
                    let left = query[1] as usize;
                    let right = query[2] as usize;
                    segment_tree.flip_range(left, right);
                }
                2 => {
                    let delta = query[1] as i64;
                    sum_nums2 += delta * segment_tree.total_ones();
                }
                3 => {
                    answers.push(sum_nums2);
                }
                _ => {}
            }
        }

        answers
    }
}

struct SegmentTree {
    n: usize,
    ones: Vec<i32>,
    lazy: Vec<bool>,
}

impl SegmentTree {
    fn new(bits: &[i32]) -> Self {
        let n = bits.len();
        let size = n.max(1) * 4;
        let mut ones = vec![0; size];
        let lazy = vec![false; size];

        if n > 0 {
            Self::build(1, 0, n - 1, bits, &mut ones);
        }

        Self { n, ones, lazy }
    }

    fn build(node: usize, left: usize, right: usize, bits: &[i32], ones: &mut [i32]) {
        if left == right {
            ones[node] = bits[left];
            return;
        }
        let mid = (left + right) / 2;
        let left_node = node * 2;
        let right_node = left_node + 1;
        Self::build(left_node, left, mid, bits, ones);
        Self::build(right_node, mid + 1, right, bits, ones);
        ones[node] = ones[left_node] + ones[right_node];
    }

    fn total_ones(&self) -> i64 {
        if self.n == 0 { 0 } else { self.ones[1] as i64 }
    }

    fn flip_range(&mut self, left: usize, right: usize) {
        if self.n == 0 {
            return;
        }
        self.flip_range_inner(1, 0, self.n - 1, left, right);
    }

    fn flip_range_inner(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        query_left: usize,
        query_right: usize,
    ) {
        if query_left <= left && right <= query_right {
            self.apply_flip(node, left, right);
            return;
        }

        self.push(node, left, right);
        let mid = (left + right) / 2;
        let left_node = node * 2;
        let right_node = left_node + 1;

        if query_left <= mid {
            self.flip_range_inner(left_node, left, mid, query_left, query_right);
        }
        if query_right > mid {
            self.flip_range_inner(right_node, mid + 1, right, query_left, query_right);
        }

        self.ones[node] = self.ones[left_node] + self.ones[right_node];
    }

    fn push(&mut self, node: usize, left: usize, right: usize) {
        if !self.lazy[node] || left == right {
            return;
        }
        let mid = (left + right) / 2;
        let left_node = node * 2;
        let right_node = left_node + 1;
        self.apply_flip(left_node, left, mid);
        self.apply_flip(right_node, mid + 1, right);
        self.lazy[node] = false;
    }

    fn apply_flip(&mut self, node: usize, left: usize, right: usize) {
        let len = (right - left + 1) as i32;
        self.ones[node] = len - self.ones[node];
        self.lazy[node] = !self.lazy[node];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums1 = vec![1, 0, 1];
        let nums2 = vec![0, 0, 0];
        let queries = vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]];
        assert_eq!(Solution::handle_query(nums1, nums2, queries), vec![3]);
    }

    #[test]
    fn example_two() {
        let nums1 = vec![1];
        let nums2 = vec![5];
        let queries = vec![vec![2, 0, 0], vec![3, 0, 0]];
        assert_eq!(Solution::handle_query(nums1, nums2, queries), vec![5]);
    }

    #[test]
    fn flip_and_accumulate() {
        let nums1 = vec![1, 0, 0, 1];
        let nums2 = vec![2, 1, 3, 4];
        let queries = vec![
            vec![3, 0, 0],
            vec![1, 1, 2],
            vec![2, 2, 0],
            vec![1, 0, 3],
            vec![2, 5, 0],
            vec![3, 0, 0],
        ];
        assert_eq!(Solution::handle_query(nums1, nums2, queries), vec![10, 18]);
    }

    #[test]
    fn single_element_flips() {
        let nums1 = vec![0];
        let nums2 = vec![7];
        let queries = vec![vec![3, 0, 0], vec![1, 0, 0], vec![2, 3, 0], vec![3, 0, 0]];
        assert_eq!(Solution::handle_query(nums1, nums2, queries), vec![7, 10]);
    }
}
