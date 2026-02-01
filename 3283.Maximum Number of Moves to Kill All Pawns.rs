impl Solution {
    /// Minimax over pawn choices with knight-move distances.
    ///
    /// # Intuition
    /// The number of moves needed to capture a chosen pawn is fixed: it is the knight distance
    /// from the current square. So the game state is just the current knight position and the
    /// set of remaining pawns.
    ///
    /// # Approach
    /// - Precompute knight distances between the start position and every pawn using BFS on the
    ///   50x50 board.
    /// - Use a bitmask DP with minimax: `dp[mask][pos]` is the optimal total moves from `pos`
    ///   with `mask` pawns remaining. Turn parity follows from how many pawns are already removed.
    /// - Alice maximizes the accumulated distance, Bob minimizes it.
    ///
    /// # Complexity
    /// - Time: O((P + 1) * 50 * 50 + 2^P * P), where P = positions.len()
    /// - Space: O(2^P * P + 50 * 50)
    pub fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        const BOARD_SIZE: usize = 50;
        const KNIGHT_DIRS: [(i32, i32); 8] = [
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ];

        fn bfs(start: (usize, usize)) -> Vec<Vec<i32>> {
            let mut dist = vec![vec![-1; BOARD_SIZE]; BOARD_SIZE];
            let mut queue = VecDeque::new();
            dist[start.0][start.1] = 0;
            queue.push_back(start);
            while let Some((x, y)) = queue.pop_front() {
                let next = dist[x][y] + 1;
                for (dx, dy) in KNIGHT_DIRS {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 && nx < BOARD_SIZE as i32 && ny < BOARD_SIZE as i32 {
                        let ux = nx as usize;
                        let uy = ny as usize;
                        if dist[ux][uy] == -1 {
                            dist[ux][uy] = next;
                            queue.push_back((ux, uy));
                        }
                    }
                }
            }
            dist
        }

        let pawn_count = positions.len();
        if pawn_count == 0 {
            return 0;
        }

        let mut points = Vec::with_capacity(pawn_count + 1);
        points.push((kx as usize, ky as usize));
        points.extend(
            positions
                .into_iter()
                .map(|pos| (pos[0] as usize, pos[1] as usize)),
        );

        let mut distances = vec![vec![0_i32; pawn_count + 1]; pawn_count + 1];
        for i in 0..=pawn_count {
            let dist = bfs(points[i]);
            for j in 0..=pawn_count {
                distances[i][j] = dist[points[j].0][points[j].1];
            }
        }

        fn solve(
            mask: usize,
            pos: usize,
            total: usize,
            distances: &[Vec<i32>],
            memo: &mut [Vec<i32>],
        ) -> i32 {
            if mask == 0 {
                return 0;
            }
            if memo[mask][pos] != i32::MIN {
                return memo[mask][pos];
            }

            let moves_done = total - mask.count_ones() as usize;
            let alice_turn = moves_done % 2 == 0;
            let mut best = if alice_turn { i32::MIN } else { i32::MAX };
            let mut remaining = mask;

            while remaining != 0 {
                let idx = remaining.trailing_zeros() as usize;
                remaining &= remaining - 1;
                let pawn_index = idx + 1;
                let next_mask = mask & !(1_usize << idx);
                let value = distances[pos][pawn_index]
                    + solve(next_mask, pawn_index, total, distances, memo);
                if alice_turn {
                    if value > best {
                        best = value;
                    }
                } else if value < best {
                    best = value;
                }
            }

            memo[mask][pos] = best;
            best
        }

        let full_mask = (1_usize << pawn_count) - 1;
        let mut memo = vec![vec![i32::MIN; pawn_count + 1]; 1_usize << pawn_count];
        solve(full_mask, 0, pawn_count, &distances, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::max_moves(1, 1, vec![vec![0, 0]]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::max_moves(0, 2, vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            8
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::max_moves(0, 0, vec![vec![1, 2], vec![2, 4]]), 3);
    }

    #[test]
    fn no_pawns() {
        assert_eq!(Solution::max_moves(7, 7, vec![]), 0);
    }
}
