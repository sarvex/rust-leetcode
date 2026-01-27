use std::collections::HashMap;

impl Solution {
    /// Finds minimum rounds to complete all tasks, doing 2 or 3 identical tasks per round.
    ///
    /// # Intuition
    /// Group tasks by difficulty. Any count of 1 is impossible. For count >= 2,
    /// greedy use of groups-of-3 with remainder handling yields the minimum rounds.
    ///
    /// # Approach
    /// 1. Count task frequencies using a HashMap
    /// 2. For each frequency: if 1, return -1; otherwise rounds = ceil(count / 3)
    /// 3. Sum all rounds using fold, short-circuiting on impossibility
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the frequency map
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut freq = HashMap::with_capacity(tasks.len());
        for &t in &tasks {
            *freq.entry(t).or_insert(0) += 1;
        }

        freq.values()
            .try_fold(0, |acc, &v| {
                if v == 1 {
                    None
                } else {
                    Some(acc + (v + 2) / 3)
                }
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]),
            4
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::minimum_rounds(vec![2, 3, 3]), -1);
    }

    #[test]
    fn test_all_same_count_two() {
        assert_eq!(Solution::minimum_rounds(vec![1, 1]), 1);
    }

    #[test]
    fn test_all_same_count_three() {
        assert_eq!(Solution::minimum_rounds(vec![1, 1, 1]), 1);
    }

    #[test]
    fn test_count_four() {
        assert_eq!(Solution::minimum_rounds(vec![1, 1, 1, 1]), 2);
    }

    #[test]
    fn test_single_task_impossible() {
        assert_eq!(Solution::minimum_rounds(vec![5]), -1);
    }
}
