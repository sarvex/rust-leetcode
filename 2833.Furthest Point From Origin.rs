impl Solution {
    /// Counts directional moves and converts all wildcards into the dominant direction.
    ///
    /// # Intuition
    /// The furthest reachable point is maximized when every flexible `'_'` move
    /// commits to the same direction as the majority of fixed moves. The signed
    /// displacement from fixed moves alone is `|#R - #L|`, and each wildcard
    /// extends that magnitude by exactly one, so the answer is
    /// `|#R - #L| + #_`.
    ///
    /// # Approach
    /// Iterate the bytes of `moves` once, maintaining a running signed balance
    /// (`+1` for `'R'`, `-1` for `'L'`) and a wildcard counter. The final
    /// distance is `balance.abs() + wildcards`.
    ///
    /// # Complexity
    /// - Time: O(n) where n = moves.len()
    /// - Space: O(1)
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (balance, wildcards) = moves.bytes().fold((0_i32, 0_i32), |(b, w), c| match c {
            b'R' => (b + 1, w),
            b'L' => (b - 1, w),
            _ => (b, w + 1),
        });
        balance.abs() + wildcards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::furthest_distance_from_origin("L_RL__R".to_string()),
            3
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::furthest_distance_from_origin("_R__LL_".to_string()),
            5
        );
    }

    #[test]
    fn test_example_three() {
        assert_eq!(
            Solution::furthest_distance_from_origin("_______".to_string()),
            7
        );
    }

    #[test]
    fn test_single_left() {
        assert_eq!(Solution::furthest_distance_from_origin("L".to_string()), 1);
    }

    #[test]
    fn test_single_right() {
        assert_eq!(Solution::furthest_distance_from_origin("R".to_string()), 1);
    }

    #[test]
    fn test_single_wildcard() {
        assert_eq!(Solution::furthest_distance_from_origin("_".to_string()), 1);
    }

    #[test]
    fn test_balanced_no_wildcards() {
        assert_eq!(
            Solution::furthest_distance_from_origin("LRLR".to_string()),
            0
        );
    }

    #[test]
    fn test_all_left() {
        assert_eq!(
            Solution::furthest_distance_from_origin("LLLLL".to_string()),
            5
        );
    }

    #[test]
    fn test_all_right() {
        assert_eq!(
            Solution::furthest_distance_from_origin("RRRRR".to_string()),
            5
        );
    }

    #[test]
    fn test_balanced_with_wildcards() {
        // 2 L, 2 R, 3 _  -> |0| + 3 = 3
        assert_eq!(
            Solution::furthest_distance_from_origin("LR_LR_".to_string()),
            2
        );
    }

    #[test]
    fn test_max_length_all_wildcards() {
        let moves = "_".repeat(50);
        assert_eq!(Solution::furthest_distance_from_origin(moves), 50);
    }

    #[test]
    fn test_max_length_mixed() {
        // 25 L, 25 R -> 0
        let moves: String = "LR".repeat(25);
        assert_eq!(Solution::furthest_distance_from_origin(moves), 0);
    }

    #[test]
    fn test_mostly_right_with_wildcards() {
        // 1 L, 3 R, 2 _ -> |3-1| + 2 = 4
        assert_eq!(
            Solution::furthest_distance_from_origin("RRRL__".to_string()),
            4
        );
    }
}
