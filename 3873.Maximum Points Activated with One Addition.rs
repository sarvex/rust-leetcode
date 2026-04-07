impl Solution {
    /// Sort-and-sweep Union-Find on point indices.
    ///
    /// # Intuition
    /// Points sharing an x- or y-coordinate activate each other transitively,
    /// forming connected components. Adding one point at (x, y) merges the
    /// component of row x with that of column y. The optimum bridges the two
    /// largest components, which is always feasible for distinct components.
    ///
    /// # Approach
    /// 1. Build `(coordinate, point_index)` pairs, sort by coordinate, and
    ///    union adjacent pairs that share the same value — first for x, then
    ///    reuse the same buffer for y. This avoids coordinate compression and
    ///    per-point binary-search lookups entirely.
    /// 2. Scan roots for the two largest component sizes.
    /// 3. The answer is their sum plus one (the added point).
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting; O(n α(n)) for unions
    /// - Space: O(n)
    pub fn max_activated(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 0 {
            return 1;
        }

        let mut parent: Vec<usize> = (0..n).collect();
        let mut size = vec![1i32; n];

        let mut sorted: Vec<(i32, usize)> =
            points.iter().enumerate().map(|(i, p)| (p[0], i)).collect();
        sorted.sort_unstable();
        for w in sorted.windows(2) {
            if w[0].0 == w[1].0 {
                Self::union(w[0].1, w[1].1, &mut parent, &mut size);
            }
        }

        for (i, p) in points.iter().enumerate() {
            sorted[i] = (p[1], i);
        }
        sorted.sort_unstable();
        for w in sorted.windows(2) {
            if w[0].0 == w[1].0 {
                Self::union(w[0].1, w[1].1, &mut parent, &mut size);
            }
        }

        let (mut top1, mut top2) = (0i32, 0i32);
        for i in 0..n {
            if parent[i] == i {
                if size[i] >= top1 {
                    top2 = top1;
                    top1 = size[i];
                } else if size[i] > top2 {
                    top2 = size[i];
                }
            }
        }

        if top2 > 0 {
            top1 + top2 + 1
        } else {
            top1 + 1
        }
    }

    fn find(mut x: usize, parent: &mut [usize]) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }

    fn union(i: usize, j: usize, parent: &mut [usize], size: &mut [i32]) {
        let ri = Self::find(i, parent);
        let rj = Self::find(j, parent);
        if ri != rj {
            if size[ri] < size[rj] {
                parent[ri] = rj;
                size[rj] += size[ri];
            } else {
                parent[rj] = ri;
                size[ri] += size[rj];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_activation() {
        assert_eq!(
            Solution::max_activated(vec![vec![1, 1], vec![1, 2], vec![2, 2]]),
            4
        );
    }

    #[test]
    fn two_components_merged() {
        assert_eq!(
            Solution::max_activated(vec![vec![2, 2], vec![1, 1], vec![3, 3]]),
            3
        );
    }

    #[test]
    fn partial_chain() {
        assert_eq!(
            Solution::max_activated(vec![vec![2, 3], vec![2, 2], vec![1, 1], vec![4, 5]]),
            4
        );
    }

    #[test]
    fn single_point() {
        assert_eq!(Solution::max_activated(vec![vec![0, 0]]), 2);
    }

    #[test]
    fn all_same_x() {
        assert_eq!(
            Solution::max_activated(vec![vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4]]),
            5
        );
    }

    #[test]
    fn two_large_components() {
        assert_eq!(
            Solution::max_activated(vec![
                vec![1, 1],
                vec![1, 2],
                vec![2, 2],
                vec![10, 10],
                vec![10, 20],
                vec![20, 20],
            ]),
            7
        );
    }

    #[test]
    fn negative_coordinates() {
        assert_eq!(
            Solution::max_activated(vec![vec![-1, -1], vec![-1, 1], vec![1, -1]]),
            4
        );
    }
}
