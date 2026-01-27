impl Solution {
    /// Adds integer k to a number represented as an array of digits.
    ///
    /// # Intuition
    /// Process from the least significant digit, adding k as a carry.
    /// Each step produces one result digit and the remaining carry.
    ///
    /// # Approach
    /// Iterate from the end of the array, adding k to each digit. Extract
    /// the last digit as the result digit and carry the rest. Continue
    /// while k > 0 even after array exhaustion.
    ///
    /// # Complexity
    /// - Time: O(max(n, log k))
    /// - Space: O(max(n, log k)) for the result
    pub fn add_to_array_form(num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i = num.len() as i32 - 1;

        while i >= 0 || k > 0 {
            if i >= 0 {
                k += num[i as usize];
            }
            result.push(k % 10);
            k /= 10;
            i -= 1;
        }

        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_carry_over() {
        assert_eq!(
            Solution::add_to_array_form(vec![2, 7, 4], 181),
            vec![4, 5, 5]
        );
    }

    #[test]
    fn test_larger_k() {
        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            vec![1, 0, 2, 1]
        );
    }

    #[test]
    fn test_k_is_zero() {
        assert_eq!(Solution::add_to_array_form(vec![0], 0), vec![0]);
    }
}
