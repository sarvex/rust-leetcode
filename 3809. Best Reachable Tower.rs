impl Solution {
    /// Scan reachable towers and keep the best by quality, then lexicographic order.
    ///
    /// # Intuition
    /// Each tower's reachability and quality are independent, so a single pass can pick the best
    /// reachable candidate while applying the tie-break rule.
    ///
    /// # Approach
    /// Iterate through all towers, compute Manhattan distance to `center`, and skip unreachable
    /// ones. Track the best `(quality, x, y)` seen so far, updating when quality improves or when
    /// a lexicographically smaller coordinate ties on quality.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
        let cx = center[0];
        let cy = center[1];
        let mut best_quality = -1;
        let mut best_x = -1;
        let mut best_y = -1;

        for tower in towers.iter() {
            let x = tower[0];
            let y = tower[1];
            let quality = tower[2];
            let distance = (x - cx).abs() + (y - cy).abs();
            if distance > radius {
                continue;
            }

            let is_better = quality > best_quality
                || (quality == best_quality && (x < best_x || (x == best_x && y < best_y)));
            if is_better {
                best_quality = quality;
                best_x = x;
                best_y = y;
            }
        }

        if best_quality < 0 {
            vec![-1, -1]
        } else {
            vec![best_x, best_y]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::best_tower(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], vec![1, 1], 2),
            vec![3, 1]
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::best_tower(vec![vec![1, 3, 4], vec![2, 2, 4], vec![4, 4, 7]], vec![0, 0], 5),
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
