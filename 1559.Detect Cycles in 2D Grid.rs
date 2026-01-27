impl Solution {
    /// BFS cycle detection in a 2D grid of same-character regions.
    ///
    /// # Intuition
    /// A cycle exists when BFS from a cell revisits an already-visited cell
    /// that is not the immediate parent. Track visited cells globally and
    /// parent coordinates per BFS step to distinguish back-edges from
    /// parent edges.
    ///
    /// # Approach
    /// 1. For each unvisited cell, start BFS
    /// 2. Expand to 4-directional neighbors with the same character
    /// 3. Skip the parent cell; if the neighbor is already visited, a cycle exists
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n) for the visited matrix
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let dirs: [isize; 5] = [-1, 0, 1, 0, -1];

        for i in 0..m {
            for j in 0..n {
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;
                let mut queue = vec![(i as isize, j as isize, -1isize, -1isize)];

                while let Some((x, y, px, py)) = queue.pop() {
                    for k in 0..4 {
                        let (nx, ny) = (x + dirs[k], y + dirs[k + 1]);
                        if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize {
                            continue;
                        }
                        let (ux, uy) = (nx as usize, ny as usize);
                        if grid[ux][uy] != grid[x as usize][y as usize] || (nx == px && ny == py) {
                            continue;
                        }
                        if visited[ux][uy] {
                            return true;
                        }
                        visited[ux][uy] = true;
                        queue.push((nx, ny, x, y));
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_cycle() {
        assert!(Solution::contains_cycle(vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a'],
        ]));
    }

    #[test]
    fn no_cycle() {
        assert!(!Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a'],
        ]));
    }
}
