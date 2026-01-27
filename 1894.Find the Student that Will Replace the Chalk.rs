impl Solution {
    /// Finds the student who runs out of chalk using modular arithmetic.
    ///
    /// # Intuition
    /// After full rounds, only the remainder matters. Scan once to find
    /// the student whose chalk usage exceeds the remaining amount.
    ///
    /// # Approach
    /// 1. Compute the total chalk per round using i64 to avoid overflow.
    /// 2. Reduce k modulo the total.
    /// 3. Iterate to find the first student whose usage exceeds the remainder.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let total: i64 = chalk.iter().map(|&x| x as i64).sum();
        let mut remaining = (k as i64) % total;

        for (i, &usage) in chalk.iter().enumerate() {
            if remaining < usage as i64 {
                return i as i32;
            }
            remaining -= usage as i64;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
    }

    #[test]
    fn test_mid_student() {
        assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
    }

    #[test]
    fn test_single_student() {
        assert_eq!(Solution::chalk_replacer(vec![1], 0), 0);
    }
}
