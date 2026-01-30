
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
            Solution::min_distance(5, 7, vec![2, 2], vec![4, 4], vec![vec![3, 0], vec![2, 5]]),
            12
        );
    }

    #[test]
    fn test_single_nut() {
        // Squirrel at (0,0), tree at (1,1), nut at (2,2)
        // Distance: squirrel->nut (4) + nut->tree (2) = 6
        assert_eq!(
            Solution::min_distance(3, 3, vec![1, 1], vec![0, 0], vec![vec![2, 2]]),
            6
        );
    }

    #[test]
    fn test_squirrel_at_tree() {
        // Squirrel at tree position, should just do round trips
        // Nut at (0,0), tree at (1,1) -> round trip = 4
        // Nut at (2,2), tree at (1,1) -> round trip = 4
        // Squirrel starts at (1,1), same as tree
        // First nut picked saves: tree_dist - squirrel_dist = 2 - 2 = 0 for (0,0)
        // OR 2 - 2 = 0 for (2,2)
        // Total = 8 - 0 = 8
        assert_eq!(
            Solution::min_distance(3, 3, vec![1, 1], vec![1, 1], vec![vec![0, 0], vec![2, 2]]),
            8
        );
    }

    #[test]
    fn test_squirrel_closer_to_nut() {
        // Tree at (0,0), squirrel at (5,5)
        // Nut 1 at (1,0): tree_dist=1, squirrel_dist=9, savings=-8
        // Nut 2 at (5,4): tree_dist=9, squirrel_dist=1, savings=8
        // Round trip = 2*(1+9) = 20
        // Best savings = 8 (pick nut 2 first)
        // Total = 20 - 8 = 12
        assert_eq!(
            Solution::min_distance(6, 6, vec![0, 0], vec![5, 5], vec![vec![1, 0], vec![5, 4]]),
            12
        );
    }

    #[test]
    fn test_multiple_nuts() {
        // Tree at (2,2), squirrel at (0,0)
        // Nuts at: (0,2), (2,0), (4,2), (2,4)
        // Round trip = 2*(2+2+2+2) = 16
        // Savings for (0,2): tree_dist=2, squirrel_dist=2, savings=0
        // Savings for (2,0): tree_dist=2, squirrel_dist=2, savings=0
        // Savings for (4,2): tree_dist=2, squirrel_dist=6, savings=-4
        // Savings for (2,4): tree_dist=2, squirrel_dist=6, savings=-4
        // Best savings = 0
        // Total = 16 - 0 = 16
        assert_eq!(
            Solution::min_distance(
                5,
                5,
                vec![2, 2],
                vec![0, 0],
                vec![vec![0, 2], vec![2, 0], vec![4, 2], vec![2, 4]]
            ),
            16
        );
    }
}
