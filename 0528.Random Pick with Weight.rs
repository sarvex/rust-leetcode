use rand::{Rng, thread_rng};

/// Weighted random index picker using prefix sums and binary search.
///
/// # Intuition
/// Transform weights into cumulative distribution. Binary search locates
/// the index corresponding to a random value in the total weight range.
///
/// # Approach
/// 1. Precompute prefix sums of weights.
/// 2. Generate random number in [1, total_weight].
/// 3. Binary search for the first prefix sum >= target.
///
/// # Complexity
/// - Time: O(n) for construction, O(log n) per pick
/// - Space: O(n)
struct Solution {
    prefix: Vec<i32>,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for &weight in &w {
            sum += weight;
            prefix.push(sum);
        }
        Self { prefix }
    }

    fn pick_index(&self) -> i32 {
        let total = *self.prefix.last().unwrap();
        let target = thread_rng().gen_range(1..=total);
        match self.prefix.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Test version with deterministic random number
    struct TestSolution {
        prefix: Vec<i32>,
    }

    impl TestSolution {
        fn new(w: Vec<i32>) -> Self {
            let mut prefix = Vec::with_capacity(w.len());
            let mut sum = 0;
            for &weight in &w {
                sum += weight;
                prefix.push(sum);
            }
            Self { prefix }
        }

        fn pick_index_with_target(&self, target: i32) -> i32 {
            match self.prefix.binary_search(&target) {
                Ok(i) => i as i32,
                Err(i) => i as i32,
            }
        }

        fn get_total(&self) -> i32 {
            *self.prefix.last().unwrap()
        }
    }

    #[test]
    fn test_single_weight() {
        let solution = TestSolution::new(vec![1]);
        assert_eq!(solution.pick_index_with_target(1), 0);
    }

    #[test]
    fn test_equal_weights() {
        let solution = TestSolution::new(vec![1, 1, 1]);
        assert_eq!(solution.pick_index_with_target(1), 0);
        assert_eq!(solution.pick_index_with_target(2), 1);
        assert_eq!(solution.pick_index_with_target(3), 2);
    }

    #[test]
    fn test_different_weights() {
        let solution = TestSolution::new(vec![1, 3, 2]);
        // Prefix: [1, 4, 6]
        assert_eq!(solution.pick_index_with_target(1), 0); // Weight 1: target 1
        assert_eq!(solution.pick_index_with_target(2), 1); // Weight 3: targets 2-4
        assert_eq!(solution.pick_index_with_target(3), 1);
        assert_eq!(solution.pick_index_with_target(4), 1);
        assert_eq!(solution.pick_index_with_target(5), 2); // Weight 2: targets 5-6
        assert_eq!(solution.pick_index_with_target(6), 2);
    }

    #[test]
    fn test_large_weight_difference() {
        let solution = TestSolution::new(vec![1, 99]);
        // Prefix: [1, 100]
        assert_eq!(solution.pick_index_with_target(1), 0);
        assert_eq!(solution.pick_index_with_target(2), 1);
        assert_eq!(solution.pick_index_with_target(50), 1);
        assert_eq!(solution.pick_index_with_target(100), 1);
    }

    #[test]
    fn test_boundary_cases() {
        let solution = TestSolution::new(vec![3, 14, 1, 7]);
        // Prefix: [3, 17, 18, 25]
        assert_eq!(solution.pick_index_with_target(1), 0);
        assert_eq!(solution.pick_index_with_target(3), 0);
        assert_eq!(solution.pick_index_with_target(4), 1);
        assert_eq!(solution.pick_index_with_target(17), 1);
        assert_eq!(solution.pick_index_with_target(18), 2);
        assert_eq!(solution.pick_index_with_target(19), 3);
        assert_eq!(solution.pick_index_with_target(25), 3);
    }

    #[test]
    fn test_distribution() {
        // Test that the actual implementation produces reasonable distribution
        let solution = Solution::new(vec![1, 3]);
        let mut counts = HashMap::new();

        for _ in 0..1000 {
            let idx = solution.pick_index();
            *counts.entry(idx).or_insert(0) += 1;
        }

        // With weights [1, 3], we expect roughly 25% index 0, 75% index 1
        // Allow for statistical variation
        assert!(counts.contains_key(&0));
        assert!(counts.contains_key(&1));

        let count_0 = counts[&0];
        let count_1 = counts[&1];

        // Very loose bounds to avoid flaky tests
        assert!(count_0 > 100 && count_0 < 400, "Index 0 count: {}", count_0);
        assert!(count_1 > 600 && count_1 < 900, "Index 1 count: {}", count_1);
    }
}
