impl Solution {
    /// Grade-school digit-by-digit multiplication of two number strings.
    ///
    /// # Intuition
    /// Multiplying two numbers by hand works digit-by-digit with carry
    /// propagation. Using a result array indexed by the sum of digit
    /// positions accumulates partial products naturally.
    ///
    /// # Approach
    /// Iterate through each digit of `num1` (right to left), multiply by
    /// each digit of `num2`, and accumulate into a result vector at position
    /// `i + j`. Propagate carries as each position is updated. Convert the
    /// reversed result to a string.
    ///
    /// # Complexity
    /// - Time: O(n × m) — every digit pair multiplied once
    /// - Space: O(n + m) — result array length
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return String::from("0");
        }

        let (d1, d2) = (num1.as_bytes(), num2.as_bytes());
        let (n, m) = (d1.len(), d2.len());
        let mut result = vec![0u8; n + m];

        for i in (0..n).rev() {
            let a = d1[i] - b'0';
            for j in (0..m).rev() {
                let b = d2[j] - b'0';
                let sum = a * b + result[i + j + 1];
                result[i + j + 1] = sum % 10;
                result[i + j] += sum / 10;
            }
        }

        result
            .iter()
            .skip_while(|d| **d == 0)
            .map(|d| (*d + b'0') as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6");
    }

    #[test]
    fn multi_digit() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088"
        );
    }

    #[test]
    fn zero_input() {
        assert_eq!(
            Solution::multiply("0".to_string(), "12345".to_string()),
            "0"
        );
    }

    #[test]
    fn large_numbers() {
        assert_eq!(
            Solution::multiply("999".to_string(), "999".to_string()),
            "998001"
        );
    }
}
