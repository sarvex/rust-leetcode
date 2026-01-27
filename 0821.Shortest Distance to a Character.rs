impl Solution {
    /// Computes shortest distance from each position to nearest target char.
    ///
    /// # Intuition
    /// Two passes: left-to-right tracks distance from the last seen target,
    /// right-to-left refines with the nearest target from the right.
    ///
    /// # Approach
    /// First pass sets distances from the left. Second pass updates each
    /// position with the minimum of its current value and distance from
    /// the right.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let bytes = s.as_bytes();
        let target = c as u8;
        let n = bytes.len();
        let mut result = vec![n as i32; n];

        let mut prev = -(n as i32);
        for i in 0..n {
            if bytes[i] == target {
                prev = i as i32;
            }
            result[i] = (i as i32 - prev).abs();
        }

        prev = 2 * n as i32;
        for i in (0..n).rev() {
            if bytes[i] == target {
                prev = i as i32;
            }
            result[i] = result[i].min((i as i32 - prev).abs());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
    }

    #[test]
    fn test_single_occurrence() {
        assert_eq!(
            Solution::shortest_to_char("aaab".to_string(), 'b'),
            vec![3, 2, 1, 0]
        );
    }
}
