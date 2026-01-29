impl Solution {
    /// Row-minimum and column-maximum intersection for lucky numbers.
    ///
    /// # Intuition
    /// A lucky number is the minimum in its row and maximum in its column.
    /// Precomputing column maximums allows O(1) validation for each row minimum.
    ///
    /// # Approach
    /// 1. Compute the maximum value per column
    /// 2. For each row, find the minimum and its column index
    /// 3. If that minimum equals the column maximum, it's a lucky number
    ///
    /// # Complexity
    /// - Time: O(m · n) for scanning the matrix
    /// - Space: O(n) for column maximums
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix[0].len();
        let col_max: Vec<i32> = (0..n)
            .map(|j| matrix.iter().map(|row| row[j]).max().unwrap())
            .collect();

        matrix
            .iter()
            .filter_map(|row| {
                let (min_col, &min_val) = row.iter().enumerate().min_by_key(|(_, v)| *v).unwrap();
                if min_val == col_max[min_col] {
                    Some(min_val)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_lucky() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17],]),
            vec![15]
        );
    }

    #[test]
    fn different_lucky() {
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12],
            ]),
            vec![12]
        );
    }

    #[test]
    fn no_lucky() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }
}
