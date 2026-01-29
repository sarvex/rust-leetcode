impl Solution {
    /// Finds the two numbers appearing once using XOR and bit partitioning.
    ///
    /// # Intuition
    /// XOR of all elements gives `a ^ b`. The lowest set bit in this result
    /// differs between a and b, allowing partitioning into two groups.
    ///
    /// # Approach
    /// 1. XOR all elements to get `a ^ b`.
    /// 2. Find the lowest set bit as a partition mask.
    /// 3. XOR elements where the bit is set to isolate one number.
    /// 4. XOR the result with `a ^ b` to get the other.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_all = nums.iter().fold(0, |acc, x| acc ^ *x);
        let lowest_bit = xor_all & (-xor_all);
        let a = nums
            .iter()
            .filter(|x| **x & lowest_bit != 0)
            .fold(0, |acc, x| acc ^ *x);
        vec![a, xor_all ^ a]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_unique_numbers() {
        let mut result = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        result.sort();
        assert_eq!(result, vec![3, 5]);
    }

    #[test]
    fn negative_numbers() {
        let mut result = Solution::single_number(vec![-1, 0]);
        result.sort();
        assert_eq!(result, vec![-1, 0]);
    }
}
