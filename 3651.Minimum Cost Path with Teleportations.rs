use std::collections::HashMap;

impl Solution {
    /// Minimum cost path with teleportations using sorted DP.
    ///
    /// # Intuition
    /// Teleporting to cells with value <= current cell's value allows free jumps. By sorting
    /// cells in descending order by value, we can track the best reachable cost for each
    /// threshold and propagate it efficiently across teleport levels.
    ///
    /// # Approach
    /// 1. Initialize base case (t=0) with standard DP: min cost from top-left moving right/down
    /// 2. Sort all cells by value in descending order for threshold processing
    /// 3. For each teleport level t, track running minimum cost from previous level
    /// 4. Use HashMap to store best achievable cost for each cell value threshold
    /// 5. Combine teleport costs with normal movement costs for optimal result
    ///
    /// # Complexity
    /// - Time: O(mnk + mn log(mn)) where m,n are grid dimensions and k is max teleports
    /// - Space: O(mnk) for the DP state array
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let height = grid.len();
        let width = grid[0].len();
        let k = k as usize;

        const INF: i32 = i32::MAX;

        // Preallocate the 3D DP state array: state[teleports][row][col]
        let mut state = vec![vec![vec![INF; width]; height]; k + 1];

        // Preallocate sorted cells array with exact capacity
        let mut sorted_cells = Vec::with_capacity(height * width);

        // Initialize starting position
        state[0][0][0] = 0;
        sorted_cells.push((grid[0][0], 0, 0));

        // Fill first row (can only move right)
        (1..width).for_each(|col| {
            state[0][0][col] = state[0][0][col - 1] + grid[0][col];
            sorted_cells.push((grid[0][col], 0, col));
        });

        // Fill remaining cells with standard DP (no teleports)
        (1..height).for_each(|row| {
            state[0][row][0] = state[0][row - 1][0] + grid[row][0];
            sorted_cells.push((grid[row][0], row, 0));

            (1..width).for_each(|col| {
                // Take minimum of coming from top or left
                state[0][row][col] =
                    (state[0][row - 1][col]).min(state[0][row][col - 1]) + grid[row][col];
                sorted_cells.push((grid[row][col], row, col));
            });
        });

        // Sort cells by value in descending order for threshold processing
        sorted_cells.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        // Process each teleport level
        (1..=k).for_each(|teleports| {
            // HashMap to store best cost for each threshold value
            let mut threshold_costs = HashMap::with_capacity(height * width);

            // Process cells in descending value order, tracking running minimum
            sorted_cells.iter().fold(INF, |min_cost, &(_, row, col)| {
                let new_min = min_cost.min(state[teleports - 1][row][col]);
                // Store the best cost achievable for this threshold
                threshold_costs.insert(grid[row][col], new_min);
                new_min
            });

            // Fill current teleport level's DP table
            (0..height).for_each(|row| {
                (0..width).for_each(|col| {
                    // Start with cost from teleporting to this cell
                    // Safe unwrap: we inserted all grid values above
                    state[teleports][row][col] = *threshold_costs.get(&grid[row][col]).unwrap();

                    // Consider normal movement from top
                    if row > 0 {
                        let from_top = state[teleports][row - 1][col] + grid[row][col];
                        state[teleports][row][col] = state[teleports][row][col].min(from_top);
                    }

                    // Consider normal movement from left
                    if col > 0 {
                        let from_left = state[teleports][row][col - 1] + grid[row][col];
                        state[teleports][row][col] = state[teleports][row][col].min(from_left);
                    }
                });
            });
        });

        // Return minimum cost to reach bottom-right with at most k teleports
        state[k][height - 1][width - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teleportation_shortcut_reduces_cost() {
        let grid = vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]];
        assert_eq!(Solution::min_cost(grid, 2), 7);
    }

    #[test]
    fn test_single_teleport_insufficient_for_full_skip() {
        let grid = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::min_cost(grid, 1), 9);
    }

    #[test]
    fn test_no_teleports_forces_normal_path() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::min_cost(grid, 0), 6);
    }

    #[test]
    fn test_teleport_directly_to_destination() {
        let grid = vec![vec![5, 3, 3], vec![2, 5, 4], vec![4, 3, 1]];
        assert_eq!(Solution::min_cost(grid, 1), 0);
    }

    #[test]
    fn test_excess_teleports_zero_cost_path() {
        let grid = vec![vec![1, 100], vec![100, 1]];
        assert_eq!(Solution::min_cost(grid, 10), 0);
    }

    #[test]
    fn test_single_cell_grid() {
        let grid = vec![vec![42]];
        assert_eq!(Solution::min_cost(grid, 0), 0);
        assert_eq!(Solution::min_cost(grid, 5), 0);
    }

    #[test]
    fn test_single_row_grid() {
        let grid = vec![vec![1, 2, 3, 4]];
        assert_eq!(Solution::min_cost(grid, 0), 9); // 0 + 1 + 2 + 3 + 4 - 1
        assert_eq!(Solution::min_cost(grid, 1), 3); // teleport to value <= 1, then traverse
    }

    #[test]
    fn test_single_column_grid() {
        let grid = vec![vec![1], vec![2], vec![3], vec![4]];
        assert_eq!(Solution::min_cost(grid, 0), 9); // 0 + 1 + 2 + 3 + 4 - 1
        assert_eq!(Solution::min_cost(grid, 1), 3); // teleport to value <= 1, then traverse
    }

    #[test]
    fn test_all_same_values() {
        let grid = vec![vec![5, 5], vec![5, 5]];
        assert_eq!(Solution::min_cost(grid, 0), 15); // 0 + 5 + 5 + 5
        assert_eq!(Solution::min_cost(grid, 1), 0); // can teleport anywhere since all <= 5
    }

    #[test]
    fn test_decreasing_values() {
        let grid = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
        assert_eq!(Solution::min_cost(grid, 0), 20); // 0 + 8 + 7 + 4 + 1
        assert_eq!(Solution::min_cost(grid, 1), 12); // Better with one teleport
        assert_eq!(Solution::min_cost(grid, 2), 0); // Can reach destination with 2 teleports
    }

    #[test]
    fn test_increasing_values() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::min_cost(grid, 0), 20); // 0 + 2 + 3 + 6 + 9
        assert_eq!(Solution::min_cost(grid, 1), 0); // From (0,0) with value 1, can teleport to any cell
    }

    #[test]
    fn test_large_value_differences() {
        let grid = vec![vec![1, 1000], vec![1000, 1]];
        assert_eq!(Solution::min_cost(grid, 0), 1001); // 0 + 1000 + 1
        assert_eq!(Solution::min_cost(grid, 1), 0); // Teleport from start to end
    }
}
