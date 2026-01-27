impl Solution {
    /// Counts minimum moves to convert all X's to O's in groups of three.
    ///
    /// # Intuition
    /// Each move converts three consecutive characters. Greedily convert
    /// starting at each X encountered, then skip three positions.
    ///
    /// # Approach
    /// 1. Scan left to right.
    /// 2. On encountering 'X', increment count and skip ahead by 3.
    /// 3. Otherwise, advance by 1.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_moves(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut moves = 0;
        let mut i = 0;

        while i < n {
            if bytes[i] == b'X' {
                moves += 1;
                i += 3;
            } else {
                i += 1;
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(Solution::minimum_moves("XXOX".to_string()), 2);
    }

    #[test]
    fn test_no_x() {
        assert_eq!(Solution::minimum_moves("OOOO".to_string()), 0);
    }
}
