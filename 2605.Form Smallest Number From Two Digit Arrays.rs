impl Solution {
    /// Form the smallest number from two digit arrays.
    ///
    /// # Intuition
    /// If a common digit exists, the smallest common digit is the answer.
    /// Otherwise, combine the smallest digit from each array into a two-digit number.
    ///
    /// # Approach
    /// 1. Check all pairs: if equal, track the minimum common digit
    /// 2. If no common digit, form min(a*10+b, b*10+a) from the two minimums
    /// 3. Return the overall minimum
    ///
    /// # Complexity
    /// - Time: O(n * m) where n, m are array lengths
    /// - Space: O(1)
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 100;

        for &a in &nums1 {
            for &b in &nums2 {
                if a == b {
                    ans = ans.min(a);
                } else {
                    ans = ans.min((a * 10 + b).min(b * 10 + a));
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_digit() {
        assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7, 1]), 1);
    }

    #[test]
    fn test_no_common() {
        assert_eq!(Solution::min_number(vec![3, 5, 2, 6], vec![8, 7, 4]), 23);
    }

    #[test]
    fn test_single_elements() {
        assert_eq!(Solution::min_number(vec![9], vec![1]), 19);
    }
}
