impl Solution {
    /// Backtracking generation of all valid parenthesis combinations.
    ///
    /// # Intuition
    /// A valid combination can be built character by character: add '(' when
    /// fewer than `n` have been used, and add ')' only when the count of
    /// closing brackets is less than the count of opening brackets.
    ///
    /// # Approach
    /// Use DFS with two counters (open and close). At each step, branch
    /// into adding '(' or ')' when the constraints allow. Collect the
    /// string when both counters reach `n`.
    ///
    /// # Complexity
    /// - Time: O(4^n / sqrt(n)) — the nth Catalan number of valid sequences
    /// - Space: O(n) — recursion depth and current string buffer
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(result: &mut Vec<String>, current: &mut String, open: i32, close: i32, n: i32) {
            if open == n && close == n {
                result.push(current.clone());
                return;
            }
            if open < n {
                current.push('(');
                dfs(result, current, open + 1, close, n);
                current.pop();
            }
            if close < open {
                current.push(')');
                dfs(result, current, open, close + 1, n);
                current.pop();
            }
        }

        let mut result = Vec::new();
        dfs(
            &mut result,
            &mut String::with_capacity(n as usize * 2),
            0,
            0,
            n,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_equals_three() {
        let mut result = Solution::generate_parenthesis(3);
        result.sort();
        assert_eq!(
            result,
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn n_equals_one() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }

    #[test]
    fn n_equals_two() {
        let mut result = Solution::generate_parenthesis(2);
        result.sort();
        assert_eq!(result, vec!["(())", "()()"]);
    }
}
