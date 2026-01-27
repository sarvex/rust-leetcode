impl Solution {
    /// Constructs the lexicographically largest binary string after allowed operations.
    ///
    /// # Intuition
    /// The operations can turn "00" → "10" and "10" → "01". This means all zeros
    /// after the first zero can be gathered and converted to ones, leaving exactly
    /// one zero whose position equals the index of the first zero plus the count
    /// of remaining zeros.
    ///
    /// # Approach
    /// 1. Find the position of the first '0'.
    /// 2. Count remaining zeros after that position.
    /// 3. The single remaining '0' sits at index `first_zero + remaining_zeros`.
    /// 4. Build the result with all '1's except that single '0'.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn maximum_binary_string(binary: String) -> String {
        match binary.find('0') {
            Some(k) => {
                let zeros_after = binary[k + 1..].bytes().filter(|&b| b == b'0').count();
                let pos = k + zeros_after;
                let n = binary.len();
                format!("{}{}{}", "1".repeat(pos), "0", "1".repeat(n - pos - 1))
            }
            None => binary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::maximum_binary_string("000110".to_string()),
            "111011"
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::maximum_binary_string("01".to_string()), "01");
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::maximum_binary_string("111".to_string()), "111");
    }
}
