impl Solution {
    /// Simulate push/pop operations to build the target array from a stream.
    ///
    /// # Intuition
    /// Numbers arrive sequentially 1..n. For each target element, push-and-pop
    /// all skipped numbers, then push the matching number.
    ///
    /// # Approach
    /// 1. Track the current stream value
    /// 2. For each target element, emit "Push"/"Pop" for skipped values
    /// 3. Emit "Push" for the matching value
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = 1;

        for &num in &target {
            while current < num {
                result.push("Push".to_string());
                result.push("Pop".to_string());
                current += 1;
            }
            result.push("Push".to_string());
            current += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_gaps() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec!["Push", "Push", "Pop", "Push"]
        );
    }

    #[test]
    fn consecutive() {
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push", "Push", "Push"]
        );
    }

    #[test]
    fn single_target() {
        assert_eq!(Solution::build_array(vec![1, 2], 4), vec!["Push", "Push"]);
    }
}
