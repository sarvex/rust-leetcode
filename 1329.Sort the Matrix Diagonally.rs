impl Solution {
    /// Diagonal extraction, sort, and reassignment.
    ///
    /// # Intuition
    /// Elements on the same diagonal share the same `row - col` offset. Group
    /// elements by this key, sort each group, then place them back. Using
    /// `m - i + j` as the key avoids negative indices.
    ///
    /// # Approach
    /// 1. Group elements by diagonal index `m - i + j`
    /// 2. Sort each diagonal group in descending order (pop from back)
    /// 3. Reassign by popping smallest remaining element per diagonal
    ///
    /// # Complexity
    /// - Time: O(m·n·log(min(m,n))) for sorting diagonals
    /// - Space: O(m·n) for diagonal buckets
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut diags: Vec<Vec<i32>> = vec![vec![]; m + n];

        for i in 0..m {
            for j in 0..n {
                diags[m - i + j].push(mat[i][j]);
            }
        }

        for d in &mut diags {
            d.sort_unstable_by(|a, b| b.cmp(a));
        }

        for i in 0..m {
            for j in 0..n {
                mat[i][j] = diags[m - i + j].pop().unwrap();
            }
        }

        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_four() {
        assert_eq!(
            Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2],]),
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3],]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::diagonal_sort(vec![vec![5]]), vec![vec![5]]);
    }

    #[test]
    fn single_row() {
        assert_eq!(
            Solution::diagonal_sort(vec![vec![3, 1, 2]]),
            vec![vec![3, 1, 2]]
        );
    }
}
