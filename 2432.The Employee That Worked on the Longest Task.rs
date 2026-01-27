impl Solution {
    /// Finds the employee who worked on the longest single task.
    ///
    /// # Intuition
    /// Each log entry records cumulative end time, so the task duration is the
    /// difference between consecutive end times. Track the maximum duration
    /// and break ties by choosing the smallest employee ID.
    ///
    /// # Approach
    /// 1. Fold over logs while tracking previous end time
    /// 2. Compute each task duration as current end time minus previous
    /// 3. Update the result when a longer task is found or a tie with smaller ID
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through logs
    /// - Space: O(1)
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        logs.iter()
            .fold((0, 0, 0), |(res, max_dur, prev), log| {
                let duration = log[1] - prev;
                if duration > max_dur || (duration == max_dur && log[0] < res) {
                    (log[0], duration, log[1])
                } else {
                    (res, max_dur, log[1])
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let logs = vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]];
        assert_eq!(Solution::hardest_worker(10, logs), 1);
    }

    #[test]
    fn test_example_2() {
        let logs = vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]];
        assert_eq!(Solution::hardest_worker(26, logs), 3);
    }

    #[test]
    fn test_tie_returns_smallest_id() {
        let logs = vec![vec![5, 10], vec![3, 20]];
        assert_eq!(Solution::hardest_worker(10, logs), 3);
    }

    #[test]
    fn test_single_task() {
        let logs = vec![vec![0, 5]];
        assert_eq!(Solution::hardest_worker(1, logs), 0);
    }
}
