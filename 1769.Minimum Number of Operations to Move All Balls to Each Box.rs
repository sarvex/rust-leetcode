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
        let n = bytes.len();
        let mut result = vec![0i32; n];
        let mut count = 0i32;
        let mut ops = 0i32;
        for i in 0..n {
            result[i] += ops;
            count += (bytes[i] - b'0') as i32;
            ops += count;
        }
        count = 0;
        ops = 0;
        for i in (0..n).rev() {
            result[i] += ops;
            count += (bytes[i] - b'0') as i32;
            ops += count;
        }
        result
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
