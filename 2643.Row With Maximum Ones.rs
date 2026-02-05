impl Solution {
    /// Find the row with the maximum number of ones.
    ///
    /// # Intuition
    /// Scan each row, count ones, and track the best row index and count.
    ///
    /// # Approach
    /// 1. Enumerate rows
    /// 2. Count ones per row
    /// 3. Track the row with the highest count (earliest index on tie)
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(1)
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        mat.iter()
            .enumerate()
            .map(|(i, row)| (i as i32, row.iter().filter(|&&v| v == 1).count() as i32))
            .fold(vec![0, 0], |best, (idx, cnt)| {
                if cnt > best[1] { vec![idx, cnt] } else { best }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
            vec![0, 1]
        );
    }

    #[test]
    fn test_second_row() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![0, 0]]),
            vec![0, 0]
        );
    }
}
