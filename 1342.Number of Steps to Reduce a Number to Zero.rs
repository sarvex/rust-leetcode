impl Solution {
    /// Bit manipulation step counting.
    ///
    /// # Intuition
    /// Each step either halves (right shift for even) or subtracts one (for odd).
    /// Counting these operations until reaching zero gives the answer.
    ///
    /// # Approach
    /// 1. While n > 0: if even, right shift; if odd, subtract 1
    /// 2. Increment step counter each iteration
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps = 0;
        while num != 0 {
            if num % 2 == 0 {
                num >>= 1;
            } else {
                num -= 1;
            }
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_14() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn example_8() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn zero_steps() {
        assert_eq!(Solution::number_of_steps(0), 0);
    }

    #[test]
    fn one_step() {
        assert_eq!(Solution::number_of_steps(1), 1);
    }
}
