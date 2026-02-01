use std::collections::BTreeMap;

impl Solution {
    /// Tracks maximal shortcut intervals to keep the shortest path length.
    ///
    /// # Intuition
    /// A direct road `u -> v` always beats any path that stays inside `(u, v)`.
    /// With non-crossing roads, every path from `0` to `n - 1` must pass through
    /// the start of each outermost road, so taking all such roads is optimal.
    ///
    /// # Approach
    /// - Maintain only maximal (not contained) roads as disjoint intervals.
    /// - Each maximal road saves `v - u - 1` steps compared to the chain.
    /// - On insertion:
    ///   - If the new road is contained in an existing maximal interval, ignore it.
    ///   - Otherwise remove all maximal intervals starting in `[u, v)` and insert `(u, v)`.
    /// - The shortest distance is `(n - 1) - total_savings`.
    ///
    /// # Complexity
    /// - Time: O(q log q) amortized, each interval removed once
    /// - Space: O(q)
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n_usize = n as usize;
        let mut max_intervals: BTreeMap<usize, usize> = BTreeMap::new();
        let mut total_savings: i64 = 0;
        let mut answers = Vec::with_capacity(queries.len());

        for query in queries {
            let u = query[0] as usize;
            let v = query[1] as usize;

            let mut contained = false;
            if let Some((&start, &end)) = max_intervals.range(..=u).next_back() {
                if end >= v {
                    contained = true;
                }
            }

            if !contained {
                loop {
                    let next = max_intervals
                        .range(u..v)
                        .next()
                        .map(|(&start, &end)| (start, end));
                    if let Some((start, end)) = next {
                        total_savings -= (end - start - 1) as i64;
                        max_intervals.remove(&start);
                    } else {
                        break;
                    }
                }
                max_intervals.insert(u, v);
                total_savings += (v - u - 1) as i64;
            }

            let shortest = (n_usize - 1) as i64 - total_savings;
            answers.push(shortest as i32);
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let n = 5;
        let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_example_two() {
        let n = 4;
        let queries = vec![vec![0, 3], vec![0, 2]];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![1, 1]);
    }

    #[test]
    fn test_nested_edges() {
        let n = 6;
        let queries = vec![vec![1, 4], vec![0, 5], vec![2, 3]];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![3, 1, 1]);
    }

    #[test]
    fn test_disjoint_edges() {
        let n = 8;
        let queries = vec![vec![0, 3], vec![5, 7], vec![3, 5]];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![5, 4, 3]);
    }

    #[test]
    fn test_contained_edge_no_change() {
        let n = 5;
        let queries = vec![vec![0, 4], vec![1, 3]];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![1, 1]);
    }
}
