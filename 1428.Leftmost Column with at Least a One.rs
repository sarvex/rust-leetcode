// This is the BinaryMatrix's API interface.
// You should not implement it, or speculate about its implementation
// struct BinaryMatrix;
// impl BinaryMatrix {
//     fn get(row: i32, col: i32) -> i32;
//     fn dimensions() -> Vec<i32>;
// }

impl Solution {
    /// Binary search per row to find leftmost column with a one.
    ///
    /// # Intuition
    /// Each row is sorted (0s then 1s). Binary searching each row for the
    /// first 1 and tracking the global minimum column index finds the answer.
    ///
    /// # Approach
    /// 1. Get matrix dimensions
    /// 2. For each row, binary search for the leftmost 1
    /// 3. Track the minimum column index across all rows
    /// 4. Return -1 if no 1 exists
    ///
    /// # Complexity
    /// - Time: O(m log n)
    /// - Space: O(1)
    pub fn left_most_column_with_one(binary_matrix: &BinaryMatrix) -> i32 {
        let dims = binary_matrix.dimensions();
        let (m, n) = (dims[0] as usize, dims[1] as usize);
        let mut result = n;

        for i in 0..m {
            let (mut lo, mut hi) = (0, n);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if binary_matrix.get(i as i32, mid as i32) == 1 {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            result = result.min(lo);
        }

        if result >= n {
            -1
        } else {
            result as i32
        }
    }
}
