impl Solution {
    /// Finds the box with the most balls using digit-sum bucketing.
    ///
    /// # Intuition
    /// Each ball numbered `x` goes into box `digit_sum(x)`. Count occurrences
    /// per digit sum and return the maximum count.
    ///
    /// # Approach
    /// 1. For each number in `[low, high]`, compute its digit sum.
    /// 2. Increment the corresponding bucket.
    /// 3. Return the maximum bucket value.
    ///
    /// # Complexity
    /// - Time: O((high - low) × d) where d is the number of digits
    /// - Space: O(1) — at most 46 possible digit sums for values up to 10^5
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut cnt = [0i32; 46];
        for x in low_limit..=high_limit {
            let mut sum = 0usize;
            let mut n = x;
            while n > 0 {
                sum += (n % 10) as usize;
                n /= 10;
            }
            cnt[sum] += 1;
        }
        cnt.iter().copied().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::count_balls(1, 10), 2);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::count_balls(5, 15), 2);
    }

    #[test]
    fn test_example_three() {
        assert_eq!(Solution::count_balls(19, 28), 2);
    }
}
