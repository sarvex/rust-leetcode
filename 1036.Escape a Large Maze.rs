use std::collections::{HashSet, VecDeque};

impl Solution {
    /// Determines if source can reach target in a large grid with blocked cells.
    ///
    /// # Intuition
    /// With at most 200 blocked cells, the maximum enclosed area is bounded
    /// (~20000 cells). BFS from both source and target; if either explores
    /// beyond this limit, it has escaped the blocked region.
    ///
    /// # Approach
    /// BFS from source and target independently. If BFS visits >= 20000 cells
    /// or reaches the other point, the path exists. If BFS terminates within
    /// the limit without reaching the target, it is enclosed.
    ///
    /// # Complexity
    /// - Time: O(B^2) where B is blocked cell count
    /// - Space: O(B^2) for visited sets
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let block: HashSet<(i32, i32)> = blocked.iter().map(|b| (b[0], b[1])).collect();
        Self::bfs(&block, &source, &target) && Self::bfs(&block, &target, &source)
    }

    fn bfs(block: &HashSet<(i32, i32)>, source: &[i32], target: &[i32]) -> bool {
        const LIMIT: usize = 20000;
        const BOUND: i32 = 1_000_000;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((source[0], source[1]));
        visited.insert((source[0], source[1]));

        while let Some((x, y)) = queue.pop_front() {
            if visited.len() >= LIMIT || (x == target[0] && y == target[1]) {
                return true;
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0
                    && nx < BOUND
                    && ny >= 0
                    && ny < BOUND
                    && !visited.contains(&(nx, ny))
                    && !block.contains(&(nx, ny))
                {
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }
        visited.len() >= LIMIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked() {
        assert!(!Solution::is_escape_possible(
            vec![vec![0, 1], vec![1, 0]],
            vec![0, 0],
            vec![0, 2],
        ));
    }

    #[test]
    fn test_no_blocks() {
        assert!(Solution::is_escape_possible(
            vec![],
            vec![0, 0],
            vec![999999, 999999],
        ));
    }
}
