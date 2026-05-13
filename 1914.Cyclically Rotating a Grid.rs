impl Solution {
    /// Layer-by-layer cyclic rotation using index-offset placement.
    ///
    /// # Intuition
    /// The grid is composed of concentric rectangular layers. Each layer is a
    /// closed ring of elements. A counter-clockwise rotation by k steps is
    /// equivalent to reading the ring starting at offset k (mod perimeter).
    /// We exploit this to write directly into the result in a single pass per
    /// layer, avoiding any in-place shifting.
    ///
    /// # Approach
    /// 1. Determine the number of layers: `min(m, n) / 2`.
    /// 2. For each layer, compute `top`, `bottom`, `left`, `right` bounds and
    ///    the perimeter `p = 2*(width + height - 2)`.
    /// 3. Extract the ring into a pre-allocated `Vec<i32>` of capacity `p`.
    /// 4. Compute `shift = k % p`. Element at ring index `i` maps to result
    ///    index `(i + shift) % p` — but since we write back in traversal order,
    ///    we read from `(i + p - shift) % p` instead (equivalent to a left
    ///    rotation by `shift`).
    /// 5. Write back using the same four-segment traversal, reading from the
    ///    offset-adjusted ring with a single modulo per element.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each cell is read once and written once
    /// - Space: O(max(m, n)) — one ring buffer reused across layers
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = grid;
        let layers = m.min(n) / 2;

        for layer in 0..layers {
            let top = layer;
            let bottom = m - 1 - layer;
            let left = layer;
            let right = n - 1 - layer;
            let width = right - left + 1;
            let height = bottom - top + 1;
            let perimeter = 2 * (width + height - 2);

            // Pre-allocate ring buffer — no reallocation during push
            let mut ring = Vec::with_capacity(perimeter);

            // Extract ring in counter-clockwise order
            for col in left..=right {
                ring.push(result[top][col]);
            }
            for row in (top + 1)..=bottom {
                ring.push(result[row][right]);
            }
            for col in (left..right).rev() {
                ring.push(result[bottom][col]);
            }
            for row in (top + 1..bottom).rev() {
                ring.push(result[row][left]);
            }

            // Effective shift: rotating counter-clockwise by k steps
            let shift = (k as usize) % perimeter;
            // Reading from `(i + shift) % p` for write position i is equivalent
            // to reading ring[(i + shift) % p] — precompute once.
            let read = |i: usize| ring[(i + shift) % perimeter];

            // Write back with offset — same traversal, single modulo per cell
            let mut idx = 0usize;
            for col in left..=right {
                result[top][col] = read(idx);
                idx += 1;
            }
            for row in (top + 1)..=bottom {
                result[row][right] = read(idx);
                idx += 1;
            }
            for col in (left..right).rev() {
                result[bottom][col] = read(idx);
                idx += 1;
            }
            for row in (top + 1..bottom).rev() {
                result[row][left] = read(idx);
                idx += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![40, 10], vec![30, 20]];
        assert_eq!(
            Solution::rotate_grid(grid, 1),
            vec![vec![10, 20], vec![40, 30]]
        );
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(
            Solution::rotate_grid(grid, 2),
            vec![
                vec![3, 4, 8, 12],
                vec![2, 11, 10, 16],
                vec![1, 7, 6, 15],
                vec![5, 9, 13, 14],
            ]
        );
    }

    #[test]
    fn test_full_cycle_is_identity() {
        // k equal to perimeter (4) must return the original grid
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::rotate_grid(grid.clone(), 4), grid);
    }

    #[test]
    fn test_large_k() {
        // k = 1_000_000_001, perimeter = 4, effective shift = 1
        let grid = vec![vec![10, 20], vec![30, 40]];
        assert_eq!(
            Solution::rotate_grid(grid, 1_000_000_001),
            vec![vec![20, 40], vec![10, 30]]
        );
    }

    #[test]
    fn test_multiple_layers() {
        // 4×6 grid, 2 layers, k=1
        let grid = vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18],
            vec![19, 20, 21, 22, 23, 24],
        ];
        assert_eq!(
            Solution::rotate_grid(grid, 1),
            vec![
                vec![2, 3, 4, 5, 6, 12],
                vec![1, 9, 10, 11, 17, 18],
                vec![7, 8, 14, 15, 16, 24],
                vec![13, 19, 20, 21, 22, 23],
            ]
        );
    }

    #[test]
    fn test_single_layer_wide() {
        // 2×3 grid has only one layer (perimeter = 6), k=2
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            Solution::rotate_grid(grid, 2),
            vec![vec![3, 6, 5], vec![2, 1, 4]]
        );
    }

    #[test]
    fn test_max_constraints() {
        // 50×50 grid filled with 1s — result must also be all 1s
        let grid = vec![vec![1i32; 50]; 50];
        let result = Solution::rotate_grid(grid, 1_000_000_000);
        assert!(result.iter().all(|row| row.iter().all(|&v| v == 1)));
    }
}
