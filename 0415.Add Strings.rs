impl Solution {
    /// Adds two non-negative integer strings using digit-by-digit simulation.
    ///
    /// # Intuition
    /// Process digits from least significant to most significant, carrying
    /// overflow just like manual addition.
    ///
    /// # Approach
    /// 1. Walk both strings backward using byte iterators.
    /// 2. Sum corresponding digits plus carry.
    /// 3. Collect result digits in reverse, then reverse the final string.
    ///
    /// # Complexity
    /// - Time: O(max(m, n))
    /// - Space: O(max(m, n))
    pub fn add_strings(num1: String, num2: String) -> String {
        let s1 = num1.as_bytes();
        let s2 = num2.as_bytes();
        let mut i = s1.len();
        let mut j = s2.len();
        let mut carry = 0u8;
        let mut result = Vec::with_capacity(i.max(j) + 1);
        while i > 0 || j > 0 || carry > 0 {
            let mut sum = carry;
            if i > 0 {
                i -= 1;
                sum += s1[i] - b'0';
            }
            if j > 0 {
                j -= 1;
                sum += s2[j] - b'0';
            }
            carry = sum / 10;
            result.push(b'0' + sum % 10);
        }
        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::add_strings("11".to_string(), "123".to_string()),
            "134"
        );
    }

    #[test]
    fn test_carry() {
        assert_eq!(
            Solution::add_strings("999".to_string(), "1".to_string()),
            "1000"
        );
    }

    #[test]
    fn test_zeros() {
        assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0");
    }
}
