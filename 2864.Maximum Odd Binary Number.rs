impl Solution {
    /// Constructs the maximum odd binary number from the given binary string.
    ///
    /// # Intuition
    /// An odd binary number must end with '1'. To maximize the value, place all
    /// remaining '1's at the most significant positions and '0's in between.
    ///
    /// # Approach
    /// 1. Count the number of '1' bits using `as_bytes()` and iterator filtering.
    /// 2. Build the result: `(cnt-1)` ones, then `(len-cnt)` zeros, then a final '1'.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of the string
    /// - Space: O(n) for the result string
    pub fn maximum_odd_binary_number(s: String) -> String {
        let cnt = s.as_bytes().iter().filter(|&&b| b == b'1').count();
        let mut result = String::with_capacity(s.len());
        (0..cnt - 1).for_each(|_| result.push('1'));
        (0..s.len() - cnt).for_each(|_| result.push('0'));
        result.push('1');
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_ones() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".to_string()),
            "001"
        );
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".to_string()),
            "1001"
        );
    }
}
