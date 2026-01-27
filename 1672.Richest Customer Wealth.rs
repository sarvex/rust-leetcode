impl Solution {
    /// Maximum row sum for richest customer.
    ///
    /// # Intuition
    /// Each customer's wealth is the sum of their bank accounts (row sum).
    /// The richest customer has the maximum row sum.
    ///
    /// # Approach
    /// 1. Map each row to its sum
    /// 2. Return the maximum
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(1)
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|row| row.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_wealth() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }

    #[test]
    fn different_sizes() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }
}
