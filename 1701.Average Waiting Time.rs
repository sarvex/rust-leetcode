impl Solution {
    /// Simulates sequential order processing to compute average wait time.
    ///
    /// # Intuition
    /// Each customer must wait until the chef finishes the previous order (or
    /// starts immediately if idle). Track the current finish time and accumulate
    /// each customer's wait.
    ///
    /// # Approach
    /// 1. Maintain current time `t` starting at 0.
    /// 2. For each customer `[arrival, duration]`, set `t = max(t, arrival) + duration`.
    /// 3. Accumulate `t - arrival` as the wait for this customer.
    /// 4. Return total wait divided by number of customers.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let (total, _) = customers.iter().fold((0.0_f64, 0_i64), |(total, t), c| {
            let arrival = c[0] as i64;
            let duration = c[1] as i64;
            let finish = t.max(arrival) + duration;
            (total + (finish - arrival) as f64, finish)
        });
        total / customers.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let result = Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]);
        assert!((result - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_example_two() {
        let result =
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]);
        assert!((result - 3.25).abs() < 1e-5);
    }
}
