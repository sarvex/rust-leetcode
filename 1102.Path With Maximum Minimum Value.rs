struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        parent.extend(0..n);
        Self {
            parent,
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (pa, pb) = (self.find(a), self.find(b));
        if pa != pb {
            if self.size[pa] >= self.size[pb] {
                self.parent[pb] = pa;
                self.size[pa] += self.size[pb];
            } else {
                self.parent[pa] = pb;
                self.size[pb] += self.size[pa];
            }
        }
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

impl Solution {
    /// Finds the path maximizing the minimum cell value using sorted Union-Find.
    ///
    /// # Intuition
    /// Process cells in descending value order. Union adjacent visited cells.
    /// The answer is the value of the cell that first connects source to target.
    ///
    /// # Approach
    /// Sort all cells by value descending. Iterate, marking cells as visited
    /// and unioning with visited neighbors. Return the value when (0,0) and
    /// (m-1,n-1) become connected.
    ///
    /// # Complexity
    /// - Time: O(m*n * log(m*n))
    /// - Space: O(m*n)
    pub fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut uf = UnionFind::new(m * n);
        let mut cells = Vec::with_capacity(m * n);
        cells.extend(
            grid.iter()
                .enumerate()
                .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, v)| (*v, i, j))),
        );
        cells.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut visited = vec![vec![false; n]; m];
        for (val, r, c) in &cells {
            visited[r][c] = true;
            for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr < m && nc < n && visited[nr][nc] {
                    uf.union(r * n + c, nr * n + nc);
                }
            }
            if uf.connected(0, m * n - 1) {
                return *val;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]];
        assert_eq!(Solution::maximum_minimum_path(grid), 4);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![2, 2, 1, 2, 2, 2], vec![1, 2, 2, 2, 1, 2]];
        assert_eq!(Solution::maximum_minimum_path(grid), 2);
    }

    #[test]
    fn test_single_cell() {
        let grid = vec![vec![5]];
        assert_eq!(Solution::maximum_minimum_path(grid), 5);
    }

    #[test]
    fn test_two_cells_horizontal() {
        let grid = vec![vec![3, 7]];
        assert_eq!(Solution::maximum_minimum_path(grid), 3);
    }

    #[test]
    fn test_two_cells_vertical() {
        let grid = vec![vec![3], vec![7]];
        assert_eq!(Solution::maximum_minimum_path(grid), 3);
    }
}
