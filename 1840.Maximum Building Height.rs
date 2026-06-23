impl Solution {
    /// Greedy two-pass sweep over sorted restriction checkpoints.
    ///
    /// # Intuition
    /// Between two checkpoints at positions `i` and `j` with allowed heights `hi` and `hj`,
    /// the tallest peak we can form is a triangle that rises from both ends simultaneously.
    /// The peak height equals `(hi + hj + (j - i)) / 2`.
    ///
    /// # Approach
    /// 1. Add a synthetic restriction `[1, 0]` (building 1 must be height 0) and sort by id.
    /// 2. Forward pass: clamp each restriction so it is reachable from the previous one,
    ///    i.e., `max_height[i] = min(max_height[i], max_height[i-1] + gap)`.
    /// 3. Backward pass: same propagation right-to-left to handle downstream constraints.
    /// 4. Scan adjacent pairs and compute the midpoint peak; also check from the last
    ///    restriction to building `n`.
    ///
    /// # Complexity
    /// - Time: O(m log m) where m = restrictions.length
    /// - Space: O(m)
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut pts: Vec<(i64, i64)> = restrictions
            .iter()
            .map(|r| (r[0] as i64, r[1] as i64))
            .collect();
        pts.push((1, 0));
        pts.sort_unstable();

        let m = pts.len();

        // Forward pass: each checkpoint cannot be higher than what is reachable from the left.
        for i in 1..m {
            let gap = pts[i].0 - pts[i - 1].0;
            pts[i].1 = pts[i].1.min(pts[i - 1].1 + gap);
        }

        // Backward pass: propagate constraints back from the right.
        for i in (0..m - 1).rev() {
            let gap = pts[i + 1].0 - pts[i].0;
            pts[i].1 = pts[i].1.min(pts[i + 1].1 + gap);
        }

        // Find the maximum peak between every pair of adjacent checkpoints.
        let mut ans = pts
            .windows(2)
            .map(|w| {
                let (pos_l, h_l) = w[0];
                let (pos_r, h_r) = w[1];
                let gap = pos_r - pos_l;
                // Peak = lower base + distance to meet in the middle
                (h_l + h_r + gap) / 2
            })
            .max()
            .unwrap_or(0);

        // From the last checkpoint to building n (no upper bound on the right).
        let (last_pos, last_h) = pts[m - 1];
        ans = ans.max(last_h + (n as i64 - last_pos));

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // Heights [0,1,2,1,2] → max = 2
        assert_eq!(Solution::max_building(5, vec![vec![2, 1], vec![4, 1]]), 2);
    }

    #[test]
    fn test_example2() {
        // No restrictions → can reach height n-1 = 5
        assert_eq!(Solution::max_building(6, vec![]), 5);
    }

    #[test]
    fn test_example3() {
        // Heights [0,1,2,3,3,4,4,5,4,3] → max = 5
        assert_eq!(
            Solution::max_building(10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]]),
            5
        );
    }

    #[test]
    fn test_two_buildings_no_restrictions() {
        // n=2, no restrictions → heights [0,1] → max = 1
        assert_eq!(Solution::max_building(2, vec![]), 1);
    }

    #[test]
    fn test_single_tight_restriction_at_end() {
        // n=5, restriction [5,0] forces all to be 0 except building 1 which is already 0
        // Possible: [0,0,0,0,0] → max = 0? No: restriction says ≤0 and adjacency forces descent.
        // Actually [0,0,0,0,0] is valid → max = 0
        assert_eq!(Solution::max_building(5, vec![vec![5, 0]]), 0);
    }

    #[test]
    fn test_large_n_no_restrictions() {
        // n = 10^9, no restrictions → max = n - 1
        assert_eq!(Solution::max_building(1_000_000_000, vec![]), 999_999_999);
    }

    #[test]
    fn test_restriction_at_second_building() {
        // Building 2 max 0 → heights must be [0,0,1,2,...] → gap freed after pos 2
        // n=5, restrictions=[[2,0]] → max reachable at 5 = 0 + (5-2) = 3
        assert_eq!(Solution::max_building(5, vec![vec![2, 0]]), 3);
    }

    #[test]
    fn test_restriction_higher_than_reachable() {
        // maxHeight much larger than the gap allows — constraint is naturally non-binding
        assert_eq!(
            Solution::max_building(5, vec![vec![3, 100]]),
            // From pos 1 (h=0) to pos 3 gap=2, reachable=2; from pos 3 (h=2) to 5 gap=2 → 4
            4
        );
    }
}
