use std::collections::HashMap;

/// Hash map leaderboard with on-demand sorting for top-K queries.
///
/// # Intuition
/// A HashMap provides O(1) score updates and resets. For top-K queries,
/// collecting and partially sorting scores is sufficient since K is
/// typically small relative to the total player count.
///
/// # Approach
/// - `add_score`: accumulate scores per player using entry API
/// - `top`: collect all scores, sort descending, sum the first K
/// - `reset`: remove the player entirely from the map
///
/// # Complexity
/// - add_score: O(1) amortized
/// - top: O(n log n) for sorting
/// - reset: O(1)
/// - Space: O(n) for the player map
struct Leaderboard {
    scores: HashMap<i32, i32>,
}

impl Leaderboard {
    fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    fn add_score(&mut self, player_id: i32, score: i32) {
        *self.scores.entry(player_id).or_insert(0) += score;
    }

    fn top(&self, k: i32) -> i32 {
        let mut vals: Vec<i32> = self.scores.values().copied().collect();
        vals.sort_unstable_by(|a, b| b.cmp(a));
        vals.iter().take(k as usize).sum()
    }

    fn reset(&mut self, player_id: i32) {
        self.scores.remove(&player_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaderboard_operations() {
        let mut lb = Leaderboard::new();
        lb.add_score(1, 73);
        lb.add_score(2, 56);
        lb.add_score(3, 39);
        lb.add_score(4, 51);
        lb.add_score(5, 4);
        assert_eq!(lb.top(1), 73);
        lb.reset(1);
        lb.reset(2);
        lb.add_score(2, 51);
        assert_eq!(lb.top(3), 141);
    }
}
