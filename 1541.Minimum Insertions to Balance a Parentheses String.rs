impl Solution {
    /// Greedy count of insertions so every `(` is closed by `))`.
    ///
    /// # Intuition
    /// `(` is an opener that demands two closers, and those two `)` must be
    /// consecutive. A `)` with nothing open therefore needs a new `(`. An
    /// opener that arrives while a previous pair is only half-closed needs one
    /// `)` inserted first; otherwise the two closers would be split.
    ///
    /// # Approach
    /// Track `needed`, how many `)` are still required, and fold over the
    /// bytes:
    /// - On `(`: if `needed` is odd, insert one `)` to finish the open pair
    ///   (`needed` increases by 1 net). Otherwise this opener adds 2.
    /// - On `)`: if `needed` is 0, insert a `(` and leave one `)` outstanding.
    ///   Otherwise consume one needed closer.
    ///
    /// Every closer still outstanding after the scan is inserted as well.
    ///
    /// # Complexity
    /// - Time: O(n) — one pass with O(1) work per character.
    /// - Space: O(1) — two counters.
    pub fn min_insertions(s: String) -> i32 {
        let (insertions, needed) = s
            .bytes()
            .fold((0_i32, 0_i32), |(insertions, needed), byte| match byte {
                b'(' if (needed & 1) == 1 => (insertions + 1, needed + 1),
                b'(' => (insertions, needed + 2),
                b')' if needed == 0 => (insertions + 1, 1),
                b')' => (insertions, needed - 1),
                _ => (insertions, needed),
            });
        insertions + needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_insertions("())".to_string()), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
    }

    #[test]
    fn test_already_balanced() {
        assert_eq!(Solution::min_insertions("())())".to_string()), 0);
        assert_eq!(Solution::min_insertions("(())))".to_string()), 0);
        assert_eq!(Solution::min_insertions("())(())))".to_string()), 0);
        assert_eq!(Solution::min_insertions("(())())))".to_string()), 0);
    }

    #[test]
    fn test_unbalanced_examples_from_statement() {
        // ")()" needs '(' before the first ')', one ')' to finish that pair,
        // and "))" for the final '('.
        assert_eq!(Solution::min_insertions(")()".to_string()), 3);
        // "()))" is two "))" pairs, each missing its '('.
        assert_eq!(Solution::min_insertions("()))".to_string()), 2);
        // "(()))" is example 1: one ')' short.
        assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
    }

    #[test]
    fn test_single_characters() {
        assert_eq!(Solution::min_insertions("(".to_string()), 2);
        assert_eq!(Solution::min_insertions(")".to_string()), 2);
    }

    #[test]
    fn test_short_patterns() {
        assert_eq!(Solution::min_insertions("()".to_string()), 1);
        assert_eq!(Solution::min_insertions("))".to_string()), 1);
        assert_eq!(Solution::min_insertions(")(".to_string()), 4);
        assert_eq!(Solution::min_insertions("))((".to_string()), 5);
        assert_eq!(Solution::min_insertions("((((".to_string()), 8);
        assert_eq!(Solution::min_insertions("))))".to_string()), 2);
        assert_eq!(Solution::min_insertions(")))))))".to_string()), 5);
    }

    #[test]
    fn test_half_closed_before_next_open() {
        // After "(()))" one ')' is still owed, so the next '(' forces that
        // insertion before its own "))".
        assert_eq!(Solution::min_insertions("(()))(".to_string()), 3);
    }

    #[test]
    fn test_large_all_opens() {
        let s = "(".repeat(100_000);
        assert_eq!(Solution::min_insertions(s), 200_000);
    }

    #[test]
    fn test_large_all_closes() {
        // Each "))" needs one '('; an even count leaves nothing half-closed.
        let s = ")".repeat(100_000);
        assert_eq!(Solution::min_insertions(s), 50_000);
    }

    #[test]
    fn test_large_already_balanced() {
        let s = "())".repeat(50_000);
        assert_eq!(Solution::min_insertions(s), 0);
    }
}
