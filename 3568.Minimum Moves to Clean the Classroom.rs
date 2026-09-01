use std::collections::VecDeque;

impl Solution {
    /// BFS over `(row, col, collected_litter_mask)` states, pruning by the best
    /// remaining energy seen so far for each state.
    ///
    /// # Intuition
    /// Every move costs one step, so the shortest number of moves is a
    /// breadth-first search where each layer is one move. The catch is energy:
    /// two paths can reach the same cell with the same set of collected litter
    /// but different remaining energy, and more energy is never worse. So the
    /// full state is `(row, col, mask)` and we only keep exploring a state if we
    /// arrive with strictly more energy than we have recorded before.
    ///
    /// # Approach
    /// Assign each 'L' cell an index and encode collected litter as a bitmask.
    /// Run BFS from 'S' with full energy and empty mask. For every dequeued
    /// state, try the four neighbours:
    /// - Skip obstacles and out-of-bounds cells.
    /// - Moving costs 1 energy; if energy would drop below 0, skip.
    /// - Collect litter by setting its bit; step onto 'R' refills energy to the
    ///   maximum.
    ///
    /// Track `best[row][col][mask]` = maximum remaining energy recorded, and
    /// only enqueue a neighbour when the new energy beats the stored value. The
    /// first time every litter bit is set, the current move count is optimal.
    ///
    /// # Complexity
    /// - Time: O(m * n * 2^L) states, each with 4 transitions
    /// - Space: O(m * n * 2^L) for the visited/best table
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let grid: Vec<&[u8]> = classroom.iter().map(|row| row.as_bytes()).collect();
        let rows = grid.len();
        let cols = grid[0].len();

        // Index every litter cell and locate the start.
        let mut litter_index = vec![vec![usize::MAX; cols]; rows];
        let mut litter_count = 0usize;
        let (mut start_r, mut start_c) = (0usize, 0usize);
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                match cell {
                    b'L' => {
                        litter_index[r][c] = litter_count;
                        litter_count += 1;
                    }
                    b'S' => {
                        start_r = r;
                        start_c = c;
                    }
                    _ => {}
                }
            }
        }

        let full_mask = (1usize << litter_count) - 1;
        if litter_count == 0 {
            return 0;
        }

        // best[r][c][mask] = maximum remaining energy recorded for that state.
        let masks = 1usize << litter_count;
        let mut best = vec![vec![vec![-1i32; masks]; cols]; rows];

        let start_mask = if grid[start_r][start_c] == b'L' {
            1 << litter_index[start_r][start_c]
        } else {
            0
        };
        best[start_r][start_c][start_mask] = energy;

        if start_mask == full_mask {
            return 0;
        }

        // Queue holds (row, col, mask, remaining_energy). moves tracked by layer.
        let mut queue = VecDeque::new();
        queue.push_back((start_r, start_c, start_mask, energy));

        let directions = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        let mut moves = 0i32;

        while !queue.is_empty() {
            moves += 1;
            for _ in 0..queue.len() {
                let (r, c, mask, cur_energy) = queue.pop_front().unwrap();
                for (dr, dc) in directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    let cell = grid[nr][nc];
                    if cell == b'X' {
                        continue;
                    }
                    let next_energy = cur_energy - 1;
                    if next_energy < 0 {
                        continue;
                    }

                    let mut new_mask = mask;
                    if cell == b'L' {
                        new_mask |= 1 << litter_index[nr][nc];
                    }
                    let arrive_energy = if cell == b'R' { energy } else { next_energy };

                    if new_mask == full_mask {
                        return moves;
                    }

                    if arrive_energy > best[nr][nc][new_mask] {
                        best[nr][nc][new_mask] = arrive_energy;
                        queue.push_back((nr, nc, new_mask, arrive_energy));
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_grid(rows: &[&str]) -> Vec<String> {
        rows.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_moves(to_grid(&["S.", "XL"]), 2), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_moves(to_grid(&["LS", "RL"]), 4), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_moves(to_grid(&["L.S", "RXL"]), 3), -1);
    }

    #[test]
    fn test_no_litter() {
        assert_eq!(Solution::min_moves(to_grid(&["S.", ".."]), 5), 0);
    }

    #[test]
    fn test_start_on_litter() {
        // Only one litter, and it is on the start cell.
        assert_eq!(Solution::min_moves(to_grid(&["L"]), 1), 0);
    }

    #[test]
    fn test_single_adjacent_litter() {
        assert_eq!(Solution::min_moves(to_grid(&["SL"]), 1), 1);
    }

    #[test]
    fn test_unreachable_blocked() {
        // Litter walled off by obstacles.
        assert_eq!(Solution::min_moves(to_grid(&["SX", "XL"]), 5), -1);
    }

    #[test]
    fn test_reset_required() {
        // Energy too small to reach litter without recharging at R.
        assert_eq!(Solution::min_moves(to_grid(&["S.R.L"]), 2), 4);
    }

    #[test]
    fn test_energy_insufficient_no_reset() {
        // Straight corridor with no reset; energy 2 cannot reach litter 3 away.
        assert_eq!(Solution::min_moves(to_grid(&["S..L"]), 2), -1);
    }
}
