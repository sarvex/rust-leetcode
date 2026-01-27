use std::collections::VecDeque;

impl Solution {
    /// Computes the distance of each cell to the nearest zero using multi-source BFS.
    ///
    /// # Intuition
    /// Start BFS from all zero cells simultaneously. Each step expands to
    /// unvisited neighbors, naturally computing shortest distances.
    ///
    /// # Approach
    /// 1. Initialize the result matrix to -1 (unvisited).
    /// 2. Enqueue all zero cells with distance 0.
    /// 3. BFS: for each dequeued cell, set unvisited neighbors to current + 1.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n)
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut dist = vec![vec![-1i32; n]; m];
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    dist[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }
        let dirs: [isize; 5] = [-1, 0, 1, 0, -1];
        while let Some((i, j)) = queue.pop_front() {
            for d in 0..4 {
                let ni = i as isize + dirs[d];
                let nj = j as isize + dirs[d + 1];
                if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                    let (ni, nj) = (ni as usize, nj as usize);
                    if dist[ni][nj] == -1 {
                        dist[ni][nj] = dist[i][j] + 1;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn test_larger() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
