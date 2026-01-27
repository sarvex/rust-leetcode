impl Solution {
    /// Right-to-left carry propagation for incrementing a digit array.
    ///
    /// # Intuition
    /// Adding one to the last digit may cause a carry that propagates
    /// leftward. If no carry remains after any digit, the result is ready.
    /// A full carry-through (e.g., 999 → 1000) requires prepending a 1.
    ///
    /// # Approach
    /// Iterate from the last digit. Increment and check if the result is
    /// less than 10 (no carry). If so, return immediately. Otherwise, set
    /// the digit to 0 and continue. If the loop completes, insert 1 at
    /// the front.
    ///
    /// # Complexity
    /// - Time: O(n) — worst case traverses all digits
    /// - Space: O(1) — modifies in-place (amortized for the insert case)
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            digits[i] += 1;
            if digits[i] < 10 {
                return digits;
            }
            digits[i] = 0;
        }

        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_carry() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn single_carry() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn full_carry() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }
}
