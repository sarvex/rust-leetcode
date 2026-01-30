impl Solution {
    /// Finds the element appearing once using two-bit masks.
    ///
    /// # Intuition
    /// Track which bits have appeared once and twice. When a bit appears a third
    /// time, it is cleared from both masks.
    ///
    /// # Approach
    /// 1. Maintain `ones` for bits seen once and `twos` for bits seen twice.
    /// 2. For each value, update masks with XOR and clear bits present in the other mask.
    /// 3. After processing all numbers, `ones` holds the unique value.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut ones, mut twos) = (0_i32, 0_i32);

        for &value in &nums {
            ones = (ones ^ value) & !twos;
            twos = (twos ^ value) & !ones;
        }

        ones
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
