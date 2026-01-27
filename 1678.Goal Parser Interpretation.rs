impl Solution {
    /// Simple string replacement for Goal Parser.
    ///
    /// # Intuition
    /// The parser replaces `()` with `o` and `(al)` with `al`, leaving `G`
    /// unchanged. Two sequential replacements handle all cases.
    ///
    /// # Approach
    /// 1. Replace `()` with `o`
    /// 2. Replace `(al)` with `al`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn interpret(command: String) -> String {
        command.replace("()", "o").replace("(al)", "al")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_g() {
        assert_eq!(Solution::interpret("G()(al)".to_string()), "Goal");
    }

    #[test]
    fn repeated() {
        assert_eq!(Solution::interpret("G()()()()(al)".to_string()), "Gooooal");
    }

    #[test]
    fn mixed() {
        assert_eq!(
            Solution::interpret("(al)G(al)()()G".to_string()),
            "alGalooG"
        );
    }
}
