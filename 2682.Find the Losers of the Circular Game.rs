impl Solution {
    /// Find players who never receive the ball in a circular game.
    ///
    /// # Intuition
    /// Simulate the game: the ball moves forward by increasing multiples of k.
    /// Track visited positions and return those never visited.
    ///
    /// # Approach
    /// 1. Simulate ball movement with step size p*k (p increments each turn)
    /// 2. Mark visited positions, stopping when a position is revisited
    /// 3. Collect all unvisited positions as losers
    ///
    /// # Complexity
    /// - Time: O(n) — at most n positions before revisit
    /// - Space: O(n)
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n_us = n as usize;
        let mut visited = vec![false; n_us];
        let mut pos = 0usize;
        let mut step = 1usize;

        while !visited[pos] {
            visited[pos] = true;
            pos = (pos + step * k as usize) % n_us;
            step += 1;
        }

        visited
            .iter()
            .enumerate()
            .filter(|(_, &v)| !v)
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::circular_game_losers(5, 2), vec![4, 5]);
    }

    #[test]
    fn test_all_visited() {
        assert_eq!(Solution::circular_game_losers(2, 1), vec![]);
    }

    #[test]
    fn test_large_k() {
        assert_eq!(Solution::circular_game_losers(4, 4), vec![2, 3, 4]);
    }
}
