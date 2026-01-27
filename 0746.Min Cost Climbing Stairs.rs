impl Solution {
    /// Finds minimum cost to climb stairs using bottom-up DP.
    ///
    /// # Intuition
    /// At each step, choose the cheaper of the previous one or two steps.
    /// Only two previous values are needed at any time.
    ///
    /// # Approach
    /// Iterate from step 2 to n, computing the minimum cost to reach each
    /// step from the two preceding steps. The answer is the cost at step n.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let (mut a, mut b) = (cost[0], cost[1]);
        for i in 2..n {
            let next = cost[i] + a.min(b);
            a = b;
            b = next;
        }
        a.min(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test_longer() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn test_two_steps() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 1]), 0);
    }
}
