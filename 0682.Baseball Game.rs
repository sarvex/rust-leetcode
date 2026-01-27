impl Solution {
    /// Simulates a baseball score record using a stack.
    ///
    /// # Intuition
    /// Process operations sequentially: numbers push scores, "+" sums the
    /// last two, "D" doubles the last, and "C" removes the last.
    ///
    /// # Approach
    /// 1. Use a Vec as a stack.
    /// 2. Match each operation and modify the stack.
    /// 3. Sum all remaining scores.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for op in &operations {
            match op.as_str() {
                "+" => {
                    let n = stack.len();
                    stack.push(stack[n - 1] + stack[n - 2]);
                }
                "D" => stack.push(stack.last().unwrap() * 2),
                "C" => {
                    stack.pop();
                }
                num => stack.push(num.parse().unwrap()),
            }
        }
        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".into(),
                "2".into(),
                "C".into(),
                "D".into(),
                "+".into()
            ]),
            30
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".into(),
                "-2".into(),
                "4".into(),
                "C".into(),
                "D".into(),
                "9".into(),
                "+".into(),
                "+".into()
            ]),
            27
        );
    }
}
