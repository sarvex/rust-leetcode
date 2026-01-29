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
        const MOD: i64 = 1_000_000_007;
        let n = s.len();

        // Map characters: F=0, W=1, E=2
        let alice: Vec<usize> = s
            .bytes()
            .map(|c| match c {
                b'F' => 0,
                b'W' => 1,
                _ => 2,
            })
            .collect();

        // outcome[alice][bob] = Bob's point change
        // F beats E, W beats F, E beats W
        let outcome: [[i32; 3]; 3] = [[0, 1, -1], [-1, 0, 1], [1, -1, 0]];

        let offset = n as i32;
        let size = 2 * n + 1;

        // dp[bob's last move][score difference + offset]
        let mut dp = vec![vec![0i64; size]; 3];

        // Initialize first round
        for bob in 0..3 {
            let diff = outcome[alice[0]][bob];
            dp[bob][(offset + diff) as usize] = 1;
        }

        // Process remaining rounds
        for i in 1..n {
            let mut next = vec![vec![0i64; size]; 3];
            for (last, dp_last) in dp.iter().enumerate() {
                for (d, &count) in dp_last.iter().enumerate() {
                    if count == 0 {
                        continue;
                    }
                    for bob in 0..3 {
                        if bob == last {
                            continue;
                        }
                        let delta = outcome[alice[i]][bob];
                        let new_d = d as i32 + delta;
                        if new_d >= 0 && new_d < size as i32 {
                            next[bob][new_d as usize] =
                                (next[bob][new_d as usize] + count) % MOD;
                        }
                    }
                }
            }
            dp = next;
        }

        // Sum winning states (diff > 0 means index > offset)
        dp.iter()
            .flat_map(|row| row.iter().skip(offset as usize + 1))
            .fold(0i64, |acc, &x| (acc + x) % MOD) as i32
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
