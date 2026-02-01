impl Solution {
    /// Counts peaks under point updates using a Fenwick tree.
    ///
    /// # Intuition
    /// A peak depends only on its immediate neighbors, so changing one element can only affect the
    /// peak status at indices `index - 1`, `index`, and `index + 1`. Range queries then become a
    /// sum over the interior indices of the subarray.
    ///
    /// # Approach
    /// - Precompute `is_peak[i]` for each index and store it in a Fenwick tree.
    /// - For query `[1, l, r]`, return the sum on indices `l + 1 ..= r - 1`.
    /// - For update `[2, index, value]`, set `nums[index] = value`, then recompute the peak status
    ///   for the affected indices and update the Fenwick tree with the deltas.
    ///
    /// # Complexity
    /// - Time: O((n + q) log n)
    /// - Space: O(n)
    pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut peaks = vec![0i32; n];
        let mut bit = Fenwick::new(n);

        for i in 1..n.saturating_sub(1) {
            if is_peak_at(&nums, i) {
                peaks[i] = 1;
                bit.add(i, 1);
            }
        }

        let mut answers = Vec::new();
        for query in queries {
            if query.len() != 3 {
                continue;
            }
            match query[0] {
                1 => {
                    let left = query[1] as usize;
                    let right = query[2] as usize;
                    if right <= left + 1 {
                        answers.push(0);
                        continue;
                    }
                    let sum = bit.range_sum(left + 1, right - 1);
                    answers.push(sum);
                }
                2 => {
                    let index = query[1] as usize;
                    let value = query[2];
                    if index >= n {
                        continue;
                    }
                    let start = index.saturating_sub(1);
                    let end = (index + 1).min(n - 1);
                    nums[index] = value;
                    for pos in start..=end {
                        let new_peak = if is_peak_at(&nums, pos) { 1 } else { 0 };
                        let old_peak = peaks[pos];
                        if new_peak != old_peak {
                            bit.add(pos, new_peak - old_peak);
                            peaks[pos] = new_peak;
                        }
                    }
                }
                _ => {}
            }
        }

        answers
    }
}

fn is_peak_at(nums: &[i32], index: usize) -> bool {
    if index == 0 || index + 1 >= nums.len() {
        return false;
    }
    nums[index] > nums[index - 1] && nums[index] > nums[index + 1]
}

struct Fenwick {
    tree: Vec<i32>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn add(&mut self, index: usize, delta: i32) {
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix_sum(&self, index: usize) -> i32 {
        let mut i = index + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    fn range_sum(&self, left: usize, right: usize) -> i32 {
        if left > right {
            return 0;
        }
        if left == 0 {
            return self.prefix_sum(right);
        }
        self.prefix_sum(right) - self.prefix_sum(left - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 1, 4, 2, 5];
        let queries = vec![vec![2, 3, 4], vec![1, 0, 4]];
        assert_eq!(Solution::count_of_peaks(nums, queries), vec![0]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 1, 4, 2, 1, 5];
        let queries = vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]];
        assert_eq!(Solution::count_of_peaks(nums, queries), vec![0, 1]);
    }

    #[test]
    fn test_updates_toggle_peaks() {
        let nums = vec![1, 3, 2, 4, 1];
        let queries = vec![vec![1, 0, 4], vec![2, 3, 0], vec![1, 0, 4]];
        assert_eq!(Solution::count_of_peaks(nums, queries), vec![2, 1]);
    }

    #[test]
    fn test_small_range_no_peaks() {
        let nums = vec![1, 3, 2];
        let queries = vec![vec![1, 0, 1], vec![1, 1, 2], vec![1, 0, 2]];
        assert_eq!(Solution::count_of_peaks(nums, queries), vec![0, 0, 1]);
    }
}
