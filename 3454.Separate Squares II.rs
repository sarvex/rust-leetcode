struct SegmentTree {
    count: Vec<i32>,
    occupied: Vec<i64>,
    total_width: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    fn new(xs: &[i32]) -> Self {
        let n = xs.len() - 1;
        let size = 4 * n;
        let mut st = Self {
            count: vec![0; size],
            occupied: vec![0; size],
            total_width: vec![0; size],
            n,
        };
        if n > 0 {
            st.build(0, 0, n - 1, xs);
        }
        st
    }

    fn build(&mut self, x: usize, lx: usize, rx: usize, xs: &[i32]) {
        if lx == rx {
            self.total_width[x] = (xs[lx + 1] - xs[lx]) as i64;
            return;
        }
        let mid = (lx + rx) / 2;
        self.build(2 * x + 1, lx, mid, xs);
        self.build(2 * x + 2, mid + 1, rx, xs);
        self.total_width[x] = self.total_width[2 * x + 1] + self.total_width[2 * x + 2];
    }

    fn update(&mut self, x: usize, lx: usize, rx: usize, l: usize, r: usize, v: i32) {
        if lx >= l && rx <= r {
            self.count[x] += v;
        } else {
            let mid = (lx + rx) / 2;
            if l <= mid {
                self.update(2 * x + 1, lx, mid, l, r, v);
            }
            if r > mid {
                self.update(2 * x + 2, mid + 1, rx, l, r, v);
            }
        }

        if self.count[x] > 0 {
            self.occupied[x] = self.total_width[x];
        } else if lx != rx {
            self.occupied[x] = self.occupied[2 * x + 1] + self.occupied[2 * x + 2];
        } else {
            self.occupied[x] = 0;
        }
    }
}

#[derive(Clone, Copy)]
struct Event {
    y: i32,
    type_: i32,
    l_idx: u32,
    r_idx: u32,
}

impl Solution {
    /// # Segment Tree with Sweep Line Algorithm
    ///
    /// # Intuition
    /// Use a segment tree to efficiently track occupied x-intervals as we sweep through y-coordinates.
    /// This allows O(log n) updates and queries for interval coverage, making the overall algorithm much faster.
    ///
    /// # Approach
    /// 1. Compress x-coordinates and build a segment tree over them
    /// 2. Create events for square start/end at each y-coordinate
    /// 3. Process events in sorted order, updating the segment tree
    /// 4. Track cumulative area and find where it equals half the total
    /// 5. Use lazy propagation in segment tree for efficient range updates
    ///
    /// # Complexity
    /// - Time: O(n log n) where n is the number of squares
    /// - Space: O(n) for segment tree and events
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        if squares.is_empty() {
            return 0.0;
        }

        let n_sq = squares.len();
        let mut xs = Vec::with_capacity(n_sq * 2);
        for s in &squares {
            xs.push(s[0]);
            xs.push(s[0] + s[2]);
        }
        xs.sort_unstable();
        xs.dedup();

        if xs.len() < 2 {
            return squares[0][1] as f64;
        }

        let mut events = Vec::with_capacity(n_sq * 2);
        for s in &squares {
            let x1 = s[0];
            let x2 = s[0] + s[2];
            let y1 = s[1];
            let y2 = s[1] + s[2];

            let l = xs.binary_search(&x1).unwrap();
            let r = xs.binary_search(&x2).unwrap();

            if l < r {
                events.push(Event {
                    y: y1,
                    type_: 1,
                    l_idx: l as u32,
                    r_idx: r as u32,
                });
                events.push(Event {
                    y: y2,
                    type_: -1,
                    l_idx: l as u32,
                    r_idx: r as u32,
                });
            }
        }

        events.sort_unstable_by(|a, b| a.y.cmp(&b.y));

        let mut st = SegmentTree::new(&xs);
        let mut total_area: f64 = 0.0;
        let mut history = Vec::with_capacity(events.len());

        let mut i = 0;
        let n_events = events.len();

        while i < n_events {
            let y = events[i].y;

            while i < n_events && events[i].y == y {
                let e = events[i];
                st.update(
                    0,
                    0,
                    st.n - 1,
                    e.l_idx as usize,
                    (e.r_idx - 1) as usize,
                    e.type_,
                );
                i += 1;
            }

            if i < n_events {
                let next_y = events[i].y;
                if next_y > y {
                    let width = st.occupied[0];
                    if width > 0 {
                        let h = next_y - y;
                        history.push((total_area, y, next_y, width));
                        total_area += width as f64 * h as f64;
                    }
                }
            }
        }

        let target = total_area / 2.0;

        for (prev_area, y1, _y2, width) in history {
            let strip_max_area = width as f64 * (_y2 - y1) as f64;
            if prev_area + strip_max_area >= target {
                let needed = target - prev_area;
                return y1 as f64 + needed / width as f64;
            }
        }

        events.last().map(|e| e.y as f64).unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let squares = vec![vec![0, 0, 1], vec![2, 2, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_example_2() {
        let squares = vec![vec![0, 0, 2], vec![1, 1, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_single_square() {
        let squares = vec![vec![0, 0, 2]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_non_overlapping() {
        let squares = vec![vec![0, 0, 1], vec![0, 2, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5 || (result - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_large_input() {
        let mut squares = Vec::new();
        for i in 0..5000 {
            squares.push(vec![i * 2, i * 2, 1]);
        }
        let result = Solution::separate_squares(squares);
        assert!(result > 0.0);
    }

    #[test]
    fn test_overlapping_complex() {
        let squares = vec![vec![0, 0, 3], vec![1, 1, 3], vec![2, 2, 3]];
        let result = Solution::separate_squares(squares);
        assert!(result > 0.0);
    }
}
