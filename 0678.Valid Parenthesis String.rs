impl Solution {
    /// Greedy range tracking of unmatched open parentheses.
    ///
    /// # Intuition
    /// Each `*` can act as `(`, `)`, or empty, so the count of unmatched opens
    /// is a range rather than a single balance. The string is valid when that
    /// range never goes fully negative and can finish at zero.
    ///
    /// # Approach
    /// Scan once, keeping the lowest and highest possible balances. `(` raises
    /// both bounds, `)` lowers both, and `*` lowers the lower bound while
    /// raising the upper bound. A negative upper bound means some `)` has no
    /// match under any interpretation of `*`. Clamp the lower bound at zero,
    /// because extra `*` can be treated as empty. Accept when the lower bound
    /// ends at zero, so at least one interpretation closes every open.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn check_valid_string(s: String) -> bool {
        let (mut low, mut high) = (0, 0);
        for &byte in s.as_bytes() {
            match byte {
                b'(' => {
                    low += 1;
                    high += 1;
                }
                b')' => {
                    low -= 1;
                    high -= 1;
                }
                _ => {
                    low -= 1;
                    high += 1;
                }
            }
            if high < 0 {
                return false;
            }
            low = low.max(0);
        }
        low == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_plain_pair() {
        assert!(Solution::check_valid_string("()".to_string()));
    }

    #[test]
    fn example_star_between() {
        assert!(Solution::check_valid_string("(*)".to_string()));
    }

    #[test]
    fn example_extra_close_covered_by_star() {
        assert!(Solution::check_valid_string("(*))".to_string()));
    }

    #[test]
    fn example_unmatched_open() {
        assert!(!Solution::check_valid_string("(".to_string()));
    }

    #[test]
    fn unmatched_close() {
        assert!(!Solution::check_valid_string(")".to_string()));
    }

    #[test]
    fn close_before_open() {
        assert!(!Solution::check_valid_string(")(".to_string()));
    }

    #[test]
    fn single_star_is_empty() {
        assert!(Solution::check_valid_string("*".to_string()));
    }

    #[test]
    fn stars_only() {
        assert!(Solution::check_valid_string("***".to_string()));
    }

    #[test]
    fn star_closes_open() {
        assert!(Solution::check_valid_string("(*".to_string()));
    }

    #[test]
    fn star_opens_for_close() {
        assert!(Solution::check_valid_string("*)".to_string()));
    }

    #[test]
    fn too_many_opens_for_stars() {
        assert!(!Solution::check_valid_string("(((*)".to_string()));
    }

    #[test]
    fn nested_and_sequential() {
        assert!(Solution::check_valid_string("(*()".to_string()));
        assert!(Solution::check_valid_string("(())".to_string()));
        assert!(!Solution::check_valid_string("((())".to_string()));
    }

    #[test]
    fn long_valid_mix() {
        let s = "((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()";
        assert!(Solution::check_valid_string(s.to_string()));
    }

    #[test]
    fn long_invalid_mix() {
        let s = "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())";
        assert!(!Solution::check_valid_string(s.to_string()));
    }
}
