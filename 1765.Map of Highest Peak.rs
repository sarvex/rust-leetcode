use std::collections::VecDeque;

impl Solution {
    /// Multi-source BFS from water cells to assign minimum heights.
    ///
    /// # Intuition
    /// Water cells have height 0. Each land cell's height is its shortest
    /// distance to any water cell. BFS from all water cells simultaneously
    /// guarantees adjacent cells differ by at most 1.
    ///
    /// # Approach
    /// 1. Initialize result grid with -1 (unvisited).
    /// 2. Enqueue all water cells with height 0.
    /// 3. BFS outward, setting each unvisited neighbor's height to current + 1.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n)
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (is_water.len(), is_water[0].len());
        let mut height = vec![vec![-1; cols]; rows];
        let mut queue = VecDeque::new();
        for r in 0..rows {
            for c in 0..cols {
                if is_water[r][c] == 1 {
                    height[r][c] = 0;
                    queue.push_back((r, c));
                }
            }
        }
        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if height[nr][nc] == -1 {
                        height[nr][nc] = height[r][c] + 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
        height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let result = Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]);
        assert_eq!(result, vec![vec![1, 0], vec![2, 1]]);
    }

    #[test]
    fn test_example_two() {
        let result = Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]);
        assert_eq!(result, vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]);
    }
}
