use std::collections::{BTreeSet, VecDeque};

impl Solution {
    /// BFS over reachable cells while skipping visited indices efficiently.
    ///
    /// # Intuition
    /// Each move visits exactly one new cell, so the shortest path is found by BFS. The bottleneck
    /// is enumerating all unvisited cells in a row or column range without scanning the whole
    /// range every time.
    ///
    /// # Approach
    /// Maintain a `BTreeSet` of unvisited columns for every row and unvisited rows for every
    /// column. When expanding `(r, c)`, take all indices in the allowed right or down range from
    /// the corresponding set, remove them from both structures, and enqueue them with distance
    /// `+1`. Each cell is removed once, so total work stays near-linear.
    ///
    /// # Complexity
    /// - Time: O(m * n * log(m * n))
    /// - Space: O(m * n)
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 && n == 1 {
            return 1;
        }

        let mut row_unvisited: Vec<BTreeSet<usize>> = (0..m)
            .map(|_| (0..n).collect::<BTreeSet<usize>>())
            .collect();
        let mut col_unvisited: Vec<BTreeSet<usize>> = (0..n)
            .map(|_| (0..m).collect::<BTreeSet<usize>>())
            .collect();

        let mut distance = vec![vec![-1; n]; m];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        distance[0][0] = 1;
        queue.push_back((0, 0));
        row_unvisited[0].remove(&0);
        col_unvisited[0].remove(&0);

        while let Some((row, col)) = queue.pop_front() {
            let current_dist = distance[row][col];
            if row == m - 1 && col == n - 1 {
                return current_dist;
            }

            let step = grid[row][col] as usize;
            let max_right = (col + step).min(n - 1);
            if col + 1 <= max_right {
                let next_cols: Vec<usize> = row_unvisited[row]
                    .range((col + 1)..=max_right)
                    .copied()
                    .collect();
                for next_col in next_cols {
                    row_unvisited[row].remove(&next_col);
                    col_unvisited[next_col].remove(&row);
                    distance[row][next_col] = current_dist + 1;
                    queue.push_back((row, next_col));
                }
            }

            let max_down = (row + step).min(m - 1);
            if row + 1 <= max_down {
                let next_rows: Vec<usize> = col_unvisited[col]
                    .range((row + 1)..=max_down)
                    .copied()
                    .collect();
                for next_row in next_rows {
                    col_unvisited[col].remove(&next_row);
                    row_unvisited[next_row].remove(&col);
                    distance[next_row][col] = current_dist + 1;
                    queue.push_back((next_row, col));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let grid = vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 3, 1],
            vec![2, 1, 0, 0],
            vec![2, 4, 0, 0],
        ];
        assert_eq!(Solution::minimum_visited_cells(grid), 4);
    }

    #[test]
    fn example_two() {
        let grid = vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 1, 1],
            vec![2, 1, 1, 0],
            vec![3, 4, 1, 0],
        ];
        assert_eq!(Solution::minimum_visited_cells(grid), 3);
    }

    #[test]
    fn example_three() {
        let grid = vec![vec![2, 1, 0], vec![1, 0, 0]];
        assert_eq!(Solution::minimum_visited_cells(grid), -1);
    }

    #[test]
    fn single_cell() {
        let grid = vec![vec![0]];
        assert_eq!(Solution::minimum_visited_cells(grid), 1);
    }

    #[test]
    fn single_row_reachable() {
        let grid = vec![vec![3, 0, 0, 0]];
        assert_eq!(Solution::minimum_visited_cells(grid), 2);
    }

    #[test]
    fn single_column_reachable() {
        let grid = vec![vec![1], vec![1], vec![0]];
        assert_eq!(Solution::minimum_visited_cells(grid), 3);
    }
}
