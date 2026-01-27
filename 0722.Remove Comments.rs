impl Solution {
    /// Removes line and block comments from source code.
    ///
    /// # Intuition
    /// Parse character by character, toggling a block-comment state flag.
    /// Line comments discard the rest of the line; block comments span
    /// across lines until the closing `*/`.
    ///
    /// # Approach
    /// Iterate over each source line, scanning two characters at a time to
    /// detect `//`, `/*`, and `*/`. Accumulate non-comment characters into
    /// a buffer, flushing to the result when a line ends outside a block comment.
    ///
    /// # Complexity
    /// - Time: O(n) where n is total characters across all lines
    /// - Space: O(n) for the output
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut buffer = String::new();
        let mut in_block = false;

        for line in &source {
            let bytes = line.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                if in_block {
                    if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                        in_block = false;
                        i += 2;
                    } else {
                        i += 1;
                    }
                } else if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    in_block = true;
                    i += 2;
                } else if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
                    break;
                } else {
                    buffer.push(bytes[i] as char);
                    i += 1;
                }
            }
            if !in_block && !buffer.is_empty() {
                result.push(buffer.clone());
                buffer.clear();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_and_line_comments() {
        let source: Vec<String> = vec![
            "/*Test program */",
            "int main()",
            "{ ",
            "  // variable declaration ",
            "int a, b, c;",
            "/* This is a test",
            "   multiline  ",
            "   comment for ",
            "   testing */",
            "b = c;",
            "}",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let expected: Vec<String> = vec!["int main()", "{ ", "  ", "int a, b, c;", "b = c;", "}"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::remove_comments(source), expected);
    }

    #[test]
    fn test_inline_block_comment() {
        let source: Vec<String> = vec!["a/*comment*/b"]
            .into_iter()
            .map(String::from)
            .collect();
        let expected: Vec<String> = vec!["ab"].into_iter().map(String::from).collect();
        assert_eq!(Solution::remove_comments(source), expected);
    }
}
