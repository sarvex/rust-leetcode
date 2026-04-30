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

    /// Returns `true` when `a` and `b` were merged, `false` when they already
    /// shared a root (i.e. adding this edge closes a cycle).
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        if self.size[ra] >= self.size[rb] {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        } else {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        }
        true
    }
}

impl Solution {
    /// Union-Find cycle detection on a 2D grid of same-character regions.
    ///
    /// # Intuition
    /// Treat every cell as a node and every edge between same-character
    /// 4-neighbors as an undirected edge. A cycle exists iff adding an edge
    /// unites two endpoints that already share a DSU root. Scanning cells in
    /// row-major order and only joining each cell to its LEFT and TOP same-
    /// character neighbors visits every undirected edge exactly once, so the
    /// first redundant union is exactly the cycle-closing edge.
    ///
    /// # Approach
    /// 1. Flatten 2D coordinates to a 1D DSU: `id = i * n + j`.
    /// 2. For each cell `(i, j)`, attempt to union with:
    ///    - the cell above `(i - 1, j)` when `i > 0` and characters match,
    ///    - the cell to the left `(i, j - 1)` when `j > 0` and characters match.
    /// 3. If any `union` reports the endpoints were already connected, return
    ///    `true` immediately; otherwise `false` after the full scan.
    ///
    /// # Complexity
    /// - Time: O(m · n · α(m · n)) where α is the inverse Ackermann function.
    /// - Space: O(m · n) for the DSU arrays.
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dsu = UnionFind::new(m * n);

        for i in 0..m {
            for j in 0..n {
                let id = i * n + j;
                if i > 0 && grid[i - 1][j] == grid[i][j] && !dsu.union(id - n, id) {
                    return true;
                }
                if j > 0 && grid[i][j - 1] == grid[i][j] && !dsu.union(id - 1, id) {
                    return true;
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

    #[test]
    fn minimal_2x2_cycle() {
        assert!(Solution::contains_cycle(vec![
            vec!['c', 'c'],
            vec!['c', 'c'],
        ]));
    }

    #[test]
    fn single_row_no_cycle() {
        assert!(!Solution::contains_cycle(vec![vec!['a', 'a', 'a', 'a', 'a']]));
    }

    #[test]
    fn single_column_no_cycle() {
        assert!(!Solution::contains_cycle(vec![
            vec!['a'],
            vec!['a'],
            vec!['a'],
            vec!['a'],
        ]));
    }

    #[test]
    fn single_cell_no_cycle() {
        assert!(!Solution::contains_cycle(vec![vec!['z']]));
    }

    #[test]
    fn different_chars_no_cycle() {
        assert!(!Solution::contains_cycle(vec![
            vec!['a', 'b', 'a', 'b'],
            vec!['b', 'a', 'b', 'a'],
            vec!['a', 'b', 'a', 'b'],
            vec!['b', 'a', 'b', 'a'],
        ]));
    }

    #[test]
    fn disjoint_regions_one_with_cycle() {
        assert!(Solution::contains_cycle(vec![
            vec!['a', 'a', 'x', 'x'],
            vec!['a', 'x', 'x', 'x'],
            vec!['x', 'x', 'b', 'b'],
            vec!['x', 'x', 'b', 'b'],
        ]));
    }

    #[test]
    fn large_uniform_grid_has_cycle() {
        let grid = vec![vec!['q'; 50]; 50];
        assert!(Solution::contains_cycle(grid));
    }
}
