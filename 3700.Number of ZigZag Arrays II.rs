const MODULO: i64 = 1_000_000_007;

impl Solution {
    /// Counts valid ZigZag arrays using matrix exponentiation with band optimization.
    ///
    /// # Intuition
    /// The transition matrix has a symmetric band structure where the first half mirrors
    /// the second half (reversed). This allows skipping zero regions during multiplication.
    ///
    /// # Approach
    /// 1. Build symmetric band matrix of size 2*(r-l)
    /// 2. Use sparse matrix multiplication that skips zero bands
    /// 3. Apply matrix exponentiation for n-1 transitions
    /// 4. Sum all entries in the resulting matrix
    ///
    /// # Complexity
    /// - Time: O(k³ log n) with significant constant factor reduction from band skipping
    /// - Space: O(k²) for the transition matrix
    pub fn zig_zag_arrays(length: i32, lower: i32, upper: i32) -> i32 {
        let transition_matrix = Self::build_transition_matrix((upper - lower) as usize * 2);
        Self::matrix_power(&transition_matrix, length - 1)
            .iter()
            .flat_map(|row: &Vec<i64>| row.iter())
            .fold(0, |acc, &val| (acc + val) % MODULO) as i32
    }

    fn build_transition_matrix(size: usize) -> Vec<Vec<i64>> {
        let half_size = size / 2;
        let mut matrix: Vec<Vec<i64>> = (0..half_size)
            .map(|row_idx| {
                (0..size)
                    .map(|col_idx| {
                        i64::from(col_idx >= half_size && col_idx <= half_size + row_idx)
                    })
                    .collect()
            })
            .collect();

        (0..half_size).rev().for_each(|row_idx| {
            let mut mirrored_row = matrix[row_idx].clone();
            mirrored_row.reverse();
            matrix.push(mirrored_row);
        });

        matrix
    }

    fn matrix_power(matrix: &[Vec<i64>], exponent: i32) -> Vec<Vec<i64>> {
        match exponent {
            0 => Self::identity_matrix(matrix.len()),
            exp if exp % 2 == 1 => {
                Self::matrix_multiply(matrix, &Self::matrix_power(matrix, exp - 1))
            }
            exp => Self::matrix_power(&Self::matrix_multiply(matrix, matrix), exp / 2),
        }
    }

    fn identity_matrix(size: usize) -> Vec<Vec<i64>> {
        (0..size)
            .map(|row| (0..size).map(|col| i64::from(row == col)).collect())
            .collect()
    }

    fn matrix_multiply(left: &[Vec<i64>], right: &[Vec<i64>]) -> Vec<Vec<i64>> {
        let size = left.len();
        let mut product = vec![vec![0i64; size]; size];

        for inner in 0..size {
            let band_start = right[inner]
                .iter()
                .position(|&val| val != 0)
                .unwrap_or(size);
            let band_end = right[inner]
                .iter()
                .rposition(|&val| val != 0)
                .map_or(0, |pos| pos + 1);

            for row in 0..size {
                if left[row][inner] == 0 {
                    continue;
                }
                for col in band_start..band_end {
                    product[row][col] =
                        (product[row][col] + left[row][inner] * right[inner][col]) % MODULO;
                }
            }
        }

        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_range_two_values() {
        assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
    }

    #[test]
    fn test_three_values() {
        assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
    }

    #[test]
    fn test_length_four_small_range() {
        assert_eq!(Solution::zig_zag_arrays(4, 1, 2), 2);
    }

    #[test]
    fn test_large_n_matrix_exponentiation() {
        let result = Solution::zig_zag_arrays(1000000000, 1, 75);
        assert!(result >= 0);
    }
}
