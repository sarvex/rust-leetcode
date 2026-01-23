impl Solution {
    /// Bit manipulation to find minimum ans where ans OR (ans+1) equals num
    ///
    /// # Intuition
    /// For ans OR (ans+1) to equal num, num must be odd (since adding 1 flips the lowest 0 bit).
    /// The key insight is that ans OR (ans+1) always sets consecutive bits from some position.
    /// We need to find the lowest set bit in num that can be "unset" in ans.
    ///
    /// # Approach
    /// For each num:
    /// 1. If num is even (2), return -1 (no valid ans exists for even numbers)
    /// 2. Find the lowest 0 bit position in num - this determines where the carry stops
    /// 3. The answer is num with the bit just below the lowest 0 bit cleared
    /// 4. Equivalently: ans = num - (1 << (trailing_ones_count - 1)) for odd numbers > 1
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of nums
    /// - Space: O(n) for the result array
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|&num| {
                if num == 2 {
                    -1
                } else {
                    let trailing_ones = num.trailing_ones();
                    num - (1 << (trailing_ones - 1))
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_bitwise_array(vec![2, 3, 5, 7]), vec![-1, 1, 4, 3]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_bitwise_array(vec![11, 13, 31]), vec![9, 12, 15]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_bitwise_array(vec![3]), vec![1]);
    }
}
