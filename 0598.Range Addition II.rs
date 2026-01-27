impl Solution {
    /// Counts elements equal to the maximum after all range additions.
    ///
    /// # Intuition
    /// Each operation increments the top-left rectangle [0..a, 0..b]. The
    /// maximum value is at (0,0), and the count equals the intersection of
    /// all rectangles: min(a) × min(b).
    ///
    /// # Approach
    /// 1. Find the minimum row bound and column bound across all operations.
    /// 2. Return min_row × min_col, or m × n if no operations.
    ///
    /// # Complexity
    /// - Time: O(k) where k = number of operations
    /// - Space: O(1)
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let min_r = ops.iter().map(|op| op[0]).min().unwrap_or(m);
        let min_c = ops.iter().map(|op| op[1]).min().unwrap_or(n);
        min_r * min_c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
    }

    #[test]
    fn test_no_ops() {
        assert_eq!(Solution::max_count(3, 3, vec![]), 9);
    }

    #[test]
    fn test_single_op() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![1, 1]]), 1);
    }
}
