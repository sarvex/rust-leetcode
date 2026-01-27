impl Solution {
    /// Finds the maximum area of an island using DFS flood fill.
    ///
    /// # Intuition
    /// Each unvisited land cell starts a DFS that counts all connected land
    /// cells, zeroing them to mark as visited.
    ///
    /// # Approach
    /// 1. For each cell with value 1, DFS to count the island area.
    /// 2. Mark visited cells by setting them to 0.
    /// 3. Track the maximum area.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n) recursion depth worst case
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            if i >= m || j >= n || grid[i][j] == 0 {
                return 0;
            }
            grid[i][j] = 0;
            let mut area = 1;
            area += dfs(grid, i + 1, j);
            area += dfs(grid, i, j + 1);
            if i > 0 {
                area += dfs(grid, i - 1, j);
            }
            if j > 0 {
                area += dfs(grid, i, j - 1);
            }
            area
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut max_area = 0;
        for i in 0..m {
            for j in 0..n {
                max_area = max_area.max(dfs(&mut grid, i, j));
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);
    }

    #[test]
    fn test_no_island() {
        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0], vec![0, 0]]),
            0
        );
    }
}
