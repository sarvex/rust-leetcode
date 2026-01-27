use std::collections::HashSet;

impl Solution {
    /// Set difference to find the city with no outgoing path.
    ///
    /// # Intuition
    /// The destination city appears only as a destination, never as a source.
    /// Collecting all source cities into a set and finding the destination
    /// not in that set identifies the answer.
    ///
    /// # Approach
    /// 1. Collect all source cities into a HashSet
    /// 2. Find the destination city not present in the source set
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the source set
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let sources: HashSet<&str> = paths.iter().map(|p| p[0].as_str()).collect();
        paths
            .iter()
            .find(|p| !sources.contains(p[1].as_str()))
            .unwrap()[1]
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn simple_chain() {
        assert_eq!(
            Solution::dest_city(vec![
                vec![s("London"), s("New York")],
                vec![s("New York"), s("Lima")],
                vec![s("Lima"), s("Sao Paulo")],
            ]),
            "Sao Paulo"
        );
    }

    #[test]
    fn two_cities() {
        assert_eq!(Solution::dest_city(vec![vec![s("A"), s("B")]]), "B");
    }
}
