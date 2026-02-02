struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn add(&mut self, mut idx: usize, delta: i64) {
        let n = self.tree.len();
        while idx < n {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn sum(&self, mut idx: usize) -> i64 {
        let mut acc = 0i64;
        while idx > 0 {
            acc += self.tree[idx];
            idx &= idx - 1;
        }
        acc
    }

    fn lower_bound(&self, target: i64) -> usize {
        let mut idx = 0usize;
        let mut bit = 1usize;
        while bit < self.tree.len() {
            bit <<= 1;
        }
        let mut acc = 0i64;
        while bit > 0 {
            let next = idx + bit;
            if next < self.tree.len() && acc + self.tree[next] < target {
                acc += self.tree[next];
                idx = next;
            }
            bit >>= 1;
        }
        idx + 1
    }
}

fn sum_k_smallest(k: usize, counts: &Fenwick, sums: &Fenwick, values: &[i32]) -> i64 {
    if k == 0 {
        return 0;
    }
    let pos = counts.lower_bound(k as i64);
    let count_before = counts.sum(pos - 1);
    let sum_before = sums.sum(pos - 1);
    let remaining = k as i64 - count_before;
    sum_before + remaining * values[pos - 1] as i64
}

impl Solution {
    /// Minimize the sum of k-1 chosen starts with a rolling window.
    ///
    /// # Intuition
    /// For a fixed last start index `i`, the other k-2 starts must lie in
    /// `[max(1, i - dist), i - 1]`, so we only need the k-2 smallest values
    /// inside that window.
    ///
    /// # Approach
    /// - Maintain a window over indices `[max(1, i - dist), i - 1]`.
    /// - Coordinate-compress values and keep Fenwick trees for counts and sums.
    /// - Query the sum of the k-2 smallest values via a prefix search.
    /// - For each `i`, the candidate cost is `nums[0] + nums[i] + sum_k_smallest`.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;
        let mut values = nums[1..].to_vec();
        values.sort_unstable();
        values.dedup();

        let mut compressed = vec![0usize; n];
        for (idx, value) in nums.iter().enumerate().skip(1) {
            let pos = values
                .binary_search(value)
                .expect("value should be present in compression")
                + 1;
            compressed[idx] = pos;
        }

        let mut counts = Fenwick::new(values.len());
        let mut sums = Fenwick::new(values.len());
        let target = k - 2;

        for idx in 1..k - 1 {
            let pos = compressed[idx];
            counts.add(pos, 1);
            sums.add(pos, nums[idx] as i64);
        }

        let mut best = sum_k_smallest(target, &counts, &sums, &values) + nums[k - 1] as i64;

        for i in k..n {
            if i > dist + 1 {
                let out_idx = i - dist - 1;
                let pos = compressed[out_idx];
                counts.add(pos, -1);
                sums.add(pos, -(nums[out_idx] as i64));
            }
            let pos = compressed[i - 1];
            counts.add(pos, 1);
            sums.add(pos, nums[i - 1] as i64);
            let candidate = sum_k_smallest(target, &counts, &sums, &values) + nums[i] as i64;
            if candidate < best {
                best = candidate;
            }
        }

        best + nums[0] as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 2, 6, 4, 2];
        assert_eq!(Solution::minimum_cost(nums, 3, 3), 5);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![10, 1, 2, 2, 2, 1];
        assert_eq!(Solution::minimum_cost(nums, 4, 3), 15);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![10, 8, 18, 9];
        assert_eq!(Solution::minimum_cost(nums, 3, 1), 36);
    }

    #[test]
    fn test_min_dist_window() {
        let nums = vec![5, 100, 1, 1];
        assert_eq!(Solution::minimum_cost(nums, 3, 1), 7);
    }

    #[test]
    fn test_max_dist_window() {
        let nums = vec![8, 5, 2, 9, 1, 4];
        assert_eq!(Solution::minimum_cost(nums, 4, 4), 15);
    }
}
