impl Solution {
    /// Evaluates a Reverse Polish Notation expression using a stack.
    ///
    /// # Intuition
    /// RPN naturally maps to stack evaluation: push operands, and when an operator
    /// is encountered, pop two operands, apply the operator, and push the result.
    ///
    /// # Approach
    /// 1. Iterate through tokens.
    /// 2. If a token parses as a number, push it onto the stack.
    /// 3. Otherwise, pop two operands and apply the operator.
    /// 4. The final stack element is the result.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of tokens
    /// - Space: O(n) for the stack
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len());
        for token in &tokens {
            match token.parse::<i32>() {
                Ok(num) => stack.push(num),
                Err(_) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => unreachable!(),
                    });
                }
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_tokens(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn simple_addition() {
        assert_eq!(Solution::eval_rpn(to_tokens(&["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn division_truncates_toward_zero() {
        assert_eq!(
            Solution::eval_rpn(to_tokens(&["4", "13", "5", "/", "+"])),
            6
        );
    }

    #[test]
    fn complex_expression() {
        assert_eq!(
            Solution::eval_rpn(to_tokens(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ])),
            22
        );
    }
}
