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
    /// Finds the y-coordinate that splits total square area in half via sweep line.
    ///
    /// # Intuition
    /// A segment tree tracks occupied x-intervals as we sweep upward through
    /// y-events. Cumulative area grows in strips; binary search within the
    /// crossing strip locates the exact split line.
    ///
    /// # Approach
    /// 1. Compress x-coordinates and build a segment tree over them.
    /// 2. Create open/close events for each square at its bottom/top y.
    /// 3. Process events in y-order, accumulating area per horizontal strip.
    /// 4. Walk the strip history to find where cumulative area reaches half.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        if squares.is_empty() {
            return 0.0;
        }

        let mut xs: Vec<i32> = squares.iter().flat_map(|s| [s[0], s[0] + s[2]]).collect();
        xs.sort_unstable();
        xs.dedup();

        if xs.len() < 2 {
            return squares[0][1] as f64;
        }

        let mut events = Vec::with_capacity(squares.len() * 2);
        for s in &squares {
            let (x1, x2) = (s[0], s[0] + s[2]);
            let (y1, y2) = (s[1], s[1] + s[2]);

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

        events.sort_unstable_by_key(|e| e.y);

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

        history
            .iter()
            .find_map(|&(prev_area, y1, y2, width)| {
                let strip_max_area = width as f64 * (y2 - y1) as f64;
                (prev_area + strip_max_area >= target).then(|| {
                    let needed = target - prev_area;
                    y1 as f64 + needed / width as f64
                })
            })
            .unwrap_or_else(|| events.last().map(|e| e.y as f64).unwrap_or(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_separated_squares_split_between() {
        let squares = vec![vec![0, 0, 1], vec![2, 2, 1]];
        assert!((Solution::separate_squares(squares) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn overlapping_squares() {
        let squares = vec![vec![0, 0, 2], vec![1, 1, 1]];
        assert!((Solution::separate_squares(squares) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn single_square_splits_at_midpoint() {
        let squares = vec![vec![0, 0, 2]];
        assert!((Solution::separate_squares(squares) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn vertically_stacked_non_overlapping() {
        let squares = vec![vec![0, 0, 1], vec![0, 2, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5 || (result - 2.0).abs() < 1e-5);
    }

    #[test]
    fn large_input_runs_efficiently() {
        let squares: Vec<Vec<i32>> = (0..5000).map(|i| vec![i * 2, i * 2, 1]).collect();
        let result = Solution::separate_squares(squares);
        assert!(result > 0.0);
    }

    #[test]
    fn triple_overlapping_squares() {
        let squares = vec![vec![0, 0, 3], vec![1, 1, 3], vec![2, 2, 3]];
        assert!(Solution::separate_squares(squares) > 0.0);
    }
}
