impl Solution {
    /// Find the divisor with the maximum divisibility score.
    ///
    /// # Intuition
    /// For each divisor, count how many numbers in nums it divides evenly.
    /// Return the divisor with the highest count (smallest on tie).
    ///
    /// # Approach
    /// 1. For each divisor, count divisible elements in nums
    /// 2. Track the maximum count and corresponding smallest divisor
    ///
    /// # Complexity
    /// - Time: O(d * n) where d is divisors length
    /// - Space: O(1)
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut best_div = divisors[0];
        let mut max_count = 0;

        for &div in &divisors {
            let cnt = nums.iter().filter(|&&n| n % div == 0).count();
            if cnt > max_count || (cnt == max_count && div < best_div) {
                max_count = cnt;
                best_div = div;
            }
        }

        best_div
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]),
            3
        );
    }

    #[test]
    fn test_tie_smallest() {
        assert_eq!(
            Solution::max_div_score(vec![20, 14, 21, 10], vec![5, 7, 5]),
            5
        );
    }

    #[test]
    fn test_no_divisible() {
        assert_eq!(Solution::max_div_score(vec![3, 7], vec![2, 4]), 2);
    }
}
