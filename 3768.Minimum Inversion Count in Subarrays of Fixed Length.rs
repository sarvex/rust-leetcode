struct BIT {
    tree: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 2],
        }
    }

    fn update(&mut self, mut i: usize, delta: i64) {
        i += 1;
        let len = self.tree.len();
        while i < len {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn query(&self, mut i: usize) -> i64 {
        i += 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}

impl Solution {
    /// Sliding window with BIT for minimum inversion count.
    ///
    /// # Intuition
    /// Use a sliding window of size k and maintain inversion count incrementally.
    /// When sliding, the delta equals: (elements > incoming) - (elements < outgoing).
    ///
    /// # Approach
    /// 1. Coordinate compress values to reduce BIT size
    /// 2. Compute initial inversion count for first window
    /// 3. Slide window: update inversions using BIT prefix sums
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn min_inversion_count(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;

        if k <= 1 {
            return 0;
        }

        let mut sorted: Vec<i32> = nums.clone();
        sorted.sort_unstable();
        sorted.dedup();

        let rank: Vec<usize> = nums
            .iter()
            .map(|x| sorted.binary_search(x).unwrap())
            .collect();

        let max_rank = sorted.len();
        let mut bit = BIT::new(max_rank);
        let mut inversions: i64 = 0;

        rank[..k].iter().enumerate().for_each(|(i, &r)| {
            inversions += (i as i64) - bit.query(r);
            bit.update(r, 1);
        });

        let mut min_inversions = inversions;
        let k_minus_one = (k - 1) as i64;

        for i in k..n {
            let out_rank = rank[i - k];
            let in_rank = rank[i];

            bit.update(out_rank, -1);
            if out_rank > 0 {
                inversions -= bit.query(out_rank - 1);
            }

            inversions += k_minus_one - bit.query(in_rank);
            bit.update(in_rank, 1);

            min_inversions = min_inversions.min(inversions);
        }

        min_inversions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_with_zero_inversions() {
        assert_eq!(Solution::min_inversion_count(vec![3, 1, 2, 5, 4], 3), 0);
    }

    #[test]
    fn test_fully_reversed_array() {
        assert_eq!(Solution::min_inversion_count(vec![5, 3, 2, 1], 4), 6);
    }

    #[test]
    fn test_window_size_one() {
        assert_eq!(Solution::min_inversion_count(vec![2, 1], 1), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_inversion_count(vec![1], 1), 0);
    }

    #[test]
    fn test_sorted_pair() {
        assert_eq!(Solution::min_inversion_count(vec![1, 2], 2), 0);
    }

    #[test]
    fn test_reversed_pair() {
        assert_eq!(Solution::min_inversion_count(vec![2, 1], 2), 1);
    }
}
