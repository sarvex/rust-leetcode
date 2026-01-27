impl Solution {
    /// Stack-based simplification of Unix-style file paths.
    ///
    /// # Intuition
    /// Split the path by '/' to extract components. A stack naturally handles
    /// directory traversal: push on valid names, pop on '..', and ignore
    /// '.' and empty segments.
    ///
    /// # Approach
    /// Split the path string on '/'. For each component, match against
    /// ".." (pop), "." or "" (skip), and anything else (push). Join the
    /// stack with '/' and prepend the root.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the path
    /// - Space: O(n) — stack and split components
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();

        for component in path.split('/') {
            match component {
                ".." => {
                    stack.pop();
                }
                "" | "." => {}
                name => stack.push(name),
            }
        }

        format!("/{}", stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_path() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn double_dot() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    }

    #[test]
    fn multiple_slashes() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo"
        );
    }

    #[test]
    fn complex_path() {
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c");
    }

    #[test]
    fn root_only() {
        assert_eq!(Solution::simplify_path("/".to_string()), "/");
    }
}
