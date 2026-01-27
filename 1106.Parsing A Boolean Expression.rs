impl Solution {
    /// Parses and evaluates a boolean expression recursively.
    ///
    /// # Intuition
    /// The expression is a nested structure of `!`, `&`, `|` operators
    /// with `t`/`f` literals. Recursive descent parsing handles nesting.
    ///
    /// # Approach
    /// DFS parser consuming bytes. On `t`/`f`, return the literal. On
    /// `!`, evaluate the inner expression and negate. On `&`/`|`, evaluate
    /// all comma-separated sub-expressions and aggregate with `all`/`any`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) recursion depth
    pub fn parse_bool_expr(expression: String) -> bool {
        fn parse(expr: &[u8], pos: &mut usize) -> Vec<bool> {
            let mut result = Vec::new();
            while *pos < expr.len() {
                match expr[*pos] {
                    b')' => {
                        *pos += 1;
                        break;
                    }
                    b't' => {
                        *pos += 1;
                        result.push(true);
                    }
                    b'f' => {
                        *pos += 1;
                        result.push(false);
                    }
                    b'!' => {
                        *pos += 1;
                        result.push(!parse(expr, pos)[0]);
                    }
                    b'&' => {
                        *pos += 1;
                        result.push(parse(expr, pos).iter().all(|&v| v));
                    }
                    b'|' => {
                        *pos += 1;
                        result.push(parse(expr, pos).iter().any(|&v| v));
                    }
                    _ => *pos += 1,
                }
            }
            result
        }

        let mut pos = 0;
        parse(expression.as_bytes(), &mut pos)[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert!(!Solution::parse_bool_expr("!(f)".to_string()));
    }

    #[test]
    fn test_or() {
        assert!(Solution::parse_bool_expr("|(f,t)".to_string()));
    }

    #[test]
    fn test_and() {
        assert!(!Solution::parse_bool_expr("&(t,f)".to_string()));
    }

    #[test]
    fn test_nested() {
        assert!(!Solution::parse_bool_expr("|(&(t,f,t),!(t))".to_string()));
    }
}
