impl Solution {
    /// Count subtraction operations until one number becomes zero.
    ///
    /// # Intuition
    /// Repeatedly subtract the smaller from the larger, counting each step.
    /// This mirrors the Euclidean GCD algorithm with unit-step subtraction.
    ///
    /// # Approach
    /// Loop while both are non-zero, subtracting the smaller from the larger,
    /// incrementing the operation count each iteration.
    ///
    /// # Complexity
    /// - Time: O(max(num1, num2)) worst case
    /// - Space: O(1)
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut ops = 0;
        while num1 != 0 && num2 != 0 {
            if num1 >= num2 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            ops += 1;
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::count_operations(10, 10), 1);
    }

    #[test]
    fn one_is_zero() {
        assert_eq!(Solution::count_operations(0, 5), 0);
    }

    #[test]
    fn example_case() {
        assert_eq!(Solution::count_operations(2, 3), 3);
    }
}
