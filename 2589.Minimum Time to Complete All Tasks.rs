impl Solution {
    /// Schedule tasks greedily to minimize total active time slots.
    ///
    /// # Intuition
    /// Sort tasks by end time. For each task, reuse already-active slots and
    /// activate new ones from the end (latest first) to maximize overlap with
    /// future tasks.
    ///
    /// # Approach
    /// 1. Sort tasks by end time
    /// 2. For each task, count already-active slots in its range
    /// 3. Fill remaining duration from the task's end backward
    ///
    /// # Complexity
    /// - Time: O(t * r) where t is task count and r is the time range
    /// - Space: O(r)
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1]);
        let mut active = vec![false; 2001];
        let mut ans = 0;

        for task in &tasks {
            let start = task[0] as usize;
            let end = task[1] as usize;
            let existing: i32 = active[start..=end].iter().filter(|&&v| v).count() as i32;
            let mut remaining = task[2] - existing;
            let mut i = end;

            while remaining > 0 {
                if !active[i] {
                    active[i] = true;
                    remaining -= 1;
                    ans += 1;
                }
                if i == start {
                    break;
                }
                i -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapping_tasks() {
        assert_eq!(
            Solution::find_minimum_time(vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]]),
            2
        );
    }

    #[test]
    fn test_non_overlapping() {
        assert_eq!(
            Solution::find_minimum_time(vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]]),
            4
        );
    }

    #[test]
    fn test_single_task() {
        assert_eq!(Solution::find_minimum_time(vec![vec![1, 10, 5]]), 5);
    }
}
