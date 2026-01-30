use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Finds minimum knight moves to reach (x, y) from origin using BFS.
    ///
    /// # Intuition
    /// BFS from the origin explores all positions at increasing distances.
    /// Shifting coordinates avoids negative indices.
    ///
    /// # Approach
    /// Shift target and origin by 300 to handle negatives. BFS with 8
    /// knight moves. Mark visited cells to avoid revisiting.
    ///
    /// # Complexity
    /// - Time: O(max(|x|, |y|)^2) bounded by the search space
    /// - Space: O(max(|x|, |y|)^2) for the visited grid
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let (tx, ty) = ((x + 300) as usize, (y + 300) as usize);
        const MOVES: [(i32, i32); 8] = [
            (-2, 1),
            (2, 1),
            (-1, 2),
            (1, 2),
            (2, -1),
            (-2, -1),
            (1, -2),
            (-1, -2),
        ];
        let mut visited = vec![vec![false; 618]; 618];
        let mut queue = VecDeque::with_capacity(618 * 618);
        queue.push_back((300usize, 300usize));
        visited[300][300] = true;
        let mut steps = 0;

        loop {
            for _ in 0..queue.len() {
                let (cx, cy) = queue.pop_front().unwrap();
                if cx == tx && cy == ty {
                    return steps;
                }
                for (dx, dy) in MOVES {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0 && nx < 618 && ny >= 0 && ny < 618 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if !visited[nx][ny] {
                            visited[nx][ny] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
            steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        // Already at target
        assert_eq!(Solution::min_knight_moves(0, 0), 0);
    }

    #[test]
    fn test_one_move() {
        // One knight move away
        assert_eq!(Solution::min_knight_moves(2, 1), 1);
        assert_eq!(Solution::min_knight_moves(1, 2), 1);
    }

    #[test]
    fn test_example() {
        // Example from problem
        assert_eq!(Solution::min_knight_moves(5, 5), 4);
    }

    #[test]
    fn test_negative_coordinates() {
        assert_eq!(Solution::min_knight_moves(-2, -1), 1);
        assert_eq!(Solution::min_knight_moves(-5, -5), 4);
    }

    #[test]
    fn test_straight_line() {
        // Target on axis
        assert_eq!(Solution::min_knight_moves(1, 0), 3);
    }
}
