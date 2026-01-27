impl Solution {
    /// Simulates champagne tower overflow row by row.
    ///
    /// # Intuition
    /// Each glass overflows equally to its two children below. Simulate the
    /// pour row by row, distributing excess champagne downward.
    ///
    /// # Approach
    /// Start with all champagne in glass `(0, 0)`. For each row, compute
    /// overflow to the next row. The answer is `min(1.0, glass_amount)` at
    /// the queried position.
    ///
    /// # Complexity
    /// - Time: O(query_row^2)
    /// - Space: O(query_row) using single-row optimization
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        let mut row = vec![poured as f64];

        for i in 1..=query_row {
            let mut next = vec![0.0; i + 1];
            for j in 0..i {
                let overflow = (row[j] - 1.0).max(0.0) / 2.0;
                next[j] += overflow;
                next[j + 1] += overflow;
            }
            row = next;
        }

        row[query_glass].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_pour() {
        assert!((Solution::champagne_tower(1, 1, 1) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_overflow() {
        assert!((Solution::champagne_tower(2, 1, 1) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_large_pour() {
        let result = Solution::champagne_tower(100000009, 33, 17);
        assert!(result >= 0.0 && result <= 1.0);
    }

    #[test]
    fn test_top_glass() {
        assert!((Solution::champagne_tower(1, 0, 0) - 1.0).abs() < 1e-9);
    }
}
