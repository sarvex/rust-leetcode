use std::collections::VecDeque;

impl Solution {
    /// Simulates the door crossing with priority rules for enter/exit.
    ///
    /// # Intuition
    /// When both queues have people ready, the last action determines priority:
    /// if the door was last used for exit (or unused), exit has priority; otherwise enter.
    ///
    /// # Approach
    /// 1. Maintain two queues: one for entering (state=0), one for exiting (state=1)
    /// 2. At each time step, enqueue all arrivals up to current time
    /// 3. If both queues non-empty, prioritize based on last state
    /// 4. If only one queue has people, process it and update last state
    /// 5. If both empty, advance time and reset state to exit priority
    ///
    /// # Complexity
    /// - Time: O(n + T) where T is the max arrival time
    /// - Space: O(n) — two queues and result array
    pub fn time_taken(arrival: Vec<i32>, state: Vec<i32>) -> Vec<i32> {
        let n = arrival.len();
        let mut queues = [VecDeque::new(), VecDeque::new()];
        let mut t = 0i32;
        let mut i = 0;
        let mut last_state = 1usize;
        let mut ans = vec![-1i32; n];

        while i < n || !queues[0].is_empty() || !queues[1].is_empty() {
            while i < n && arrival[i] <= t {
                queues[state[i] as usize].push_back(i);
                i += 1;
            }

            if !queues[0].is_empty() && !queues[1].is_empty() {
                let front = queues[last_state].pop_front().unwrap();
                ans[front] = t;
            } else if !queues[0].is_empty() || !queues[1].is_empty() {
                last_state = if queues[0].is_empty() { 1 } else { 0 };
                let front = queues[last_state].pop_front().unwrap();
                ans[front] = t;
            } else {
                last_state = 1;
                if i < n {
                    t = arrival[i] - 1;
                }
            }

            t += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::time_taken(vec![0, 1, 1, 2, 4], vec![0, 1, 0, 0, 1]),
            vec![0, 3, 1, 2, 4]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::time_taken(vec![0, 0, 0], vec![1, 0, 1]),
            vec![0, 2, 1]
        );
    }

    #[test]
    fn test_single_person() {
        assert_eq!(Solution::time_taken(vec![0], vec![1]), vec![0]);
    }
}
