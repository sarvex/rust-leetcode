impl Solution {
    /// Finds the maximum number achievable by swapping at most two digits.
    ///
    /// # Intuition
    /// For each digit position (from left), find the largest digit to its right
    /// and swap if it is strictly greater. The rightmost occurrence of the
    /// maximum is preferred to maximize the result.
    ///
    /// # Approach
    /// 1. Extract digits into a vector (LSB first).
    /// 2. Build a "max suffix index" array: for each position, the index of the
    ///    largest digit from position 0 to i.
    /// 3. Scan from the most significant digit; swap with the first larger digit found.
    ///
    /// # Complexity
    /// - Time: O(d) where d = number of digits
    /// - Space: O(d)
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut n = num;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        if digits.is_empty() {
            return 0;
        }
        let len = digits.len();
        let mut max_idx = vec![0usize; len];
        max_idx[0] = 0;
        for i in 1..len {
            max_idx[i] = if digits[i] > digits[max_idx[i - 1]] {
                i
            } else {
                max_idx[i - 1]
            };
        }
        for i in (0..len).rev() {
            if digits[i] != digits[max_idx[i]] {
                digits.swap(i, max_idx[i]);
                break;
            }
        }
        digits.iter().rev().fold(0, |acc, &d| acc * 10 + d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
    }

    #[test]
    fn test_no_swap() {
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::maximum_swap(5), 5);
    }
}
