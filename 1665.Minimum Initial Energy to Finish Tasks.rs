impl Solution {
    /// Greedy sort by buffer (minimum - actual) descending to find minimum initial energy.
    ///
    /// # Intuition
    /// For two tasks A and B, compare the energy needed to do A→B vs B→A:
    /// - A then B requires initial E >= max(min_A, min_B + actual_A)
    /// - B then A requires initial E >= max(min_B, min_A + actual_B)
    ///
    /// A before B is better when `max(min_A, min_B + actual_A) <= max(min_B, min_A + actual_B)`,
    /// which simplifies to `min_A - actual_A >= min_B - actual_B`.
    /// So sort by buffer (minimum - actual) **descending**: highest buffer first.
    ///
    /// # Approach
    /// 1. Sort tasks by (minimum - actual) descending.
    /// 2. Simulate forward: track cumulative energy spent and the minimum initial
    ///    energy required. At each task, we need `initial >= spent + minimum`.
    ///    The answer is the maximum such requirement across all tasks.
    ///
    /// # Complexity
    /// - Time: O(n log n) — dominated by sorting
    /// - Space: O(1) — sort in place, constant extra space
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        // Sort by buffer descending: highest (minimum - actual) first.
        tasks.sort_unstable_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));

        // Simulate forward: at each task we need initial >= spent_so_far + minimum.
        let (_, required) = tasks.iter().fold((0, 0), |(spent, best), t| {
            let actual = t[0];
            let minimum = t[1];
            (spent + actual, best.max(spent + minimum))
        });
        required
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Optimal order: task 3 → task 2 → task 1, needs 8 initial energy
        assert_eq!(
            Solution::minimum_effort(vec![vec![1, 2], vec![2, 4], vec![4, 8]]),
            8
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::minimum_effort(vec![
                vec![1, 3],
                vec![2, 4],
                vec![10, 11],
                vec![10, 12],
                vec![8, 9]
            ]),
            32
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::minimum_effort(vec![
                vec![1, 7],
                vec![2, 8],
                vec![3, 9],
                vec![4, 10],
                vec![5, 11],
                vec![6, 12]
            ]),
            27
        );
    }

    #[test]
    fn test_single_task() {
        // Only one task: need exactly its minimum energy
        assert_eq!(Solution::minimum_effort(vec![vec![5, 10]]), 10);
    }

    #[test]
    fn test_actual_equals_minimum() {
        // No buffer: minimum == actual for all tasks
        assert_eq!(
            Solution::minimum_effort(vec![vec![3, 3], vec![5, 5], vec![2, 2]]),
            10
        );
    }

    #[test]
    fn test_large_buffer() {
        // [1,10000] has buffer 9999, [10000,10000] has buffer 0.
        // Optimal order: [1,10000] first → need 10000, then [10000,10000] → need 10001.
        assert_eq!(
            Solution::minimum_effort(vec![vec![1, 10000], vec![10000, 10000]]),
            10001
        );
    }
}
