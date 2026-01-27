impl Solution {
    /// Finds a peak element in a 2D matrix using binary search on rows.
    ///
    /// # Intuition
    /// Binary search on the row index. For each mid row, find the column
    /// maximum. If it is greater than the element below, the peak is in
    /// the upper half; otherwise, in the lower half.
    ///
    /// # Approach
    /// 1. Binary search rows with lo and hi pointers.
    /// 2. For the mid row, find the column index of the maximum value.
    /// 3. Compare with the row below to decide the search direction.
    /// 4. When lo == hi, return the peak position.
    ///
    /// # Complexity
    /// - Time: O(n * log m)
    /// - Space: O(1)
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut lo, mut hi) = (0, mat.len() - 1);

        while lo < hi {
            let mid = (lo + hi) / 2;
            let col = mat[mid]
                .iter()
                .enumerate()
                .max_by_key(|&(_, &val)| val)
                .unwrap()
                .0;
            if mat[mid][col] > mat[mid + 1][col] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        let col = mat[lo]
            .iter()
            .enumerate()
            .max_by_key(|&(_, &val)| val)
            .unwrap()
            .0;

        vec![lo as i32, col as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::find_peak_grid(vec![vec![1, 4], vec![3, 2]]);
        let (r, c) = (result[0] as usize, result[1] as usize);
        let mat = vec![vec![1, 4], vec![3, 2]];
        let val = mat[r][c];
        assert!(r == 0 || val > mat[r - 1][c]);
        assert!(r == mat.len() - 1 || val > mat[r + 1][c]);
    }
}
