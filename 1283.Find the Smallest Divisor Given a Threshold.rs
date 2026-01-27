impl Solution {
    /// Binary search on the divisor value.
    ///
    /// # Intuition
    /// The sum of ceiling divisions decreases monotonically as the divisor
    /// increases. Binary search finds the smallest divisor whose resulting
    /// sum does not exceed the threshold.
    ///
    /// # Approach
    /// 1. Binary search in range `[1, max(nums)]`
    /// 2. For each candidate divisor, compute the sum of `ceil(num / divisor)`
    ///    using integer arithmetic: `(num + divisor - 1) / divisor`
    /// 3. If sum ≤ threshold, search lower; otherwise search higher
    ///
    /// # Complexity
    /// - Time: O(n log M) where M = max(nums)
    /// - Space: O(1)
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut lo, mut hi) = (1, *nums.iter().max().unwrap());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let sum: i32 = nums.iter().map(|&x| (x + mid - 1) / mid).sum();
            if sum <= threshold {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_six() {
        assert_eq!(Solution::smallest_divisor(vec![1, 2, 5, 9], 6), 5);
    }

    #[test]
    fn threshold_seven() {
        assert_eq!(Solution::smallest_divisor(vec![44, 22, 33, 11, 1], 5), 44);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::smallest_divisor(vec![10], 1), 10);
    }

    #[test]
    fn divisor_one() {
        assert_eq!(Solution::smallest_divisor(vec![1, 1, 1], 3), 1);
    }
}
