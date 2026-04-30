impl Solution {
    /// Binary search on the minimum distance combined with binary lifting on the perimeter.
    ///
    /// # Intuition
    /// Every point lies on the boundary of the square, so we can "unroll" the boundary into a
    /// 1D perimeter of length `4 * side`. Walking clockwise starting from `(0, 0)`:
    /// - bottom edge `(x, 0)` -> `x`
    /// - right edge  `(side, y)` -> `side + y`
    /// - top edge    `(x, side)` -> `2*side + (side - x)`
    /// - left edge   `(0, y)` -> `3*side + (side - y)`
    ///
    /// For any two boundary points on the square, the Manhattan distance is at least
    /// `min(gap, 4*side - gap)` where `gap` is the (absolute) perimeter distance, and this bound
    /// is tight when both points are far enough to not share an edge shortcut. Critically for the
    /// greedy check, if `d` fits inside a single side (`d <= side`), perimeter distance equals
    /// Manhattan distance for neighbors on the same side, and always dominates Manhattan distance
    /// otherwise. So if we can select `k` points with pairwise perimeter gap `>= d` on the cycle,
    /// then Manhattan distance between them is `>= d`. Conversely, the Manhattan optimum never
    /// exceeds `2*side`, and for feasible `d <= 2*side` perimeter gap is an exact lower bound on
    /// Manhattan distance (since the Manhattan shortcut across the square is at most the perimeter
    /// gap for points on the boundary).
    ///
    /// # Approach
    /// 1. Sort points by their perimeter position `p[i]`.
    /// 2. Binary search the answer `d` in `[1, 2*side]`.
    /// 3. For a fixed `d`, precompute `nxt[i]` = smallest `j` such that `p[j] - p[i] >= d` on the
    ///    doubled array (to handle the circular wrap). Use binary lifting to jump `k-1` steps from
    ///    each starting index in `O(log k)`; feasibility holds if some start can reach `k-1` steps
    ///    while staying within perimeter `p[i] + 4*side - d` (ensuring the wrap-around gap is
    ///    also `>= d`).
    ///
    /// # Complexity
    /// - Time: `O(n log n + n log(side) log k)` where `n = points.len()`.
    /// - Space: `O(n log k)` for the binary-lifting table.
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let s = side as i64;
        let perim = 4 * s;
        let k = k as usize;

        // Map each point to its perimeter coordinate (clockwise from (0,0)).
        let mut coords: Vec<i64> = points
            .iter()
            .map(|p| {
                let (x, y) = (p[0] as i64, p[1] as i64);
                if y == 0 {
                    x
                } else if x == s {
                    s + y
                } else if y == s {
                    2 * s + (s - x)
                } else {
                    3 * s + (s - y)
                }
            })
            .collect();
        coords.sort_unstable();

        let n = coords.len();
        // Doubled array to handle circular wraparound.
        let mut doubled = Vec::with_capacity(2 * n);
        doubled.extend_from_slice(&coords);
        for &c in &coords {
            doubled.push(c + perim);
        }

        // Number of lift levels: enough bits to cover k-1 hops.
        let max_level = (usize::BITS - (k - 1).leading_zeros()) as usize;
        let max_level = max_level.max(1);

        // Feasibility check: can we pick k points with pairwise circular gap >= d?
        let feasible = |d: i64| -> bool {
            if d <= 0 {
                return true;
            }
            let m = 2 * n;
            // nxt[i] = smallest index j in doubled with doubled[j] >= doubled[i] + d.
            let mut nxt = vec![m; m];
            let (mut i, mut j) = (0usize, 0usize);
            while i < m {
                if j < i {
                    j = i;
                }
                while j < m && doubled[j] - doubled[i] < d {
                    j += 1;
                }
                nxt[i] = j;
                i += 1;
            }

            // Binary lifting: lift[l][i] = position after 2^l hops from i.
            let mut lift: Vec<Vec<usize>> = vec![nxt];
            for l in 1..max_level {
                let prev = &lift[l - 1];
                let mut cur = vec![m; m];
                for i in 0..m {
                    let mid = prev[i];
                    cur[i] = if mid >= m { m } else { prev[mid] };
                }
                lift.push(cur);
            }

            // For each starting point in the original n, try to take k-1 more hops.
            let steps_needed = k - 1;
            for start in 0..n {
                let mut pos = start;
                let mut remaining = steps_needed;
                for l in (0..max_level).rev() {
                    if remaining & (1 << l) != 0 {
                        pos = lift[l][pos];
                        if pos >= m {
                            break;
                        }
                        remaining &= !(1 << l);
                    }
                }
                if pos < m && doubled[pos] - doubled[start] + d <= perim {
                    return true;
                }
            }
            false
        };

        // Binary search the largest feasible d in [1, 2*side].
        let (mut lo, mut hi) = (1i64, 2 * s);
        let mut ans = 0i64;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if feasible(mid) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let side = 2;
        let points = vec![vec![0, 2], vec![2, 0], vec![2, 2], vec![0, 0]];
        assert_eq!(Solution::max_distance(side, points, 4), 2);
    }

    #[test]
    fn test_example_2() {
        let side = 2;
        let points = vec![vec![0, 0], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]];
        assert_eq!(Solution::max_distance(side, points, 4), 1);
    }

    #[test]
    fn test_example_3() {
        let side = 2;
        let points = vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 2],
            vec![2, 0],
            vec![2, 2],
            vec![2, 1],
        ];
        assert_eq!(Solution::max_distance(side, points, 5), 1);
    }

    #[test]
    fn test_four_corners_large_side() {
        let side = 1_000_000_000;
        let points = vec![vec![0, 0], vec![0, side], vec![side, 0], vec![side, side]];
        // Adjacent corners are Manhattan distance `side` apart (opposite corners are 2*side).
        // With k=4, we must pick all four, so the minimum pairwise distance is `side`.
        assert_eq!(Solution::max_distance(side, points, 4), side);
    }

    #[test]
    fn test_k_equals_four_on_single_edge_possible() {
        // Points spread around the perimeter; pick 4 maximizing min distance.
        let side = 4;
        let points = vec![
            vec![0, 0],
            vec![4, 0],
            vec![4, 4],
            vec![0, 4],
            vec![2, 0],
            vec![0, 2],
        ];
        // The four corners give pairwise Manhattan distance 8 (between opposite corners)
        // but adjacent corners are 4 apart; so min over the 4 selected corners is 4.
        assert_eq!(Solution::max_distance(side, points, 4), 4);
    }

    #[test]
    fn test_many_collinear_boundary_points() {
        // All points on bottom edge; with k=4 on an edge of length side=3,
        // best selection is equally spaced -> min gap = 1.
        let side = 3;
        let points = vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0]];
        assert_eq!(Solution::max_distance(side, points, 4), 1);
    }
}
