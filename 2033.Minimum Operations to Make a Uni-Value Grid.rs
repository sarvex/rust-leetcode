impl Solution {
    /// Flatten, check feasibility via modulo, then minimize |a_i - median| / x.
    ///
    /// # Intuition
    /// Each operation changes a value by `+x` or `-x`, so every element's
    /// residue modulo `x` is invariant. A uni-value grid is only reachable when
    /// all elements share the same residue. Once that holds, the task becomes:
    /// pick a target integer `t` minimizing sum of `|a_i - t|`. The classical
    /// answer is the **median** of the sequence.
    ///
    /// # Approach
    /// 1. Flatten the grid with preallocation (`m * n` known up front).
    /// 2. Let `r = flat[0] % x`. Since all values are non-negative per the
    ///    constraints, plain `%` matches `rem_euclid` and is faster. Early-exit
    ///    inside the same pass used to copy: fail fast on mismatch.
    /// 3. Use `select_nth_unstable` to locate the median in **O(n)** via
    ///    quickselect - strictly faster than full `sort_unstable` (O(n log n)).
    /// 4. Sum `(a - median).abs() as u64 / x as u64` - using `u64` avoids
    ///    overflow concerns without per-element `i32 -> i64` casts branching.
    ///
    /// # Complexity
    /// - Time: O(n) expected where n = m * rows.
    /// - Space: O(n) for the flattened vector (unavoidable - we need to mutate
    ///   to find the median without allocating twice).
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let total = grid.iter().map(Vec::len).sum::<usize>();
        let mut flat: Vec<i32> = Vec::with_capacity(total);
        for row in &grid {
            flat.extend_from_slice(row);
        }

        let remainder = flat[0] % x;
        for &v in &flat {
            if v % x != remainder {
                return -1;
            }
        }

        // O(n) median via quickselect - strictly faster than sort_unstable.
        let mid = flat.len() / 2;
        let (_, median, _) = flat.select_nth_unstable(mid);
        let median = *median;

        let x64 = x as i64;
        let sum: i64 = flat
            .iter()
            .map(|&v| (v - median).unsigned_abs() as i64)
            .sum();
        (sum / x64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let grid = vec![vec![2, 4], vec![6, 8]];
        assert_eq!(Solution::min_operations(grid, 2), 4);
    }

    #[test]
    fn test_example_two() {
        let grid = vec![vec![1, 5], vec![2, 3]];
        assert_eq!(Solution::min_operations(grid, 1), 5);
    }

    #[test]
    fn test_example_three_impossible() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::min_operations(grid, 2), -1);
    }

    #[test]
    fn test_single_cell() {
        let grid = vec![vec![7]];
        assert_eq!(Solution::min_operations(grid, 3), 0);
    }

    #[test]
    fn test_already_uniform() {
        let grid = vec![vec![5, 5, 5], vec![5, 5, 5]];
        assert_eq!(Solution::min_operations(grid, 4), 0);
    }

    #[test]
    fn test_x_equals_one_arbitrary_values() {
        // With x = 1 every integer shares the same residue; answer is sum of
        // |a - median|.
        let grid = vec![vec![1, 10, 2, 9], vec![100, 3, 4, 5]];
        // flat sorted: [1,2,3,4,5,9,10,100], median = flat[4] = 5.
        // distances: 4+3+2+1+0+4+5+95 = 114.
        assert_eq!(Solution::min_operations(grid, 1), 114);
    }

    #[test]
    fn test_large_x_feasible() {
        // All values share residue 3 mod 100.
        let grid = vec![vec![3, 103], vec![203, 303]];
        // flat sorted: [3,103,203,303], median = flat[2] = 203.
        // ops: 2 + 1 + 0 + 1 = 4.
        assert_eq!(Solution::min_operations(grid, 100), 4);
    }

    #[test]
    fn test_large_x_infeasible() {
        let grid = vec![vec![3, 103], vec![204, 303]];
        assert_eq!(Solution::min_operations(grid, 100), -1);
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![1, 3, 5, 7, 9]];
        // residues all 1 mod 2, median = 5, ops = (4+2+0+2+4)/2 = 6.
        assert_eq!(Solution::min_operations(grid, 2), 6);
    }

    #[test]
    fn test_single_column() {
        let grid = vec![vec![10], vec![20], vec![30], vec![40]];
        // sorted: [10,20,30,40], median = flat[2] = 30.
        // ops: (20+10+0+10)/10 = 4.
        assert_eq!(Solution::min_operations(grid, 10), 4);
    }

    #[test]
    fn test_max_values_feasible() {
        let grid = vec![vec![10_000, 10_000], vec![10_000, 10_000]];
        assert_eq!(Solution::min_operations(grid, 1), 0);
    }
}
