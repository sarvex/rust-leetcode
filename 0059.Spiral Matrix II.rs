impl Solution {
    /// Direction-cycling fill for generating a spiral matrix.
    ///
    /// # Intuition
    /// Filling a matrix in spiral order follows the same directional
    /// pattern as reading one — cycle through right, down, left, up
    /// and turn when hitting boundaries or filled cells.
    ///
    /// # Approach
    /// Initialize an n×n zero matrix. Use a direction array cycling through
    /// four directions. For each value from 1 to n², place it at the current
    /// position. Peek at the next position; if out of bounds or non-zero,
    /// rotate direction. Then advance.
    ///
    /// # Complexity
    /// - Time: O(n^2) — each cell filled once
    /// - Space: O(n^2) — the output matrix
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0i32; n]; n];
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut row, mut col, mut dir) = (0i32, 0i32, 0usize);

        for value in 1..=(n * n) as i32 {
            matrix[row as usize][col as usize] = value;

            let (nr, nc) = (row + directions[dir].0, col + directions[dir].1);
            if nr < 0
                || nr >= n as i32
                || nc < 0
                || nc >= n as i32
                || matrix[nr as usize][nc as usize] != 0
            {
                dir = (dir + 1) % 4;
            }

            row += directions[dir].0;
            col += directions[dir].1;
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn one_by_one() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn two_by_two() {
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
    }
}
