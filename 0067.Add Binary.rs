impl Solution {
    /// Right-to-left digit addition with carry for binary string sum.
    ///
    /// # Intuition
    /// Binary addition works like decimal addition: sum corresponding digits
    /// plus carry, write the remainder, and propagate the carry leftward.
    ///
    /// # Approach
    /// Iterate from the rightmost byte of both strings simultaneously. Sum
    /// the two digits and the carry, append the remainder bit to a result
    /// buffer, and update the carry. Reverse the buffer at the end.
    ///
    /// # Complexity
    /// - Time: O(max(n, m)) — processes the longer string fully
    /// - Space: O(max(n, m)) — result string
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut carry = 0u8;
        let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);

        while i >= 0 || j >= 0 || carry > 0 {
            if i >= 0 {
                carry += a[i as usize] - b'0';
                i -= 1;
            }
            if j >= 0 {
                carry += b[j as usize] - b'0';
                j -= 1;
            }
            result.push((carry % 2) + b'0');
            carry /= 2;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
    }

    #[test]
    fn longer_strings() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        );
    }

    #[test]
    fn both_zero() {
        assert_eq!(Solution::add_binary("0".to_string(), "0".to_string()), "0");
    }

    #[test]
    fn different_lengths() {
        assert_eq!(
            Solution::add_binary("1".to_string(), "111".to_string()),
            "1000"
        );
    }
}
