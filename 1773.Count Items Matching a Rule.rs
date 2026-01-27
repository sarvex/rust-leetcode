impl Solution {
    /// Counts items matching a given rule key and value.
    ///
    /// # Intuition
    /// Map the rule key to the corresponding index in each item triple
    /// (type=0, color=1, name=2) and filter-count matches.
    ///
    /// # Approach
    /// 1. Determine the index from the rule key string.
    /// 2. Filter items where the value at that index equals the rule value.
    /// 3. Return the count.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let idx = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            _ => 2,
        };
        items.iter().filter(|item| item[idx] == rule_value).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(val: &str) -> String {
        val.to_string()
    }

    #[test]
    fn test_example_one() {
        let items = vec![
            vec![s("phone"), s("blue"), s("pixel")],
            vec![s("computer"), s("silver"), s("lenovo")],
            vec![s("phone"), s("gold"), s("iphone")],
        ];
        assert_eq!(Solution::count_matches(items, s("color"), s("silver")), 1);
    }

    #[test]
    fn test_example_two() {
        let items = vec![
            vec![s("phone"), s("blue"), s("pixel")],
            vec![s("computer"), s("silver"), s("phone")],
            vec![s("phone"), s("gold"), s("iphone")],
        ];
        assert_eq!(Solution::count_matches(items, s("type"), s("phone")), 2);
    }
}
