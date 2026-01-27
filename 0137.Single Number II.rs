impl Solution {
    /// Finds the element appearing once when all others appear three times using bit counting.
    ///
    /// # Intuition
    /// For each bit position, sum the bits across all numbers. The single number's
    /// bit at that position equals the sum modulo 3.
    ///
    /// # Approach
    /// 1. For each of the 32 bit positions, sum the corresponding bit across all numbers.
    /// 2. Take the sum modulo 3 to isolate the single number's bit.
    /// 3. Combine bits with OR to reconstruct the result.
    ///
    /// # Complexity
    /// - Time: O(32n) = O(n)
    /// - Space: O(1)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        (0..32).fold(0, |ans, i| {
            let count: i32 = nums.iter().map(|v| (v >> i) & 1).sum();
            ans | ((count % 3) << i)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_single() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn mixed_values() {
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }

    #[test]
    fn negative_single() {
        assert_eq!(Solution::single_number(vec![-2, -2, 1, -2]), 1);
    }
}
