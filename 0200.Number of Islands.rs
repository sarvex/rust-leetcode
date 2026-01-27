impl Solution {
    /// DFS flood-fill for counting connected island components.
    ///
    /// # Intuition
    /// Each contiguous group of '1' cells forms one island. A depth-first
    /// traversal from any unvisited land cell marks all reachable land as
    /// visited, so the number of DFS initiations equals the island count.
    ///
    /// # Approach
    /// Iterate over every cell in the grid. When a '1' is found, increment the
    /// island counter and launch a recursive DFS that marks all connected '1'
    /// cells as '0' (visited). The DFS explores all four cardinal directions,
    /// bounds-checking before each recursive call.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each cell is visited at most once
    /// - Space: O(m × n) — recursion stack in the worst case (all land)
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut [Vec<char>], row: usize, col: usize) {
            grid[row][col] = '0';

            let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

            directions.iter().for_each(|&(dr, dc)| {
                let (nr, nc) = (row as i32 + dr, col as i32 + dc);
                if nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && grid[nr as usize][nc as usize] == '1'
                {
                    dfs(grid, nr as usize, nc as usize);
                }
            });
        }

        let (rows, cols) = (grid.len(), grid.first().map_or(0, Vec::len));

        (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .fold(0, |count, (r, c)| {
                if grid[r][c] == '1' {
                    dfs(&mut grid, r, c);
                    count + 1
                } else {
                    count
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_island() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn three_islands() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn all_water() {
        let grid = vec![vec!['0', '0', '0'], vec!['0', '0', '0']];
        assert_eq!(Solution::num_islands(grid), 0);
    }

    #[test]
    fn all_land() {
        let grid = vec![vec!['1', '1'], vec!['1', '1']];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn single_cell_island() {
        let grid = vec![vec!['1']];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn diagonal_not_connected() {
        let grid = vec![vec!['1', '0'], vec!['0', '1']];
        assert_eq!(Solution::num_islands(grid), 2);
    }
}
