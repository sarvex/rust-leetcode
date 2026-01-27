impl Solution {
    /// Compute concatenation value by pairing elements from both ends.
    ///
    /// # Intuition
    /// Two pointers from each end concatenate digits and sum. If an odd element
    /// remains in the middle, add it directly.
    ///
    /// # Approach
    /// 1. Use two pointers `i` and `j` from front and back
    /// 2. Concatenate nums[i] and nums[j] as strings, parse, and accumulate
    /// 3. Handle the middle element when i == j
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is the average digit count
    /// - Space: O(d) for string formatting
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut i = 0;
        let mut j = n - 1;

        while i < j {
            let digits_j = if nums[j] == 0 {
                1
            } else {
                (nums[j] as f64).log10().floor() as u32 + 1
            };
            ans += nums[i] as i64 * 10_i64.pow(digits_j) + nums[j] as i64;
            i += 1;
            j -= 1;
        }

        if i == j {
            ans += nums[i] as i64;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_length() {
        assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
    }

    #[test]
    fn test_odd_length() {
        assert_eq!(
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]),
            673
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::find_the_array_conc_val(vec![1]), 1);
    }
}
