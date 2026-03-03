impl Solution {
    /// Greedy bit-by-bit selection minimizing the bitwise OR.
    ///
    /// # Intuition
    /// Since OR only sets bits, we greedily avoid setting high-order bits.
    /// Instead of filtering candidate vectors, we accumulate a `must_be_zero`
    /// mask that tracks which bits we have successfully excluded. Each check
    /// scans the original grid with zero allocations.
    ///
    /// # Approach
    /// 1. Process bits from bit 16 down to bit 0.
    /// 2. For each bit, tentatively add it to `must_be_zero`.
    /// 3. Check if every row has an element with none of the `must_be_zero`
    ///    bits set — a single pass over the original grid.
    /// 4. If yes, commit: that bit stays off. If no, it must be in the result.
    ///
    /// # Complexity
    /// - Time: O(B × m × n) where B = 17 (number of bits for values ≤ 10⁵)
    /// - Space: O(1) auxiliary — reads the input grid without copying
    pub fn minimum_or(grid: Vec<Vec<i32>>) -> i32 {
        let mut must_be_zero = 0;
        let mut result = 0;

        for bit in (0..17).rev() {
            let tentative = must_be_zero | (1 << bit);
            let can_avoid = grid
                .iter()
                .all(|row| row.iter().any(|val| *val & tentative == 0));

            if can_avoid {
                must_be_zero = tentative;
            } else {
                result |= 1 << bit;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_or(vec![vec![1, 5], vec![2, 4]]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_or(vec![vec![3, 5], vec![6, 4]]), 5);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimum_or(vec![vec![7, 9, 8]]), 7);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::minimum_or(vec![vec![1]]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(
            Solution::minimum_or(vec![vec![3, 3], vec![3, 3], vec![3, 3]]),
            3
        );
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(
            Solution::minimum_or(vec![vec![1, 2], vec![4, 8], vec![16, 32]]),
            21
        );
    }

    #[test]
    fn test_single_column() {
        assert_eq!(Solution::minimum_or(vec![vec![5], vec![3], vec![6]]), 7);
    }
}
