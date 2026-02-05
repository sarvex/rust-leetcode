impl Solution {
    /// Single loop: one read per tower, one (quality, -x, -y) comparison per reachable tower.
    ///
    /// # Intuition
    /// One pass over towers; for each reachable tower, compare (quality, -x, -y) with the current
    /// best and update when strictly greater. No iterator/closure overhead, no repeated comparisons.
    ///
    /// # Approach
    /// Extract center once. Loop over towers; skip when Manhattan distance > radius. Otherwise
    /// compare ordering key (quality_factor, -tower_x, -tower_y) with best and replace when better.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
        let center_x = center[0];
        let center_y = center[1];
        let mut best_coord_x = -1;
        let mut best_coord_y = -1;
        let mut best_quality = -1i32;

        for tower in towers.iter() {
            let tower_x = tower[0];
            let tower_y = tower[1];
            let quality_factor = tower[2];
            let manhattan_distance = (tower_x - center_x).abs() + (tower_y - center_y).abs();
            if manhattan_distance > radius {
                continue;
            }
            let ordering_key = (quality_factor, -tower_x, -tower_y);
            let beats_best =
                best_quality < 0 || ordering_key > (best_quality, -best_coord_x, -best_coord_y);
            if beats_best {
                best_quality = quality_factor;
                best_coord_x = tower_x;
                best_coord_y = tower_y;
            }
        }

        if best_quality < 0 {
            vec![-1, -1]
        } else {
            vec![best_coord_x, best_coord_y]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]],
                vec![1, 1],
                2
            ),
            vec![3, 1]
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![1, 3, 4], vec![2, 2, 4], vec![4, 4, 7]],
                vec![0, 0],
                5
            ),
            vec![1, 3]
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::best_tower(vec![vec![5, 6, 8], vec![0, 3, 5]], vec![1, 2], 1),
            vec![-1, -1]
        );
    }

    #[test]
    fn test_tie_lexicographic() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![2, 2, 4], vec![1, 3, 4], vec![1, 2, 4]],
                vec![0, 0],
                10
            ),
            vec![1, 2]
        );
    }

    #[test]
    fn test_radius_zero() {
        assert_eq!(
            Solution::best_tower(vec![vec![1, 1, 5], vec![1, 2, 10]], vec![1, 1], 0),
            vec![1, 1]
        );
    }

    #[test]
    fn test_quality_zero() {
        assert_eq!(
            Solution::best_tower(vec![vec![2, 2, 0]], vec![2, 2], 0),
            vec![2, 2]
        );
    }
}
