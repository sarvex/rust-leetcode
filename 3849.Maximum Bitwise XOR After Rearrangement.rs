impl Solution {
    /// Returns the maximum binary string obtainable by XORing s with a rearrangement of t.
    ///
    /// # Intuition
    /// To maximize the resulting integer, we want the leftmost bits to be 1 when possible.
    /// XOR gives 1 when bits differ, so at each position we prefer assigning from t the bit
    /// that differs from s[i].
    ///
    /// # Approach
    /// The two resources (ones and zeros in t) are independent: ones pair with
    /// source-zeros, zeros pair with source-ones. Precompute thresholds for how
    /// many of each can produce XOR 1, then scan once, placing 1s greedily at
    /// the leftmost eligible positions.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result string.
    pub fn maximum_xor(source: String, target: String) -> String {
        let source_ones = source.as_bytes().iter().filter(|b| **b == b'1').count();
        let source_zeros = source.len() - source_ones;
        let target_ones = target.as_bytes().iter().filter(|b| **b == b'1').count();
        let target_zeros = target.len() - target_ones;

        let ones_budget_for_zeros = source_zeros.min(target_ones);
        let ones_budget_for_ones = source_ones.min(target_zeros);

        let mut seen_zeros = 0usize;
        let mut seen_ones = 0usize;

        let mut result = String::with_capacity(source.len());
        for &byte in source.as_bytes() {
            if byte == b'0' {
                seen_zeros += 1;
                result.push(if seen_zeros <= ones_budget_for_zeros { '1' } else { '0' });
            } else {
                seen_ones += 1;
                result.push(if seen_ones <= ones_budget_for_ones { '1' } else { '0' });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximum_xor("101".to_string(), "011".to_string()),
            "110"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::maximum_xor("0110".to_string(), "1110".to_string()),
            "1101"
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::maximum_xor("0101".to_string(), "1001".to_string()),
            "1111"
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::maximum_xor("0".to_string(), "1".to_string()), "1");
        assert_eq!(Solution::maximum_xor("1".to_string(), "0".to_string()), "1");
    }
}
