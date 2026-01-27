impl Solution {
    /// Simulates ball movement through a grid of diagonal boards.
    ///
    /// # Intuition
    /// Each ball falls row by row. A board value of 1 pushes right, -1 pushes left.
    /// A ball gets stuck if it hits a wall or if adjacent boards form a V-shape.
    ///
    /// # Approach
    /// 1. For each starting column, simulate the ball dropping through each row.
    /// 2. At each cell, check the direction and the adjacent cell to detect blocking.
    /// 3. If the ball exits the bottom, record the column; otherwise record -1.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(n) for the result
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len();
        (0..n)
            .map(|start| {
                let mut col = start as i32;
                for row in &grid {
                    let c = col as usize;
                    let dir = row[c];
                    let next = col + dir;
                    if next < 0 || next >= n as i32 || row[next as usize] != dir {
                        return -1;
                    }
                    col = next;
                }
                col
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let grid = vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1],
        ];
        assert_eq!(Solution::find_ball(grid), vec![1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_all_right() {
        let grid = vec![vec![1, 1, 1, 1]];
        assert_eq!(Solution::find_ball(grid), vec![1, 2, 3, -1]);
    }

    #[test]
    fn test_single_column() {
        assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1]);
    }
}
