use std::collections::{HashMap, HashSet};

impl Solution {
    /// Lowest common ancestor in a region hierarchy tree.
    ///
    /// # Intuition
    /// The region containment forms a tree where each region has one parent.
    /// Finding the smallest common region is equivalent to finding the lowest
    /// common ancestor of two nodes. Tracing ancestors of one region into a
    /// set, then walking up from the other until hitting the set, gives the LCA.
    ///
    /// # Approach
    /// 1. Build a child-to-parent map from the region lists
    /// 2. Walk from region1 to root, collecting all ancestors in a HashSet
    /// 3. Walk from region2 upward until an ancestor in the set is found
    ///
    /// # Complexity
    /// - Time: O(n) where n is total region entries
    /// - Space: O(n) for the parent map and ancestor set
    pub fn find_smallest_region(
        regions: Vec<Vec<String>>,
        region1: String,
        region2: String,
    ) -> String {
        let mut parent: HashMap<String, String> = HashMap::new();

        for r in &regions {
            for child in &r[1..] {
                parent.insert(child.clone(), r[0].clone());
            }
        }

        let mut ancestors: HashSet<String> = HashSet::new();
        let mut current = Some(region1);
        while let Some(region) = current {
            ancestors.insert(region.clone());
            current = parent.get(&region).cloned();
        }

        let mut current = Some(region2);
        while let Some(region) = current {
            if ancestors.contains(&region) {
                return region;
            }
            current = parent.get(&region).cloned();
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn common_ancestor() {
        let regions = vec![
            vec![s("Earth"), s("North America"), s("South America")],
            vec![s("North America"), s("United States"), s("Canada")],
            vec![s("United States"), s("New York"), s("Boston")],
            vec![s("Canada"), s("Ontario"), s("Quebec")],
            vec![s("South America"), s("Brazil")],
        ];
        assert_eq!(
            Solution::find_smallest_region(regions, s("Quebec"), s("New York")),
            s("North America")
        );
    }

    #[test]
    fn same_region() {
        let regions = vec![
            vec![s("Earth"), s("North America")],
            vec![s("North America"), s("US")],
        ];
        assert_eq!(
            Solution::find_smallest_region(regions, s("US"), s("US")),
            s("US")
        );
    }
}
