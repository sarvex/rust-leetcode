impl Solution {
    /// Computes move costs with prefix and suffix sweeps.
    ///
    /// # Intuition
    /// For each box, the total moves equal the sum of distances from all balls.
    /// Two passes (left-to-right and right-to-left) accumulate the contribution
    /// of balls from each side efficiently.
    ///
    /// # Approach
    /// 1. Left pass: track cumulative ball count and running cost from the left.
    /// 2. Right pass: track cumulative ball count and running cost from the right.
    /// 3. Sum both passes for each position.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let bytes = boxes.as_bytes();

        // Left pass: accumulate ops from balls to the left
        let left: Vec<i32> = bytes
            .iter()
            .scan((0i32, 0i32), |(count, ops), &b| {
                let current_ops = *ops;
                *count += (b - b'0') as i32;
                *ops += *count;
                Some(current_ops)
            })
            .collect();

        // Right pass: accumulate ops from balls to the right
        let right: Vec<i32> = bytes
            .iter()
            .rev()
            .scan((0i32, 0i32), |(count, ops), &b| {
                let current_ops = *ops;
                *count += (b - b'0') as i32;
                *ops += *count;
                Some(current_ops)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // Combine both passes
        left.into_iter().zip(right).map(|(l, r)| l + r).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::min_operations("110".to_string()), vec![1, 1, 3]);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::min_operations("001011".to_string()),
            vec![11, 8, 5, 4, 3, 1]
        );
    }
}
