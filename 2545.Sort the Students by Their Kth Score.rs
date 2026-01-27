impl Solution {
    /// Sorts student score rows by the k-th exam score in descending order.
    ///
    /// # Intuition
    /// Simply sort the matrix rows using the k-th column as the key.
    ///
    /// # Approach
    /// Use `sort_unstable_by` with a comparator on column k in reverse order.
    ///
    /// # Complexity
    /// - Time: O(m log m) where m is the number of students
    /// - Space: O(log m) — sort stack
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        score.sort_unstable_by(|a, b| b[k].cmp(&a[k]));
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_column() {
        let score = vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]];
        let expected = vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]];
        assert_eq!(Solution::sort_the_students(score, 2), expected);
    }

    #[test]
    fn test_single_student() {
        let score = vec![vec![1, 2, 3]];
        assert_eq!(Solution::sort_the_students(score.clone(), 0), score);
    }
}
