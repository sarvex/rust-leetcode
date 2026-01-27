impl Solution {
    /// Direction-cycling traversal for spiral matrix order.
    ///
    /// # Intuition
    /// A spiral visits elements by cycling through four directions
    /// (right, down, left, up), turning when hitting a boundary or
    /// a previously visited cell.
    ///
    /// # Approach
    /// Maintain a visited matrix and a direction index cycling through
    /// `[right, down, left, up]`. At each step, record the current
    /// element and mark it visited. Peek at the next position; if it is
    /// out of bounds or visited, rotate the direction. Advance to the
    /// new position.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each element visited once
    /// - Space: O(m × n) — visited matrix
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut visited = vec![vec![false; n]; m];
        let mut result = Vec::with_capacity(m * n);
        let (mut row, mut col, mut dir) = (0i32, 0i32, 0usize);

        for _ in 0..m * n {
            result.push(matrix[row as usize][col as usize]);
            visited[row as usize][col as usize] = true;

            let (nr, nc) = (row + directions[dir].0, col + directions[dir].1);
            if nr < 0
                || nr >= m as i32
                || nc < 0
                || nc >= n as i32
                || visited[nr as usize][nc as usize]
            {
                dir = (dir + 1) % 4;
            }

            row += directions[dir].0;
            col += directions[dir].1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn three_by_four() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
    }
}
