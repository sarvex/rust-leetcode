impl Solution {
    /// Finds maximum sum of a pair with equal digit sums.
    ///
    /// # Intuition
    /// Group numbers by their digit sum. For each group, track the largest value
    /// seen so far and update the answer when a second value in the same group
    /// produces a larger pair sum.
    ///
    /// # Approach
    /// 1. For each number, compute its digit sum
    /// 2. If a previous number shares the same digit sum, update max pair sum
    /// 3. Track the maximum value per digit sum bucket
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is the number of digits per element
    /// - Space: O(1) — at most 100 digit-sum buckets
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut best = [0i32; 100];
        let mut ans = -1;

        for &v in &nums {
            let digit_sum = Self::digit_sum(v) as usize;
            if best[digit_sum] > 0 {
                ans = ans.max(best[digit_sum] + v);
            }
            best[digit_sum] = best[digit_sum].max(v);
        }

        ans
    }

    #[inline]
    fn digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    }

    #[test]
    fn test_no_pair() {
        assert_eq!(Solution::maximum_sum(vec![10, 12, 19, 14]), -1);
    }

    #[test]
    fn test_all_same_digit_sum() {
        assert_eq!(Solution::maximum_sum(vec![1, 10, 100]), 101);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_sum(vec![5]), -1);
    }
}
