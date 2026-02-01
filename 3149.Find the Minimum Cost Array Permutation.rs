const MAX_N: usize = 14;

#[derive(Clone, Copy)]
struct State {
    cost: i32,
    len: u8,
    path: [u8; MAX_N],
}

impl Solution {
    /// Subset DP over Hamiltonian cycles with lexicographic tie-breaking.
    ///
    /// # Intuition
    /// The score is the sum of edge costs on a Hamiltonian cycle where the edge `(a -> b)` costs
    /// `|a - nums[b]|`. Rotating a cycle does not change the score, so we can fix the first element
    /// of the permutation to `0` and search for the best cycle starting at `0`.
    ///
    /// # Approach
    /// Use DP over subsets: `dp[mask][last]` is the minimum cost to start at `0`, visit exactly the
    /// nodes in `mask`, and end at `last`. For ties, store the lexicographically smallest path for
    /// that state. After filling the table, close the cycle by adding the edge `last -> 0`, then
    /// select the lexicographically smallest path among those with the minimum total cost.
    ///
    /// # Complexity
    /// - Time: O(n^2 * 2^n)
    /// - Space: O(n * 2^n)
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let total_masks = 1_usize << n;
        let mut edge_cost = vec![vec![0_i32; n]; n];
        for from in 0..n {
            for to in 0..n {
                edge_cost[from][to] = (from as i32 - nums[to]).abs();
            }
        }

        let inf = i32::MAX / 4;
        let mut dp = vec![
            State {
                cost: inf,
                len: 0,
                path: [0; MAX_N],
            };
            total_masks * n
        ];

        let start_mask = 1_usize << 0;
        dp[start_mask * n] = State {
            cost: 0,
            len: 1,
            path: [0; MAX_N],
        };

        for mask in 0..total_masks {
            if mask & 1 == 0 {
                continue;
            }
            for last in 0..n {
                if mask & (1_usize << last) == 0 {
                    continue;
                }
                let idx = mask * n + last;
                let state = dp[idx];
                if state.cost == inf {
                    continue;
                }
                let len = state.len as usize;
                for next in 0..n {
                    if mask & (1_usize << next) != 0 {
                        continue;
                    }
                    let next_mask = mask | (1_usize << next);
                    let new_cost = state.cost + edge_cost[last][next];
                    let next_idx = next_mask * n + next;
                    let next_state = dp[next_idx];
                    let new_len = len + 1;
                    let mut candidate_path = state.path;
                    candidate_path[len] = next as u8;

                    let should_update = if new_cost < next_state.cost {
                        true
                    } else if new_cost == next_state.cost {
                        Self::is_lex_smaller(
                            &candidate_path,
                            new_len,
                            &next_state.path,
                            next_state.len as usize,
                        )
                    } else {
                        false
                    };

                    if should_update {
                        dp[next_idx] = State {
                            cost: new_cost,
                            len: new_len as u8,
                            path: candidate_path,
                        };
                    }
                }
            }
        }

        let full_mask = total_masks - 1;
        let mut best_cost = inf;
        let mut best_path = [0_u8; MAX_N];
        let mut best_len = 0_usize;
        for last in 0..n {
            let idx = full_mask * n + last;
            let state = dp[idx];
            if state.cost == inf {
                continue;
            }
            let total_cost = state.cost + edge_cost[last][0];
            if total_cost < best_cost
                || (total_cost == best_cost
                    && Self::is_lex_smaller(
                        &state.path,
                        state.len as usize,
                        &best_path,
                        best_len,
                    ))
            {
                best_cost = total_cost;
                best_path = state.path;
                best_len = state.len as usize;
            }
        }

        best_path[..best_len]
            .iter()
            .map(|&value| value as i32)
            .collect()
    }

    fn is_lex_smaller(
        left: &[u8; MAX_N],
        left_len: usize,
        right: &[u8; MAX_N],
        right_len: usize,
    ) -> bool {
        if left_len != right_len {
            return left_len < right_len;
        }
        for i in 0..left_len {
            let left_value = left[i];
            let right_value = right[i];
            if left_value != right_value {
                return left_value < right_value;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = Solution::find_permutation(vec![1, 0, 2]);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::find_permutation(vec![0, 2, 1]);
        assert_eq!(result, vec![0, 2, 1]);
    }

    #[test]
    fn test_lexicographic_tie() {
        let result = Solution::find_permutation(vec![0, 1, 2]);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_two_elements() {
        let result = Solution::find_permutation(vec![1, 0]);
        assert_eq!(result, vec![0, 1]);
    }
}
