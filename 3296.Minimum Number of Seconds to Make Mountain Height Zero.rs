impl Solution {
    /// Binary search on the answer (time) to find the minimum seconds.
    ///
    /// # Intuition
    /// Worker `i` reducing height by `x` costs `workerTimes[i] * x * (x+1) / 2` seconds.
    /// Since workers operate simultaneously, the total time equals the maximum individual time.
    /// We binary search on time `T`: for each candidate, compute how much height each worker
    /// can reduce within `T` seconds and check if the total meets `mountainHeight`.
    ///
    /// # Approach
    /// 1. Binary search over time `T` in `[1, max_w * h * (h+1) / 2]`.
    /// 2. For a given `T`, each worker `i` with time `w` can reduce height by max `x` where
    ///    `w * x * (x+1) / 2 ≤ T`, i.e., `x * (x+1) ≤ 2T/w`.
    /// 3. Solve via quadratic formula: `x = floor((-1 + sqrt(1 + 8T/w)) / 2)`, with integer
    ///    adjustment to avoid floating-point error.
    /// 4. Sum all workers' reductions. If sum ≥ mountainHeight, `T` is feasible.
    ///
    /// # Complexity
    /// - Time: O(n · log(max_w · h²)) where n = workers.len(), h = mountainHeight
    /// - Space: O(1)
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let h = mountain_height as i64;

        let can_finish = |time: i64| -> bool {
            let mut total: i64 = 0;
            for &w in &worker_times {
                let w = w as i64;
                let limit = 2 * time / w;
                let mut x = ((-1.0 + (1.0 + 4.0 * limit as f64).sqrt()) / 2.0) as i64;
                while x > 0 && x * (x + 1) > limit {
                    x -= 1;
                }
                while (x + 1) * (x + 2) <= limit {
                    x += 1;
                }
                total += x;
                if total >= h {
                    return true;
                }
            }
            false
        };

        let mut lo: i64 = 1;
        let mut hi: i64 = worker_times.iter().map(|&w| w as i64).max().unwrap() * h * (h + 1) / 2;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if can_finish(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
    }

    #[test]
    fn test_single_worker() {
        assert_eq!(Solution::min_number_of_seconds(5, vec![1]), 15);
    }

    #[test]
    fn test_height_one() {
        assert_eq!(Solution::min_number_of_seconds(1, vec![5, 3, 7]), 3);
    }

    #[test]
    fn test_all_same_workers() {
        assert_eq!(Solution::min_number_of_seconds(6, vec![2, 2, 2]), 6);
    }

    #[test]
    fn test_large_worker_time() {
        assert_eq!(
            Solution::min_number_of_seconds(1, vec![1_000_000]),
            1_000_000
        );
    }
}
