use std::collections::HashSet;

impl Solution {
    /// Finds the largest island achievable by flipping at most one 0 to 1.
    ///
    /// # Intuition
    /// Label each connected component with a unique id and record its size.
    /// For each 0-cell, sum the sizes of distinct neighboring components
    /// plus 1 (the flipped cell).
    ///
    /// # Approach
    /// DFS to label all islands and record sizes. Then iterate over 0-cells,
    /// collecting distinct neighbor island ids and summing their sizes.
    /// Track the global maximum.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n^2) for labels and sizes
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut label = vec![vec![0i32; n]; n];
        let mut sizes = vec![0i32; n * n + 1];
        let mut island_id = 0i32;
        let mut max_size = 0;

        fn dfs(
            grid: &[Vec<i32>],
            label: &mut [Vec<i32>],
            sizes: &mut [i32],
            id: i32,
            r: usize,
            c: usize,
            n: usize,
        ) {
            label[r][c] = id;
            sizes[id as usize] += 1;
            for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr < n && nc < n && grid[nr][nc] == 1 && label[nr][nc] == 0 {
                    dfs(grid, label, sizes, id, nr, nc, n);
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && label[i][j] == 0 {
                    island_id += 1;
                    dfs(&grid, &mut label, &mut sizes, island_id, i, j, n);
                    max_size = max_size.max(sizes[island_id as usize]);
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut seen = HashSet::new();
                    let mut total = 1;
                    for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
                        let nr = i.wrapping_add(dr);
                        let nc = j.wrapping_add(dc);
                        if nr < n && nc < n && label[nr][nc] > 0 && seen.insert(label[nr][nc]) {
                            total += sizes[label[nr][nc] as usize];
                        }
                    }
                    max_size = max_size.max(total);
                }
            }
        }

        max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::largest_island(vec![vec![0, 0], vec![0, 0]]), 1);
    }
}
