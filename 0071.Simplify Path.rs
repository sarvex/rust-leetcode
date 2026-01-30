impl Solution {
    /// Stack-based simplification of Unix-style file paths.
    ///
    /// # Intuition
    /// Treat the output path as a stack of segments. Appending a segment
    /// pushes, and ".." pops by truncating back to the previous length.
    ///
    /// # Approach
    /// Preallocate an output buffer and a stack of segment start offsets.
    /// Split the path on '/'. Skip "" and ".", pop on ".." via `truncate`,
    /// and otherwise push the current length then append "/segment". If no
    /// segments were added, return "/".
    ///
    /// # Complexity
    /// - Time: O(n) — linear scans for capacity and parsing
    /// - Space: O(n) — output buffer and segment stack
    pub fn simplify_path(path: String) -> String {
        let segment_capacity = path
            .as_bytes()
            .iter()
            .filter(|&&byte| byte == b'/')
            .count()
            .saturating_add(1);
        let mut segment_starts: Vec<usize> = Vec::with_capacity(segment_capacity);
        let mut result = String::with_capacity(path.len().saturating_add(1));

        for component in path.split('/') {
            match component {
                "" | "." => {}
                ".." => {
                    if let Some(start) = segment_starts.pop() {
                        result.truncate(start);
                    }
                }
                name => {
                    segment_starts.push(result.len());
                    result.push('/');
                    result.push_str(name);
                }
            }
        }

        if result.is_empty() {
            "/".to_string()
        } else {
            result
        }
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
