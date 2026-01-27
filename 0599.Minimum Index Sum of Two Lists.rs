use std::collections::HashMap;

impl Solution {
    /// Finds restaurants with the minimum index sum from two preference lists.
    ///
    /// # Intuition
    /// Index one list in a hash map, then scan the other to find common
    /// entries with the smallest total index.
    ///
    /// # Approach
    /// 1. Map list2 entries to their indices.
    /// 2. For each list1 entry found in the map, compute the index sum.
    /// 3. Track and return all entries matching the minimum index sum.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(n)
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let index_map: HashMap<&str, usize> = list2
            .iter()
            .enumerate()
            .map(|(i, s)| (s.as_str(), i))
            .collect();

        let mut result = Vec::new();
        let mut min_sum = usize::MAX;
        for (i, s) in list1.iter().enumerate() {
            if let Some(&j) = index_map.get(s.as_str()) {
                let sum = i + j;
                match sum.cmp(&min_sum) {
                    std::cmp::Ordering::Less => {
                        min_sum = sum;
                        result = vec![s.clone()];
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(s.clone());
                    }
                    _ => {}
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ],
            ),
            vec!["Shogun"]
        );
    }

    #[test]
    fn test_tie() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ],
            ),
            vec!["Shogun"]
        );
    }
}
