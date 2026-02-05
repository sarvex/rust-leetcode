use std::collections::{HashMap, HashSet};

impl Solution {
    /// Finds the bench with the most distinct students.
    ///
    /// # Intuition
    /// Group students by bench, count unique student IDs per bench, and return
    /// the maximum.
    ///
    /// # Approach
    /// 1. Build a map from bench_id to a set of student IDs.
    /// 2. Return the maximum set size across all benches.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of entries
    /// - Space: O(n) for the hash map and sets
    pub fn max_students_on_bench(students: Vec<Vec<i32>>) -> i32 {
        students
            .iter()
            .fold(
                HashMap::<i32, HashSet<i32>>::new(),
                |mut bench_students, entry| {
                    bench_students.entry(entry[1]).or_default().insert(entry[0]);
                    bench_students
                },
            )
            .values()
            .map(|s| s.len() as i32)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_benches_returns_max() {
        let students = vec![vec![1, 1], vec![2, 1], vec![3, 2], vec![1, 2], vec![2, 2]];
        assert_eq!(Solution::max_students_on_bench(students), 3);
    }

    #[test]
    fn single_bench_single_student() {
        assert_eq!(Solution::max_students_on_bench(vec![vec![1, 1]]), 1);
    }

    #[test]
    fn duplicate_student_on_same_bench_not_double_counted() {
        let students = vec![vec![1, 1], vec![1, 1], vec![2, 1]];
        assert_eq!(Solution::max_students_on_bench(students), 2);
    }

    #[test]
    fn empty_input_returns_zero() {
        assert_eq!(Solution::max_students_on_bench(vec![]), 0);
    }
}
