impl Solution {
    /// Simulate wheel rotations tracking maximum profit.
    ///
    /// # Intuition
    /// Each rotation boards up to 4 waiting customers. Track cumulative
    /// profit (boarding revenue minus rotation cost) and record the rotation
    /// number when maximum profit is achieved.
    ///
    /// # Approach
    /// 1. Process customer arrivals and waiting queue
    /// 2. Each rotation: board min(4, waiting), update profit
    /// 3. Continue until no customers remain
    /// 4. Return the rotation with maximum profit, or -1 if never positive
    ///
    /// # Complexity
    /// - Time: O(n + total_customers / 4)
    /// - Space: O(1)
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let (mut best_rotation, mut max_profit) = (-1, 0);
        let (mut profit, mut waiting, mut rotation) = (0, 0, 0);

        while waiting > 0 || rotation < customers.len() {
            waiting += if rotation < customers.len() {
                customers[rotation]
            } else {
                0
            };
            let board = waiting.min(4);
            waiting -= board;
            rotation += 1;
            profit += board * boarding_cost - running_cost;

            if profit > max_profit {
                max_profit = profit;
                best_rotation = rotation as i32;
            }
        }

        best_rotation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profitable_operation() {
        assert_eq!(Solution::min_operations_max_profit(vec![8, 3], 5, 6), 3);
    }

    #[test]
    fn never_profitable() {
        assert_eq!(Solution::min_operations_max_profit(vec![1], 1, 100), -1);
    }

    #[test]
    fn steady_flow() {
        assert_eq!(Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4), 7);
    }
}
