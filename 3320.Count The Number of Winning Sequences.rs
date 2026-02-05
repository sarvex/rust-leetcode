impl Solution {
    /// Count winning sequences using DP with score difference tracking.
    ///
    /// # Intuition
    /// Model as rock-paper-scissors where Bob must count sequences that:
    /// 1. Never repeat consecutive moves
    /// 2. Result in strictly positive score difference
    ///
    /// # Approach
    /// Dynamic programming tracking (last_move, score_diff). For each round,
    /// transition to valid next moves while updating the score difference.
    /// Score difference ranges from -n to n, offset by n for array indexing.
    ///
    /// # Complexity
    /// - Time: O(n² × 9) = O(n²)
    /// - Space: O(n × 3) = O(n)
    pub fn count_winning_sequences(s: String) -> i32 {
        const MOD: u32 = 1_000_000_007;
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let bytes = s.as_bytes();

        // outcome[alice][bob] = Bob's point change
        // F beats E, W beats F, E beats W
        let outcome: [[i32; 3]; 3] = [[0, 1, -1], [-1, 0, 1], [1, -1, 0]];
        const NEXT_BOBS: [[usize; 2]; 3] = [[1, 2], [0, 2], [0, 1]];

        let offset = n as i32;
        let size = 2 * n + 1;
        let stride = size;

        // dp[bob's last move][score difference + offset]
        let mut dp = vec![0u32; 3 * stride];
        let mut next = vec![0u32; 3 * stride];

        // Initialize first round
        let first_alice = match bytes[0] {
            b'F' => 0,
            b'W' => 1,
            _ => 2,
        };
        (0..3).for_each(|bob| {
            let diff = outcome[first_alice][bob];
            let idx = (offset + diff) as usize;
            dp[bob * stride + idx] = 1;
        });

        // Process remaining rounds
        let mut min_index = offset - 1;
        let mut max_index = offset + 1;
        for i in 1..n {
            let alice_move = match bytes[i] {
                b'F' => 0,
                b'W' => 1,
                _ => 2,
            };
            let delta = outcome[alice_move];

            let next_min = min_index - 1;
            let next_max = max_index + 1;
            let next_start = next_min as usize;
            let next_len = (next_max - next_min + 1) as usize;
            (0..3).for_each(|bob| {
                let base = bob * stride + next_start;
                next[base..base + next_len].fill(0);
            });

            let min_usize = min_index as usize;
            let max_usize = max_index as usize;
            for last in 0..3 {
                let base = last * stride;
                let [bob1, bob2] = NEXT_BOBS[last];
                let delta1 = delta[bob1];
                let delta2 = delta[bob2];
                let dp_row = &dp[base..base + stride];
                let next_base1 = bob1 * stride;
                let next_base2 = bob2 * stride;
                let mut d = min_usize;
                while d <= max_usize {
                    let count = dp_row[d];
                    if count != 0 {
                        let d_i32 = d as i32;
                        let idx1 = next_base1 + (d_i32 + delta1) as usize;
                        let mut value1 = next[idx1] + count;
                        if value1 >= MOD {
                            value1 -= MOD;
                        }
                        next[idx1] = value1;

                        let idx2 = next_base2 + (d_i32 + delta2) as usize;
                        let mut value2 = next[idx2] + count;
                        if value2 >= MOD {
                            value2 -= MOD;
                        }
                        next[idx2] = value2;
                    }
                    d += 1;
                }
            }
            std::mem::swap(&mut dp, &mut next);
            min_index = next_min;
            max_index = next_max;
        }

        // Sum winning states (diff > 0 means index > offset)
        let start = (offset + 1) as usize;
        let total = (0..3)
            .flat_map(|bob| dp[bob * stride + start..(bob + 1) * stride].iter())
            .fold(0u32, |mut acc, &val| {
                acc += val;
                if acc >= MOD {
                    acc -= MOD;
                }
                acc
            });
        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_fire_dragons() {
        // Bob wins with "WFW", "FWF", or "WEW"
        assert_eq!(Solution::count_winning_sequences("FFF".to_string()), 3);
    }

    #[test]
    fn mixed_sequence() {
        assert_eq!(Solution::count_winning_sequences("FWEFW".to_string()), 18);
    }

    #[test]
    fn single_round() {
        // Bob can only win by beating Alice's single move
        // F -> Bob plays W (1 winning sequence)
        assert_eq!(Solution::count_winning_sequences("F".to_string()), 1);
        // W -> Bob plays E
        assert_eq!(Solution::count_winning_sequences("W".to_string()), 1);
        // E -> Bob plays F
        assert_eq!(Solution::count_winning_sequences("E".to_string()), 1);
    }

    #[test]
    fn two_rounds() {
        // Alice: "FW"
        // Bob needs score > 0 and can't repeat moves
        // Possible: FF(invalid), FW, FE, WF, WW(invalid), WE, EF, EW, EE(invalid)
        // Valid: FW, FE, WF, WE, EF, EW
        // FW: round1 F vs F = 0, round2 W vs W = 0 → total 0, not winning
        // FE: round1 F vs F = 0, round2 E vs W = +1 → total 1, winning
        // WF: round1 W vs F = +1, round2 F vs W = -1 → total 0, not winning
        // WE: round1 W vs F = +1, round2 E vs W = +1 → total 2, winning
        // EF: round1 E vs F = -1, round2 F vs W = -1 → total -2, not winning
        // EW: round1 E vs F = -1, round2 W vs W = 0 → total -1, not winning
        assert_eq!(Solution::count_winning_sequences("FW".to_string()), 2);
    }

    #[test]
    fn all_same_creature() {
        // All W: Bob needs to beat W, so E wins. Bob alternates E and something else
        assert_eq!(Solution::count_winning_sequences("WWW".to_string()), 3);
        // All E: Bob needs F to win
        assert_eq!(Solution::count_winning_sequences("EEE".to_string()), 3);
    }
}
