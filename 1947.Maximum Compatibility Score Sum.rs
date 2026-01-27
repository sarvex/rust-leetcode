impl Solution {
    /// Maximizes total compatibility score via backtracking permutation search.
    ///
    /// # Intuition
    /// With at most 8 students/mentors, exhaustive permutation search is
    /// feasible. Precompute the pairwise compatibility scores.
    ///
    /// # Approach
    /// 1. Build a compatibility matrix g[i][j] for each student-mentor pair.
    /// 2. Use DFS with backtracking to try all mentor assignments.
    /// 3. Track the maximum total score.
    ///
    /// # Complexity
    /// - Time: O(m! * m) where m is number of students
    /// - Space: O(m^2)
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let m = students.len();
        let n = students[0].len();

        let compat: Vec<Vec<i32>> = (0..m)
            .map(|i| {
                (0..m)
                    .map(|j| (0..n).filter(|&k| students[i][k] == mentors[j][k]).count() as i32)
                    .collect()
            })
            .collect();

        let mut best = 0;
        let mut used = vec![false; m];

        fn dfs(
            student: usize,
            score: i32,
            m: usize,
            compat: &[Vec<i32>],
            used: &mut Vec<bool>,
            best: &mut i32,
        ) {
            if student == m {
                *best = (*best).max(score);
                return;
            }
            for j in 0..m {
                if !used[j] {
                    used[j] = true;
                    dfs(
                        student + 1,
                        score + compat[student][j],
                        m,
                        compat,
                        used,
                        best,
                    );
                    used[j] = false;
                }
            }
        }

        dfs(0, 0, m, &compat, &mut used, &mut best);
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::max_compatibility_sum(
                vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 1]],
                vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]
            ),
            8
        );
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(
            Solution::max_compatibility_sum(vec![vec![0]], vec![vec![0]]),
            1
        );
    }
}
