use std::collections::BTreeSet;

struct SegTree {
    size: usize,
    data: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        Self {
            size,
            data: vec![0; size * 2],
        }
    }

    fn update(&mut self, pos: usize, value: i32) {
        let mut idx = self.size + pos - 1;
        self.data[idx] = value;
        while idx > 1 {
            idx /= 2;
            let left = idx * 2;
            self.data[idx] = self.data[left].max(self.data[left + 1]);
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> i32 {
        if left > right {
            return 0;
        }
        left = left + self.size - 1;
        right = right + self.size - 1;
        let mut best = 0;
        while left <= right {
            if left % 2 == 1 {
                best = best.max(self.data[left]);
                left += 1;
            }
            if right % 2 == 0 {
                best = best.max(self.data[right]);
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }
        best
    }
}

impl Solution {
    /// Prefix maximum of obstacle gaps with a segment tree.
    ///
    /// # Intuition
    /// A block fits in `[0, x]` iff some obstacle-free gap inside that prefix has length at least
    /// `sz`. Gaps between consecutive obstacles are fixed by their right endpoint, while the last
    /// partial gap ends at `x`.
    ///
    /// # Approach
    /// Maintain a sorted set of obstacles. For every obstacle at position `b`, store the gap length
    /// `b - prev` where `prev` is the previous obstacle (or 0). Keep these lengths in a segment
    /// tree keyed by the right endpoint `b`. When inserting a new obstacle `p`, update the gap for
    /// `p` and shorten the gap for its successor. For a query `[2, x, sz]`, take the maximum of:
    /// 1) the segment tree maximum over endpoints `<= x`, and 2) the tail gap `x - prev`. If that
    /// maximum is at least `sz`, the block fits.
    ///
    /// # Complexity
    /// - Time: O(q log M), where M is the maximum coordinate
    /// - Space: O(M)
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let max_x = queries
            .iter()
            .filter_map(|query| query.get(1).copied())
            .max()
            .unwrap_or(0) as usize;
        let mut seg_tree = SegTree::new(max_x.max(1));
        let mut obstacles = BTreeSet::new();
        let mut results = Vec::new();

        for query in queries {
            match query.get(0).copied() {
                Some(1) => {
                    if query.len() < 2 {
                        continue;
                    }
                    let pos = query[1] as usize;
                    let prev = obstacles.range(..=pos).next_back().copied().unwrap_or(0);
                    let next = obstacles.range((pos + 1)..).next().copied();

                    seg_tree.update(pos, (pos - prev) as i32);
                    if let Some(next_pos) = next {
                        seg_tree.update(next_pos, (next_pos - pos) as i32);
                    }
                    obstacles.insert(pos);
                }
                Some(2) => {
                    if query.len() < 3 {
                        continue;
                    }
                    let x = query[1] as usize;
                    let size = query[2];
                    let prefix_max = if x == 0 { 0 } else { seg_tree.query(1, x) };
                    let prev = obstacles.range(..=x).next_back().copied().unwrap_or(0);
                    let tail_gap = (x - prev) as i32;
                    let best = prefix_max.max(tail_gap);
                    results.push(best >= size);
                }
                _ => {}
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let queries = vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]];
        assert_eq!(Solution::get_results(queries), vec![false, true, true]);
    }

    #[test]
    fn test_example_2() {
        let queries = vec![
            vec![1, 7],
            vec![2, 7, 6],
            vec![1, 2],
            vec![2, 7, 5],
            vec![2, 7, 6],
        ];
        assert_eq!(Solution::get_results(queries), vec![true, true, false]);
    }

    #[test]
    fn test_no_obstacles_prefix() {
        let queries = vec![vec![2, 5, 3], vec![2, 5, 6]];
        assert_eq!(Solution::get_results(queries), vec![true, false]);
    }

    #[test]
    fn test_tail_gap_only() {
        let queries = vec![
            vec![1, 10],
            vec![2, 9, 10],
            vec![2, 10, 10],
            vec![2, 10, 11],
        ];
        assert_eq!(Solution::get_results(queries), vec![false, true, false]);
    }
}
