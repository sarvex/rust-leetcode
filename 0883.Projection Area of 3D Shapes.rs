pub struct Solution;

impl Solution {
    /// Computes the total projection area of 3D shapes on three planes.
    ///
    /// # Intuition
    /// XY projection counts non-zero cells. YZ projection sums row maximums.
    /// ZX projection sums column maximums.
    ///
    /// # Approach
    /// Iterate the grid to compute all three projections in a single pass
    /// for XY and YZ, and a column-wise pass for ZX.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(1)
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let xy: i32 = grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|v| **v > 0)
            .count() as i32;
        let yz: i32 = grid.iter().map(|row| *row.iter().max().unwrap_or(&0)).sum();
        let zx: i32 = (0..n)
            .map(|j| grid.iter().map(|row| row[j]).max().unwrap_or(0))
            .sum();
        xy + yz + zx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    }

    #[test]
    fn test_with_zeros() {
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    }
}
