impl Solution {
    /// Finds minimum time for workers to finish jobs via greedy matching.
    ///
    /// # Intuition
    /// Sorting both arrays and pairing the largest job with the fastest worker
    /// minimizes the maximum completion time across all pairings.
    ///
    /// # Approach
    /// 1. Sort jobs and workers independently
    /// 2. Pair element-wise and compute ceiling division for each pair
    /// 3. Return the maximum time across all pairs
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary (in-place sort)
    pub fn minimum_time(mut jobs: Vec<i32>, mut workers: Vec<i32>) -> i32 {
        jobs.sort_unstable();
        workers.sort_unstable();
        jobs.iter()
            .zip(workers.iter())
            .map(|(&a, &b)| (a + b - 1) / b)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        assert_eq!(Solution::minimum_time(vec![5, 2, 4], vec![1, 7, 5]), 2);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::minimum_time(vec![10], vec![3]), 4);
    }

    #[test]
    fn test_exact_division() {
        assert_eq!(Solution::minimum_time(vec![6, 4], vec![3, 2]), 2);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::minimum_time(vec![1, 1, 1], vec![1, 1, 1]), 1);
    }
}
