impl Solution {
    /// DFS backtracking grid search for word existence.
    ///
    /// # Intuition
    /// Starting from each cell that matches the first character, explore
    /// all four directions recursively. A visited matrix prevents revisiting
    /// cells within the same path. Backtracking unmarks cells on return.
    ///
    /// # Approach
    /// For each cell matching `word[0]`, launch DFS. At each step, check
    /// character match and boundary conditions. Mark the cell visited,
    /// recurse in four directions, and unmark on backtrack. Return true
    /// if the entire word is matched.
    ///
    /// # Complexity
    /// - Time: O(m × n × 3^L) — at each cell, up to 3 directions (excluding where we came from), L word length
    /// - Space: O(L) — recursion depth equals word length
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let word = word.as_bytes();
        let mut visited = vec![vec![false; n]; m];

        fn dfs(
            row: usize,
            col: usize,
            pos: usize,
            word: &[u8],
            board: &[Vec<char>],
            visited: &mut [Vec<bool>],
        ) -> bool {
            if board[row][col] as u8 != word[pos] {
                return false;
            }
            if pos == word.len() - 1 {
                return true;
            }

            visited[row][col] = true;
            let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let (rows, cols) = (board.len() as i32, board[0].len() as i32);

            let found = directions.iter().any(|&(dr, dc)| {
                let (nr, nc) = (row as i32 + dr, col as i32 + dc);
                nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && !visited[nr as usize][nc as usize]
                    && dfs(nr as usize, nc as usize, pos + 1, word, board, visited)
            });

            visited[row][col] = false;
            found
        }

        (0..m).any(|i| (0..n).any(|j| dfs(i, j, 0, word, &board, &mut visited)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_found() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist(board, "ABCCED".to_string()));
    }

    #[test]
    fn word_found_snake() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist(board, "SEE".to_string()));
    }

    #[test]
    fn word_not_found() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(!Solution::exist(board, "ABCB".to_string()));
    }

    #[test]
    fn single_cell() {
        assert!(Solution::exist(vec![vec!['A']], "A".to_string()));
    }
}
