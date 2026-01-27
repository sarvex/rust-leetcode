impl Solution {
    /// Minimize the sum of two numbers formed by splitting a four-digit number's digits.
    ///
    /// # Intuition
    /// Sorting digits and placing the two smallest in the tens places minimizes the total.
    ///
    /// # Approach
    /// 1. Extract all four digits and sort them.
    /// 2. Pair the two smallest as tens digits and the two largest as units digits.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut digits = [0; 4];
        for d in &mut digits {
            *d = num % 10;
            num /= 10;
        }
        digits.sort_unstable();
        10 * (digits[0] + digits[1]) + digits[2] + digits[3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2932() {
        assert_eq!(Solution::minimum_sum(2932), 52);
    }

    #[test]
    fn example_4009() {
        assert_eq!(Solution::minimum_sum(4009), 13);
    }

    #[test]
    fn all_same_digits() {
        assert_eq!(Solution::minimum_sum(1111), 22);
    }
}
