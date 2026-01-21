impl Solution {
    /// Bit manipulation to find minimum bitwise array values
    ///
    /// # Intuition
    /// For `x | (x+1)` to equal n, adding 1 to x flips the lowest 0-bit to 1 and
    /// clears trailing 1s. The OR result is always odd, so even n has no solution.
    ///
    /// # Approach
    /// 1. If n is even, return -1 (no solution exists)
    /// 2. Find lowest 0-bit using `!n & (n+1)`
    /// 3. Flip off the bit just below it: `n - ((~n & (n+1)) >> 1)`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|n| {
                if n & 1 == 0 {
                    -1
                } else {
                    n - ((!n & (n + 1)) >> 1)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_bitwise_array(vec![2, 3, 5, 7]),
            vec![-1, 1, 4, 3]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_bitwise_array(vec![11, 13, 31]),
            vec![9, 12, 15]
        );
    }
}
