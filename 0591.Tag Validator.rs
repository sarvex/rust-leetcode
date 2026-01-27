impl Solution {
    /// Validates an XML-like tag structure using a stack-based parser.
    ///
    /// # Intuition
    /// Track open tags with a stack. Handle CDATA sections, opening tags,
    /// and closing tags in order. The code is valid if all tags are properly
    /// nested and the stack is empty at the end.
    ///
    /// # Approach
    /// 1. Scan through the string, identifying CDATA, closing tags, and opening tags.
    /// 2. For CDATA, skip to the matching `]]>`.
    /// 3. For opening tags, push the tag name.
    /// 4. For closing tags, pop and verify the match.
    /// 5. Ensure the stack is never empty mid-parse (content must be wrapped).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the tag stack
    pub fn is_valid(code: String) -> bool {
        fn check_tag(tag: &str) -> bool {
            let n = tag.len();
            n >= 1 && n <= 9 && tag.bytes().all(|b| b.is_ascii_uppercase())
        }

        let mut stack: Vec<&str> = Vec::new();
        let mut i = 0;
        while i < code.len() {
            if i > 0 && stack.is_empty() {
                return false;
            }
            if code[i..].starts_with("<![CDATA[") {
                match code[i + 9..].find("]]>") {
                    Some(n) => i += n + 12,
                    None => return false,
                }
            } else if code[i..].starts_with("</") {
                let j = i + 2;
                match code[j..].find('>') {
                    Some(n) => {
                        let tag = &code[j..j + n];
                        if !check_tag(tag) || stack.is_empty() || stack.pop().unwrap() != tag {
                            return false;
                        }
                        i = j + n + 1;
                    }
                    None => return false,
                }
            } else if code[i..].starts_with('<') {
                let j = i + 1;
                match code[j..].find('>') {
                    Some(n) => {
                        let tag = &code[j..j + n];
                        if !check_tag(tag) {
                            return false;
                        }
                        stack.push(tag);
                        i = j + n + 1;
                    }
                    None => return false,
                }
            } else {
                i += 1;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(Solution::is_valid(
            "<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()
        ));
    }

    #[test]
    fn test_invalid_cdata() {
        assert!(!Solution::is_valid(
            "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string()
        ));
    }

    #[test]
    fn test_invalid_tag() {
        assert!(!Solution::is_valid("<A>  <B> </A>   </B>".to_string()));
    }
}
