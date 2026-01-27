impl Solution {
    /// Combinatorial counting of valid pickup-delivery orderings.
    ///
    /// # Intuition
    /// For the i-th order, there are `2i - 1` valid positions to insert the
    /// delivery after its pickup among `2i - 1` gaps. The total count is the
    /// product `∏ i * (2i - 1)` for i from 2 to n.
    ///
    /// # Approach
    /// 1. Iterate from 2 to n, multiplying by `i * (2i - 1)` modulo 10^9+7
    /// 2. Return the accumulated product
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_orders(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut result = 1i64;
        for i in 2..=n as i64 {
            result = result * i % MOD * (2 * i - 1) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_order() {
        assert_eq!(Solution::count_orders(1), 1);
    }

    #[test]
    fn two_orders() {
        assert_eq!(Solution::count_orders(2), 6);
    }

    #[test]
    fn three_orders() {
        assert_eq!(Solution::count_orders(3), 90);
    }
}
