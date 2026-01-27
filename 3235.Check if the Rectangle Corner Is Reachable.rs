impl Solution {
    /// Check if the rectangle corner (x_corner, y_corner) is reachable from (0, 0).
    ///
    /// # Intuition
    /// Circles may block the path from the origin to the target corner. If a chain
    /// of overlapping circles connects the left/top boundary to the right/bottom
    /// boundary, the path is blocked. We also fail if the origin or target lies
    /// inside any circle.
    ///
    /// # Approach
    /// 1. Define helpers: point-in-circle, circle-touches-left/top, circle-touches-right/bottom.
    /// 2. Check if origin or target corner is inside any circle.
    /// 3. DFS from each circle touching the left/top boundary to find if it connects
    ///    to a circle touching the right/bottom boundary via overlapping circles whose
    ///    intersection point lies within the rectangle.
    /// 4. Return false if such a blocking chain exists.
    ///
    /// # Complexity
    /// - Time: O(n^2) for pairwise circle intersection checks
    /// - Space: O(n) for visited array and recursion stack
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let n = circles.len();
        let mut vis = vec![false; n];

        let in_circle = |px: i64, py: i64, cx: i64, cy: i64, r: i64| -> bool {
            (px - cx) * (px - cx) + (py - cy) * (py - cy) <= r * r
        };

        let touches_left_top = |cx: i64, cy: i64, r: i64| -> bool {
            let xc = x_corner as i64;
            let yc = y_corner as i64;
            (cx.abs() <= r && cy >= 0 && cy <= yc) || ((cy - yc).abs() <= r && cx >= 0 && cx <= xc)
        };

        let touches_right_bottom = |cx: i64, cy: i64, r: i64| -> bool {
            let xc = x_corner as i64;
            let yc = y_corner as i64;
            ((cx - xc).abs() <= r && cy >= 0 && cy <= yc) || (cy.abs() <= r && cx >= 0 && cx <= xc)
        };

        fn dfs(
            circles: &[Vec<i32>],
            vis: &mut [bool],
            i: usize,
            x_corner: i32,
            y_corner: i32,
            touches_rb: &dyn Fn(i64, i64, i64) -> bool,
        ) -> bool {
            let (x1, y1, r1) = (
                circles[i][0] as i64,
                circles[i][1] as i64,
                circles[i][2] as i64,
            );

            if touches_rb(x1, y1, r1) {
                return true;
            }

            vis[i] = true;
            let xc = x_corner as i64;
            let yc = y_corner as i64;

            (0..circles.len()).filter(|&j| !vis[j]).any(|j| {
                let (x2, y2, r2) = (
                    circles[j][0] as i64,
                    circles[j][1] as i64,
                    circles[j][2] as i64,
                );
                let dist_sq = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
                let radii_sq = (r1 + r2) * (r1 + r2);

                dist_sq <= radii_sq
                    && x1 * r2 + x2 * r1 < (r1 + r2) * xc
                    && y1 * r2 + y2 * r1 < (r1 + r2) * yc
                    && dfs(circles, vis, j, x_corner, y_corner, touches_rb)
            })
        }

        for i in 0..n {
            let (cx, cy, cr) = (
                circles[i][0] as i64,
                circles[i][1] as i64,
                circles[i][2] as i64,
            );

            if in_circle(0, 0, cx, cy, cr)
                || in_circle(x_corner as i64, y_corner as i64, cx, cy, cr)
            {
                return false;
            }

            if !vis[i]
                && touches_left_top(cx, cy, cr)
                && dfs(
                    &circles,
                    &mut vis,
                    i,
                    x_corner,
                    y_corner,
                    &touches_right_bottom,
                )
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reachable_no_blocking() {
        assert!(Solution::can_reach_corner(3, 4, vec![vec![2, 1, 1]]));
    }

    #[test]
    fn blocked_by_circle_chain() {
        assert!(!Solution::can_reach_corner(3, 3, vec![vec![1, 1, 2]]));
    }

    #[test]
    fn origin_inside_circle() {
        assert!(!Solution::can_reach_corner(4, 4, vec![vec![0, 0, 1]]));
    }
}
