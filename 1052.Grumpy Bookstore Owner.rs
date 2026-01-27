impl Solution {
    /// Maximizes satisfied customers using a sliding window for the secret technique.
    ///
    /// # Intuition
    /// Without the technique, sum customers on non-grumpy minutes. The technique
    /// converts a window of `minutes` grumpy minutes to non-grumpy. Maximize
    /// the extra customers gained from the best window.
    ///
    /// # Approach
    /// Compute base satisfaction (non-grumpy minutes). Sliding window of size
    /// `minutes` over grumpy minutes to find the maximum recoverable customers.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes as usize;
        let n = customers.len();

        let base: i32 = customers
            .iter()
            .zip(grumpy.iter())
            .map(|(&c, &g)| c * (1 - g))
            .sum();

        let mut window: i32 = (0..minutes).map(|i| customers[i] * grumpy[i]).sum();
        let mut max_extra = window;

        for i in minutes..n {
            window += customers[i] * grumpy[i];
            window -= customers[i - minutes] * grumpy[i - minutes];
            max_extra = max_extra.max(window);
        }

        base + max_extra
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3,
            ),
            16
        );
    }

    #[test]
    fn test_all_happy() {
        assert_eq!(Solution::max_satisfied(vec![1, 2, 3], vec![0, 0, 0], 1), 6);
    }
}
