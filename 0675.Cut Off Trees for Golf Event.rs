use std::collections::{HashSet, VecDeque};

impl Solution {
    /// Finds the minimum steps to cut all trees in height order using BFS.
    ///
    /// # Intuition
    /// Trees must be cut in ascending height order. BFS between consecutive
    /// target trees computes the shortest path on the grid.
    ///
    /// # Approach
    /// 1. Collect all tree positions sorted by height.
    /// 2. BFS from the current position to each tree in order.
    /// 3. Sum all BFS distances; return -1 if any tree is unreachable.
    ///
    /// # Complexity
    /// - Time: O(t × m × n) where t = number of trees
    /// - Space: O(m × n)
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (forest.len() as i32, forest[0].len() as i32);
        let dirs: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

        let bfs = |start: i32, end: i32| -> i32 {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back(start);
            visited.insert(start);
            let mut steps = 0;
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let pos = queue.pop_front().unwrap();
                    if pos == end {
                        return steps;
                    }
                    for d in &dirs {
                        let x = pos / cols + d[0];
                        let y = pos % cols + d[1];
                        let next = x * cols + y;
                        if x >= 0
                            && x < rows
                            && y >= 0
                            && y < cols
                            && forest[x as usize][y as usize] != 0
                            && visited.insert(next)
                        {
                            queue.push_back(next);
                        }
                    }
                }
                steps += 1;
            }
            -1
        };

        let mut trees: Vec<(i32, i32)> = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                let h = forest[i as usize][j as usize];
                if h > 1 {
                    trees.push((h, i * cols + j));
                }
            }
        }
        trees.sort_unstable();

        let mut total = 0;
        let mut pos = 0;
        for &(_, target) in &trees {
            let dist = bfs(pos, target);
            if dist == -1 {
                return -1;
            }
            total += dist;
            pos = target;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]),
            6
        );
    }

    #[test]
    fn test_unreachable() {
        assert_eq!(
            Solution::cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]]),
            -1
        );
    }
}
