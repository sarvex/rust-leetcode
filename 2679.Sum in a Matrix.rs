impl Solution {
    /// Score is sum of column-wise maximums after sorting each row.
    ///
    /// # Intuition
    /// After sorting each row, the j-th column represents the j-th smallest
    /// value per row. The score takes the max from each column.
    ///
    /// # Approach
    /// 1. Sort each row
    /// 2. For each column, find the maximum across all rows
    /// 3. Sum these column maximums
    ///
    /// # Complexity
    /// - Time: O(m * n log n)
    /// - Space: O(1) auxiliary (in-place sort)
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        for row in &mut nums {
            row.sort_unstable();
        }

        (0..nums[0].len())
            .map(|col| nums.iter().map(|row| row[col]).max().unwrap_or(0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::matrix_sum(vec![
                vec![7, 2, 1],
                vec![6, 4, 2],
                vec![6, 5, 3],
                vec![3, 2, 1]
            ]),
            15
        );
    }

    #[test]
    fn test_single_row() {
        assert_eq!(Solution::matrix_sum(vec![vec![1]]), 1);
    }
}
