use std::collections::HashSet;

impl Solution {
    /// Count artifacts fully uncovered by dig operations on an n×n grid.
    ///
    /// # Intuition
    /// An artifact can be extracted only when every cell it covers has been dug.
    /// Use a HashSet for O(1) cell lookup.
    ///
    /// # Approach
    /// 1. Store all dug cells as linearized indices in a HashSet.
    /// 2. For each artifact, check whether all its cells are in the set.
    ///
    /// # Complexity
    /// - Time: O(d + a * k) where d is dig count, a is artifact count, k is avg artifact area
    /// - Space: O(d)
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let dug: HashSet<i32> = dig.iter().map(|p| p[0] * n + p[1]).collect();

        artifacts
            .iter()
            .filter(|a| (a[0]..=a[2]).all(|x| (a[1]..=a[3]).all(|y| dug.contains(&(x * n + y)))))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_artifact_extracted() {
        assert_eq!(
            Solution::dig_artifacts(2, vec![vec![0, 0, 0, 0]], vec![vec![0, 0]],),
            1
        );
    }

    #[test]
    fn partial_dig() {
        assert_eq!(
            Solution::dig_artifacts(2, vec![vec![0, 0, 1, 1]], vec![vec![0, 0], vec![0, 1]],),
            0
        );
    }
}
