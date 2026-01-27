impl Solution {
    /// Binary search per row to count negatives in a sorted matrix.
    ///
    /// # Intuition
    /// Each row is sorted in non-increasing order. Binary search locates the
    /// first negative element, and all elements from that index onward are
    /// negative.
    ///
    /// # Approach
    /// 1. For each row, binary search for the first negative element
    /// 2. Count `n - first_negative_index` negatives per row
    /// 3. Sum across all rows
    ///
    /// # Complexity
    /// - Time: O(m log n) where m = rows, n = columns
    /// - Space: O(1)
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        grid.iter()
            .map(|row| {
                let pos = row.partition_point(|&x| x >= 0);
                (n - pos) as i32
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_matrix() {
        assert_eq!(
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3],
            ]),
            8
        );
    }

    #[test]
    fn all_positive() {
        assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(
            Solution::count_negatives(vec![vec![-1, -2], vec![-3, -4]]),
            4
        );
    }
}
