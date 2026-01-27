impl Solution {
    /// Track folder depth via simple counter simulation.
    ///
    /// # Intuition
    /// `../` moves up (decrement depth, min 0), `./` stays, anything else
    /// moves deeper. The final depth is the minimum operations to return.
    ///
    /// # Approach
    /// 1. Fold over logs, adjusting depth per operation type
    /// 2. Return final depth
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |depth, log| match log.as_str() {
            "../" => (depth - 1).max(0),
            "./" => depth,
            _ => depth + 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn nested_then_back() {
        assert_eq!(
            Solution::min_operations(s(&["d1/", "d2/", "../", "d21/", "./"])),
            2
        );
    }

    #[test]
    fn back_beyond_root() {
        assert_eq!(
            Solution::min_operations(s(&["d1/", "../", "../", "../"])),
            0
        );
    }

    #[test]
    fn deep_nesting() {
        assert_eq!(Solution::min_operations(s(&["d1/", "d2/", "d3/"])), 3);
    }
}
