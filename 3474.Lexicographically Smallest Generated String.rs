impl Solution {
    /// Generates the lexicographically smallest string satisfying T/F pattern constraints.
    ///
    /// # Intuition
    /// Process 'T' constraints first by stamping the pattern at required positions,
    /// fill gaps with 'a', then fix 'F' violations by changing the rightmost
    /// non-forced character to 'b'.
    ///
    /// # Approach
    /// 1. Place `str2` at every 'T' position in `str1`, detecting conflicting overlaps
    /// 2. Fill all remaining positions with 'a' (lexicographically smallest character)
    /// 3. For each 'F' constraint whose window still matches `str2`, modify the
    ///    rightmost non-forced character to 'b' to break the match
    /// 4. Return empty string if any constraint is unsatisfiable
    ///
    /// # Complexity
    /// - Time: O(n × m) where n = `str1` length, m = `str2` length
    /// - Space: O(n + m) for result array and forced-position flags
    pub fn generate_string(str1: String, str2: String) -> String {
        let constraint = str1.as_bytes();
        let pattern = str2.as_bytes();
        let n = constraint.len();
        let m = pattern.len();
        let len = n + m - 1;
        let mut result = vec![0u8; len];
        let mut forced = vec![false; len];

        for i in 0..n {
            if constraint[i] != b'T' {
                continue;
            }
            for j in 0..m {
                let pos = i + j;
                if result[pos] == 0 {
                    result[pos] = pattern[j];
                    forced[pos] = true;
                } else if result[pos] != pattern[j] {
                    return String::new();
                }
            }
        }

        for ch in &mut result {
            if *ch == 0 {
                *ch = b'a';
            }
        }

        for i in 0..n {
            if constraint[i] != b'F' {
                continue;
            }
            if !(0..m).all(|j| result[i + j] == pattern[j]) {
                continue;
            }
            match (0..m).rev().find(|offset| !forced[i + *offset]) {
                Some(offset) => result[i + offset] = b'b',
                None => return String::new(),
            }
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_false_alternating_with_short_pattern() {
        assert_eq!(
            Solution::generate_string("TFTF".into(), "ab".into()),
            "ababa"
        );
    }

    #[test]
    fn conflicting_overlapping_true_constraints() {
        assert_eq!(Solution::generate_string("TFTF".into(), "abc".into()), "");
    }

    #[test]
    fn single_false_avoids_pattern() {
        assert_eq!(Solution::generate_string("F".into(), "d".into()), "a");
    }

    #[test]
    fn single_true_places_pattern() {
        assert_eq!(Solution::generate_string("T".into(), "abc".into()), "abc");
    }

    #[test]
    fn all_false_single_char_pattern() {
        assert_eq!(Solution::generate_string("FF".into(), "a".into()), "bb");
    }

    #[test]
    fn adjacent_true_constraints_conflict() {
        assert_eq!(Solution::generate_string("TT".into(), "ab".into()), "");
    }

    #[test]
    fn false_with_pattern_a_fills_b() {
        assert_eq!(Solution::generate_string("F".into(), "a".into()), "b");
    }

    #[test]
    fn true_then_false_no_conflict() {
        assert_eq!(Solution::generate_string("TF".into(), "ab".into()), "aba");
    }
}
