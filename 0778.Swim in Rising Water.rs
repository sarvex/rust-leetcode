impl Solution {
    /// Finds minimum time to swim from top-left to bottom-right using Union-Find.
    ///
    /// # Intuition
    /// At time t, all cells with elevation <= t are accessible. Incrementally
    /// union adjacent accessible cells until top-left and bottom-right are connected.
    ///
    /// # Approach
    /// Sort cells by elevation. Process cells in increasing elevation order,
    /// unioning each newly accessible cell with its already-accessible neighbors.
    /// Return the elevation when source and sink become connected.
    ///
    /// # Complexity
    /// - Time: O(n^2 * alpha(n^2)) effectively O(n^2)
    /// - Space: O(n^2) for the union-find structure
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let total = n * n;
        let mut parent: Vec<usize> = (0..total).collect();

        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut [usize], x: usize, y: usize) {
            let px = find(parent, x);
            let py = find(parent, y);
            parent[px] = py;
        }

        let mut order: Vec<(i32, usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, v)| (*v, i, j)))
            .collect();
        order.sort_unstable();

        let mut accessible = vec![vec![false; n]; n];
        for &(elevation, r, c) in &order {
            accessible[r][c] = true;
            for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr < n && nc < n && accessible[nr][nc] {
                    union(&mut parent, r * n + c, nr * n + nc);
                }
            }
            if find(&mut parent, 0) == find(&mut parent, total - 1) {
                return elevation;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let grid = vec![vec![0, 2], vec![1, 3]];
        assert_eq!(Solution::swim_in_water(grid), 3);
    }

    #[test]
    fn test_larger() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(Solution::swim_in_water(grid), 16);
    }
}
