impl Solution {
    /// Maximizes bags filled to capacity using additional rocks.
    ///
    /// # Intuition
    /// Greedily fill bags that need the fewest additional rocks first, maximizing
    /// the total number of full bags.
    ///
    /// # Approach
    /// 1. Compute remaining capacity for each bag (capacity - rocks)
    /// 2. Sort remaining capacities in ascending order
    /// 3. Greedily fill bags using `take_while` until rocks run out
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) auxiliary (mutates in place)
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remaining: Vec<i32> = capacity
            .iter()
            .zip(rocks.iter())
            .map(|(&c, &r)| c - r)
            .collect();
        remaining.sort_unstable();

        let mut budget = additional_rocks;
        remaining
            .iter()
            .take_while(|&&need| {
                budget -= need;
                budget >= 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2),
            3
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100),
            3
        );
    }

    #[test]
    fn test_no_additional_rocks() {
        assert_eq!(Solution::maximum_bags(vec![1, 1], vec![1, 0], 0), 1);
    }

    #[test]
    fn test_all_already_full() {
        assert_eq!(Solution::maximum_bags(vec![3, 3], vec![3, 3], 5), 2);
    }
}
