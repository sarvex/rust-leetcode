impl Solution {
    /// Simulates champagne tower overflow row by row.
    ///
    /// # Intuition
    /// Each glass overflows equally to its two children below. Simulate the
    /// pour row by row, distributing excess champagne downward.
    ///
    /// # Approach
    /// Start with all champagne in glass `(0, 0)`. Use two preallocated
    /// buffers and alternate (double-buffer) to avoid allocating a new vec
    /// each row. Only the slice that will be written is zeroed each step.
    ///
    /// # Complexity
    /// - Time: O(query_row^2)
    /// - Space: O(query_row)
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        let cap = query_row + 2;

        let mut row0 = vec![0.0_f64; cap];
        let mut row1 = vec![0.0_f64; cap];
        row0[0] = poured as f64;

        let (mut cur, mut nxt) = (&mut row0, &mut row1);

        for i in 1..=query_row {
            nxt[..=i].fill(0.0);
            for j in 0..i {
                let overflow = (cur[j] - 1.0).max(0.0) * 0.5;
                nxt[j] += overflow;
                nxt[j + 1] += overflow;
            }
            std::mem::swap(&mut cur, &mut nxt);
        }

        cur[query_glass].min(1.0)
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
