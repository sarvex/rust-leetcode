impl Solution {
    /// Sorts people's names by their heights in descending order.
    ///
    /// # Intuition
    /// Pair names with heights, sort by height descending, then extract names.
    ///
    /// # Approach
    /// 1. Zip names and heights into tuples
    /// 2. Sort unstably by height in descending order
    /// 3. Map to extract only the names
    ///
    /// # Complexity
    /// - Time: O(n log n) — dominated by sorting
    /// - Space: O(n) — for the combined vector
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut pairs: Vec<(String, i32)> = names.into_iter().zip(heights).collect();
        pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        pairs.into_iter().map(|(name, _)| name).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        assert_eq!(
            Solution::sort_people(names, heights),
            vec!["Mary", "Emma", "John"]
        );
    }

    #[test]
    fn test_example_2() {
        let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
        let heights = vec![155, 185, 150];
        assert_eq!(
            Solution::sort_people(names, heights),
            vec!["Bob", "Alice", "Bob"]
        );
    }

    #[test]
    fn test_single_person() {
        let names = vec!["Solo".to_string()];
        let heights = vec![170];
        assert_eq!(Solution::sort_people(names, heights), vec!["Solo"]);
    }
}
