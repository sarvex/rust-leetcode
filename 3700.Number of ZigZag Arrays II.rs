const MODULO: i64 = 1_000_000_007;

impl Solution {
    /// Counts valid ZigZag arrays using k×k matrix exponentiation.
    ///
    /// # Intuition
    /// Fold the symmetric inc/dec state into a single k-element vector by observing
    /// that `sum(inc) == sum(dec)` at every step. The transition for "increasing"
    /// arrivals at position `i` is a suffix sum: count arrays where the previous
    /// value `j` satisfies `i + j >= k`. This yields a k×k transition matrix
    /// (half the size of the 2k approach), and the final answer doubles the sum.
    ///
    /// # Approach
    /// 1. Build the k×k matrix where `M[i][j] = 1` if `i + j >= k`
    /// 2. Raise it to the `(n-1)`-th power via fast exponentiation
    /// 3. Sum all entries and double (inc == dec symmetry)
    ///
    /// The multiplication uses column-major access on the right operand, which is
    /// cache-friendly because the matrix is symmetric (`M = M^T`).
    ///
    /// # Complexity
    /// - Time: O(k³ log n) where k = upper - lower + 1
    /// - Space: O(k²)
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let k = (r - l + 1) as usize;
        let mut matrix = Self::build_matrix(k);
        Self::mat_pow(&mut matrix, n - 1, k);

        let total: i64 = matrix
            .iter()
            .flat_map(|row| row.iter())
            .fold(0, |acc, &v| (acc + v) % MODULO);

        ((total * 2) % MODULO) as i32
    }

    /// Builds the k×k transition matrix where `M[i][j] = 1` iff `i + j >= k`.
    fn build_matrix(k: usize) -> Vec<Vec<i64>> {
        (0..k)
            .map(|i| (0..k).map(|j| i64::from(i + j >= k)).collect())
            .collect()
    }

    /// Raises `matrix` to the `exp`-th power in-place using repeated squaring.
    fn mat_pow(matrix: &mut Vec<Vec<i64>>, mut exp: i32, k: usize) {
        let mut result = Self::identity(k);
        while exp > 0 {
            if exp & 1 == 1 {
                Self::mat_mul(&mut result, matrix, k);
            }
            Self::mat_sqr(matrix, k);
            exp >>= 1;
        }
        *matrix = result;
    }

    /// Computes `a = a * b` using column-major access on `b` (cache-friendly since `b` is symmetric).
    fn mat_mul(a: &mut Vec<Vec<i64>>, b: &[Vec<i64>], k: usize) {
        let mut result = vec![vec![0i64; k]; k];
        for i in 0..k {
            for j in 0..k {
                if a[i][j] == 0 {
                    continue;
                }
                for p in 0..k {
                    result[i][p] = (result[i][p] + a[i][j] * b[p][j]) % MODULO;
                }
            }
        }
        *a = result;
    }

    /// Computes `a = a * a` (squaring), reusing the same column-major trick.
    fn mat_sqr(a: &mut Vec<Vec<i64>>, k: usize) {
        let mut result = vec![vec![0i64; k]; k];
        for i in 0..k {
            for j in 0..k {
                if a[i][j] == 0 {
                    continue;
                }
                for p in 0..k {
                    result[i][p] = (result[i][p] + a[i][j] * a[p][j]) % MODULO;
                }
            }
        }
        *a = result;
    }

    fn identity(k: usize) -> Vec<Vec<i64>> {
        (0..k)
            .map(|i| (0..k).map(|j| i64::from(i == j)).collect())
            .collect()
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
    fn test_length_one() {
        assert_eq!(Solution::zig_zag_arrays(1, 1, 5), 10);
    }

    #[test]
    fn test_large_n_matrix_exponentiation() {
        let result = Solution::zig_zag_arrays(1_000_000_000, 1, 75);
        assert!(result >= 0);
    }
}
