#[derive(Clone, Copy)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

impl Solution {
    /// Determine whether a corner-to-corner path exists.
    ///
    /// # Intuition
    /// A blocking configuration appears when a connected chain of circles intersects
    /// both the left/top boundary group and the right/bottom boundary group. Since the
    /// path cannot touch the rectangle boundary (except at the corners), such a chain
    /// disconnects the two corners.
    ///
    /// # Approach
    /// 1. Reject immediately if either corner lies inside or on any circle.
    /// 2. Mark circles that touch the left/top boundaries and those that touch the
    ///    right/bottom boundaries.
    /// 3. Build a graph where two circles are adjacent only if their overlap region
    ///    intersects the rectangle.
    /// 4. DFS/BFS from any circle touching left/top; if its component touches
    ///    right/bottom, the path is blocked.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n^2)
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let x_corner = x_corner as i64;
        let y_corner = y_corner as i64;
        let circles: Vec<Circle> = circles
            .into_iter()
            .map(|c| Circle {
                x: c[0] as i64,
                y: c[1] as i64,
                r: c[2] as i64,
            })
            .collect();

        if circles
            .iter()
            .any(|c| Self::point_in_circle(0, 0, c) || Self::point_in_circle(x_corner, y_corner, c))
        {
            return false;
        }

        let n = circles.len();
        let mut touches_group1 = vec![false; n];
        let mut touches_group2 = vec![false; n];

        for (i, c) in circles.iter().enumerate() {
            let touches_left = Self::intersects_vertical_segment(c, 0, 0, y_corner);
            let touches_top = Self::intersects_horizontal_segment(c, y_corner, 0, x_corner);
            let touches_right = Self::intersects_vertical_segment(c, x_corner, 0, y_corner);
            let touches_bottom = Self::intersects_horizontal_segment(c, 0, 0, x_corner);

            touches_group1[i] = touches_left || touches_top;
            touches_group2[i] = touches_right || touches_bottom;
        }

        let mut adjacency = vec![Vec::new(); n];
        for i in 0..n {
            for j in (i + 1)..n {
                if Self::circles_overlap_in_rect(&circles[i], &circles[j], x_corner, y_corner) {
                    adjacency[i].push(j);
                    adjacency[j].push(i);
                }
            }
        }

        let mut visited = vec![false; n];
        let mut stack = Vec::new();

        for i in 0..n {
            if touches_group1[i] && !visited[i] {
                let mut hits_group2 = false;
                visited[i] = true;
                stack.push(i);

                while let Some(node) = stack.pop() {
                    if touches_group2[node] {
                        hits_group2 = true;
                        break;
                    }

                    for &next in &adjacency[node] {
                        if !visited[next] {
                            visited[next] = true;
                            stack.push(next);
                        }
                    }
                }

                if hits_group2 {
                    return false;
                }
            }
        }

        true
    }

    const EPS: f64 = 1e-9;

    fn sq_i128(value: i64) -> i128 {
        let value = value as i128;
        value * value
    }

    fn clamp(value: i64, min: i64, max: i64) -> i64 {
        value.max(min).min(max)
    }

    fn point_in_circle(px: i64, py: i64, circle: &Circle) -> bool {
        let dx = px - circle.x;
        let dy = py - circle.y;
        Self::sq_i128(dx) + Self::sq_i128(dy) <= Self::sq_i128(circle.r)
    }

    fn intersects_vertical_segment(circle: &Circle, x_edge: i64, y_min: i64, y_max: i64) -> bool {
        let y = Self::clamp(circle.y, y_min, y_max);
        let dx = circle.x - x_edge;
        let dy = circle.y - y;
        Self::sq_i128(dx) + Self::sq_i128(dy) <= Self::sq_i128(circle.r)
    }

    fn intersects_horizontal_segment(circle: &Circle, y_edge: i64, x_min: i64, x_max: i64) -> bool {
        let x = Self::clamp(circle.x, x_min, x_max);
        let dx = circle.x - x;
        let dy = circle.y - y_edge;
        Self::sq_i128(dx) + Self::sq_i128(dy) <= Self::sq_i128(circle.r)
    }

    fn circle_intersects_rect(circle: &Circle, x_corner: i64, y_corner: i64) -> bool {
        let x = Self::clamp(circle.x, 0, x_corner);
        let y = Self::clamp(circle.y, 0, y_corner);
        let dx = circle.x - x;
        let dy = circle.y - y;
        Self::sq_i128(dx) + Self::sq_i128(dy) <= Self::sq_i128(circle.r)
    }

    fn circles_overlap_in_rect(
        circle_a: &Circle,
        circle_b: &Circle,
        x_corner: i64,
        y_corner: i64,
    ) -> bool {
        let dx = circle_a.x - circle_b.x;
        let dy = circle_a.y - circle_b.y;
        let dist_sq = Self::sq_i128(dx) + Self::sq_i128(dy);
        let r_sum = circle_a.r + circle_b.r;
        if dist_sq > Self::sq_i128(r_sum) {
            return false;
        }

        let r_diff = (circle_a.r - circle_b.r).abs();
        if dist_sq <= Self::sq_i128(r_diff) {
            let smaller = if circle_a.r <= circle_b.r {
                circle_a
            } else {
                circle_b
            };
            return Self::circle_intersects_rect(smaller, x_corner, y_corner);
        }

        Self::circle_intersections_in_rect(circle_a, circle_b, x_corner, y_corner)
            || Self::edges_overlap_in_rect(circle_a, circle_b, x_corner, y_corner)
    }

    fn circle_intersections_in_rect(
        circle_a: &Circle,
        circle_b: &Circle,
        x_corner: i64,
        y_corner: i64,
    ) -> bool {
        let dx = (circle_b.x - circle_a.x) as f64;
        let dy = (circle_b.y - circle_a.y) as f64;
        let d_sq = dx * dx + dy * dy;
        let d = d_sq.sqrt();
        if d <= Self::EPS {
            return false;
        }

        let r1 = circle_a.r as f64;
        let r2 = circle_b.r as f64;
        let a = (r1 * r1 - r2 * r2 + d_sq) / (2.0 * d);
        let h_sq = r1 * r1 - a * a;
        if h_sq < -Self::EPS {
            return false;
        }
        let h = h_sq.max(0.0).sqrt();

        let xm = circle_a.x as f64 + a * dx / d;
        let ym = circle_a.y as f64 + a * dy / d;
        let rx = -dy * (h / d);
        let ry = dx * (h / d);

        let p1x = xm + rx;
        let p1y = ym + ry;
        if Self::point_in_rect(p1x, p1y, x_corner, y_corner) {
            return true;
        }

        let p2x = xm - rx;
        let p2y = ym - ry;
        Self::point_in_rect(p2x, p2y, x_corner, y_corner)
    }

    fn point_in_rect(x: f64, y: f64, x_corner: i64, y_corner: i64) -> bool {
        let x_corner = x_corner as f64;
        let y_corner = y_corner as f64;
        x >= -Self::EPS && y >= -Self::EPS && x <= x_corner + Self::EPS && y <= y_corner + Self::EPS
    }

    fn edges_overlap_in_rect(
        circle_a: &Circle,
        circle_b: &Circle,
        x_corner: i64,
        y_corner: i64,
    ) -> bool {
        let left_a = Self::interval_on_vertical_edge(circle_a, 0, 0, y_corner);
        let left_b = Self::interval_on_vertical_edge(circle_b, 0, 0, y_corner);
        if Self::intervals_overlap_opt(left_a, left_b) {
            return true;
        }

        let right_a = Self::interval_on_vertical_edge(circle_a, x_corner, 0, y_corner);
        let right_b = Self::interval_on_vertical_edge(circle_b, x_corner, 0, y_corner);
        if Self::intervals_overlap_opt(right_a, right_b) {
            return true;
        }

        let bottom_a = Self::interval_on_horizontal_edge(circle_a, 0, 0, x_corner);
        let bottom_b = Self::interval_on_horizontal_edge(circle_b, 0, 0, x_corner);
        if Self::intervals_overlap_opt(bottom_a, bottom_b) {
            return true;
        }

        let top_a = Self::interval_on_horizontal_edge(circle_a, y_corner, 0, x_corner);
        let top_b = Self::interval_on_horizontal_edge(circle_b, y_corner, 0, x_corner);
        if Self::intervals_overlap_opt(top_a, top_b) {
            return true;
        }

        false
    }

    fn interval_on_vertical_edge(
        circle: &Circle,
        x_edge: i64,
        y_min: i64,
        y_max: i64,
    ) -> Option<(f64, f64)> {
        let r = circle.r as f64;
        let dx = (circle.x - x_edge) as f64;
        let inside = r * r - dx * dx;
        if inside < -Self::EPS {
            return None;
        }

        let delta = inside.max(0.0).sqrt();
        let low = (circle.y as f64 - delta).max(y_min as f64);
        let high = (circle.y as f64 + delta).min(y_max as f64);
        if low <= high + Self::EPS {
            Some((low, high))
        } else {
            None
        }
    }

    fn interval_on_horizontal_edge(
        circle: &Circle,
        y_edge: i64,
        x_min: i64,
        x_max: i64,
    ) -> Option<(f64, f64)> {
        let r = circle.r as f64;
        let dy = (circle.y - y_edge) as f64;
        let inside = r * r - dy * dy;
        if inside < -Self::EPS {
            return None;
        }

        let delta = inside.max(0.0).sqrt();
        let low = (circle.x as f64 - delta).max(x_min as f64);
        let high = (circle.x as f64 + delta).min(x_max as f64);
        if low <= high + Self::EPS {
            Some((low, high))
        } else {
            None
        }
    }

    fn intervals_overlap_opt(a: Option<(f64, f64)>, b: Option<(f64, f64)>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => Self::intervals_overlap(a, b),
            _ => false,
        }
    }

    fn intervals_overlap(a: (f64, f64), b: (f64, f64)) -> bool {
        a.0 <= b.1 + Self::EPS && b.0 <= a.1 + Self::EPS
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
    fn blocked_by_overlapping_chain() {
        assert!(!Solution::can_reach_corner(
            3,
            3,
            vec![vec![2, 1, 1], vec![1, 2, 1]]
        ));
    }

    #[test]
    fn left_bottom_touch_blocks_start() {
        assert!(!Solution::can_reach_corner(4, 4, vec![vec![1, 1, 1]]));
    }

    #[test]
    fn overlap_outside_does_not_block() {
        assert!(Solution::can_reach_corner(
            3,
            3,
            vec![vec![6, 2, 3], vec![2, 6, 3]]
        ));
    }

    #[test]
    fn target_inside_circle() {
        assert!(!Solution::can_reach_corner(4, 4, vec![vec![4, 4, 1]]));
    }
}
