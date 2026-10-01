impl Solution {
    /// Splits a valid parentheses string into two groups by depth parity.
    ///
    /// # Intuition
    /// Nesting depth is the running count of unmatched `(`. Consecutive depths
    /// differ by one, so odd depths and even depths form two separate valid
    /// strings, each about half as deep. That minimum possible maximum is
    /// `ceil(depth(seq) / 2)`.
    ///
    /// # Approach
    /// 1. Scan the string once, tracking the current depth.
    /// 2. On `(`, assign group `depth & 1`, then increment the depth.
    /// 3. On `)`, decrement the depth first, then assign `depth & 1`, so the
    ///    closer joins the opener that produced this depth.
    /// 4. `0` marks subsequence A and `1` marks subsequence B.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the answer; O(1) extra
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut answer = Vec::with_capacity(seq.len());
        let mut depth = 0i32;
        for byte in seq.bytes() {
            let group = if byte == b'(' {
                let group = depth & 1;
                depth += 1;
                group
            } else {
                depth -= 1;
                depth & 1
            };
            answer.push(group);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nesting_depth(seq: &str) -> i32 {
        seq.bytes()
            .scan(0, |depth, byte| {
                if byte == b'(' {
                    *depth += 1;
                } else {
                    *depth -= 1;
                }
                Some(*depth)
            })
            .max()
            .unwrap_or(0)
    }

    /// Both groups are valid parentheses strings, and the deeper one is as
    /// shallow as `ceil(depth(seq) / 2)`.
    fn assert_optimal(seq: &str) {
        let answer = Solution::max_depth_after_split(seq.to_string());
        assert_eq!(answer.len(), seq.len());

        let mut balance = [0i32; 2];
        let mut max_depth = [0i32; 2];
        for (byte, group) in seq.bytes().zip(answer) {
            let group = group as usize;
            assert!(group < 2);
            if byte == b'(' {
                balance[group] += 1;
                max_depth[group] = max_depth[group].max(balance[group]);
            } else {
                balance[group] -= 1;
                assert!(balance[group] >= 0);
            }
        }
        assert_eq!(balance, [0, 0]);

        let depth = nesting_depth(seq);
        let optimal = (depth + 1) / 2;
        assert_eq!(max_depth[0].max(max_depth[1]), optimal);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::max_depth_after_split("(()())".to_string()),
            vec![0, 1, 1, 1, 1, 0]
        );
        assert_optimal("(()())");
    }

    #[test]
    fn test_example_2() {
        assert_optimal("()(())()");
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(
            Solution::max_depth_after_split("()".to_string()),
            vec![0, 0]
        );
        assert_optimal("()");
    }

    #[test]
    fn test_adjacent_pairs() {
        assert_optimal("()()()");
    }

    #[test]
    fn test_fully_nested() {
        assert_optimal("(((())))");
    }

    #[test]
    fn test_mixed_primitives() {
        assert_optimal("(()())((()))");
    }

    #[test]
    fn test_odd_maximum_depth() {
        assert_optimal("((()))");
    }
}
