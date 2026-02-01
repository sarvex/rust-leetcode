impl Solution {
    /// Compute the longest increasing chain that must include `coordinates[k]`.
    ///
    /// # Intuition
    /// The best chain that includes a fixed point is the best chain ending at it plus the best
    /// chain starting from it, minus one for the shared point.
    ///
    /// # Approach
    /// - Compress the `y` values for Fenwick indexing.
    /// - Sort by `x` ascending and compute the longest chain ending at each point using a prefix
    ///   maximum over `y`, updating the Fenwick tree only after finishing each equal-`x` group to
    ///   preserve strict `x` growth.
    /// - Sort by `x` descending and repeat with reversed `y` indices to compute the longest chain
    ///   starting at each point.
    /// - Return `left[k] + right[k] - 1`.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn max_path_length(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        #[derive(Clone, Copy)]
        struct Point {
            id: usize,
            x: i32,
            y: i32,
            y_idx: usize,
            rev_idx: usize,
        }

        struct Fenwick {
            tree: Vec<i32>,
        }

        impl Fenwick {
            fn new(size: usize) -> Self {
                Self {
                    tree: vec![0; size + 1],
                }
            }

            fn update(&mut self, mut idx: usize, val: i32) {
                let n = self.tree.len();
                while idx < n {
                    if val > self.tree[idx] {
                        self.tree[idx] = val;
                    }
                    idx += idx & idx.wrapping_neg();
                }
            }

            fn query(&self, mut idx: usize) -> i32 {
                let mut best = 0;
                while idx > 0 {
                    let candidate = self.tree[idx];
                    if candidate > best {
                        best = candidate;
                    }
                    idx -= idx & idx.wrapping_neg();
                }
                best
            }
        }

        let n = coordinates.len();
        let mut points = Vec::with_capacity(n);
        let mut ys = Vec::with_capacity(n);
        for (id, coord) in coordinates.iter().enumerate() {
            let x = coord[0];
            let y = coord[1];
            points.push(Point {
                id,
                x,
                y,
                y_idx: 0,
                rev_idx: 0,
            });
            ys.push(y);
        }

        ys.sort_unstable();
        ys.dedup();
        for point in &mut points {
            let y_idx = ys.binary_search(&point.y).unwrap() + 1;
            point.y_idx = y_idx;
        }

        let m = ys.len();
        for point in &mut points {
            point.rev_idx = m - point.y_idx + 1;
        }

        let mut left = vec![1; n];
        points.sort_unstable_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
        let mut bit = Fenwick::new(m);
        let mut i = 0;
        while i < n {
            let x = points[i].x;
            let mut j = i;
            while j < n && points[j].x == x {
                j += 1;
            }
            for point in &points[i..j] {
                let best = bit.query(point.y_idx - 1);
                left[point.id] = best + 1;
            }
            for point in &points[i..j] {
                bit.update(point.y_idx, left[point.id]);
            }
            i = j;
        }

        let mut right = vec![1; n];
        points.sort_unstable_by(|a, b| b.x.cmp(&a.x).then_with(|| b.y.cmp(&a.y)));
        let mut bit = Fenwick::new(m);
        let mut i = 0;
        while i < n {
            let x = points[i].x;
            let mut j = i;
            while j < n && points[j].x == x {
                j += 1;
            }
            for point in &points[i..j] {
                let best = bit.query(point.rev_idx - 1);
                right[point.id] = best + 1;
            }
            for point in &points[i..j] {
                bit.update(point.rev_idx, right[point.id]);
            }
            i = j;
        }

        let target = k as usize;
        left[target] + right[target] - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let coordinates = vec![vec![3, 1], vec![2, 2], vec![4, 1], vec![0, 0], vec![5, 3]];
        assert_eq!(Solution::max_path_length(coordinates, 1), 3);
    }

    #[test]
    fn example_two() {
        let coordinates = vec![vec![2, 1], vec![7, 0], vec![5, 6]];
        assert_eq!(Solution::max_path_length(coordinates, 2), 2);
    }

    #[test]
    fn single_point() {
        let coordinates = vec![vec![7, 7]];
        assert_eq!(Solution::max_path_length(coordinates, 0), 1);
    }

    #[test]
    fn same_x_group_does_not_chain() {
        let coordinates = vec![vec![0, 0], vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::max_path_length(coordinates, 1), 2);
    }

    #[test]
    fn isolated_target() {
        let coordinates = vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![0, 2], vec![2, 0]];
        assert_eq!(Solution::max_path_length(coordinates, 3), 1);
    }
}
