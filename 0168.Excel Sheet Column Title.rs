impl Solution {
    /// Converts a column number to its Excel sheet column title using base-26 encoding.
    ///
    /// # Intuition
    /// Excel columns use a 1-indexed base-26 system (A=1, Z=26). Subtracting 1
    /// before each modulo operation converts to 0-indexed for standard base conversion.
    ///
    /// # Approach
    /// 1. Subtract 1 to make the system 0-indexed.
    /// 2. Extract the last character via `% 26`.
    /// 3. Divide by 26 to move to the next position.
    /// 4. Reverse the collected characters.
    ///
    /// # Complexity
    /// - Time: O(log_{26} n)
    /// - Space: O(log_{26} n) for the result string
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut result = Vec::new();
        while column_number > 0 {
            column_number -= 1;
            result.push((b'A' + (column_number % 26) as u8) as char);
            column_number /= 26;
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_letter() {
        assert_eq!(Solution::convert_to_title(1), "A");
    }

    #[test]
    fn two_letters() {
        assert_eq!(Solution::convert_to_title(28), "AB");
    }

    #[test]
    fn boundary_z() {
        assert_eq!(Solution::convert_to_title(26), "Z");
    }

    #[test]
    fn large_number() {
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }
}
