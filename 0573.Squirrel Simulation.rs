impl Solution {
    /// Minimizes total travel distance for a squirrel collecting nuts.
    ///
    /// # Intuition
    /// All nuts require a round trip to the tree except the first nut the
    /// squirrel picks up. The optimal first nut maximizes the savings:
    /// tree_dist − squirrel_dist.
    ///
    /// # Approach
    /// 1. Compute total round-trip distance (2 × sum of tree-to-nut distances).
    /// 2. For each nut, compute savings from going there first.
    /// 3. Subtract the maximum savings from the total.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_distance(
        _height: i32,
        _width: i32,
        tree: Vec<i32>,
        squirrel: Vec<i32>,
        nuts: Vec<Vec<i32>>,
    ) -> i32 {
        let round_trip: i32 = nuts
            .iter()
            .map(|n| ((n[0] - tree[0]).abs() + (n[1] - tree[1]).abs()) * 2)
            .sum();
        let max_savings = nuts
            .iter()
            .map(|n| {
                let to_tree = (n[0] - tree[0]).abs() + (n[1] - tree[1]).abs();
                let to_squirrel = (n[0] - squirrel[0]).abs() + (n[1] - squirrel[1]).abs();
                to_tree - to_squirrel
            })
            .max()
            .unwrap_or(0);
        round_trip - max_savings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::min_distance(5, 7, vec![2, 2], vec![4, 4], vec![vec![3, 0], vec![2, 5]],),
            12
        );
    }
}
