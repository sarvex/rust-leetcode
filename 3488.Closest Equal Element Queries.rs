use std::collections::HashMap;

impl Solution {
    /// Precomputed nearest-same-value circular distances with O(1) query lookup.
    ///
    /// # Intuition
    /// In a circular array, indices sharing the same value form a sorted group.
    /// Each index's nearest same-value neighbor is one of its two adjacent
    /// members in that group (wrapping circularly). Precomputing these distances
    /// for every index turns each query into a simple table lookup.
    ///
    /// # Approach
    /// 1. Group all indices by their value using a `HashMap`.
    /// 2. For each group with ≥ 2 members, iterate through consecutive pairs
    ///    (including the wrap-around pair last↔first) and record the minimum
    ///    circular distance at each index.
    /// 3. Answer each query by reading the precomputed table.
    ///
    /// # Complexity
    /// - Time: O(n + q) — grouping is O(n), precomputation visits each index
    ///   once across all groups, each query is O(1).
    /// - Space: O(n) — for the distance table and grouped-index map.
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut min_dist = vec![-1_i32; n];
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::with_capacity(n);

        for (i, &val) in nums.iter().enumerate() {
            groups.entry(val).or_default().push(i);
        }

        for group in groups.values() {
            let len = group.len();
            if len < 2 {
                continue;
            }

            for w in group.windows(2) {
                let d = (w[1] - w[0]).min(n - w[1] + w[0]) as i32;
                min_dist[w[0]] = if min_dist[w[0]] < 0 {
                    d
                } else {
                    min_dist[w[0]].min(d)
                };
                min_dist[w[1]] = if min_dist[w[1]] < 0 {
                    d
                } else {
                    min_dist[w[1]].min(d)
                };
            }

            let first = group[0];
            let last = group[len - 1];
            let d = (last - first).min(n - last + first) as i32;
            min_dist[first] = if min_dist[first] < 0 {
                d
            } else {
                min_dist[first].min(d)
            };
            min_dist[last] = if min_dist[last] < 0 {
                d
            } else {
                min_dist[last].min(d)
            };
        }

        queries.iter().map(|&q| min_dist[q as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_duplicates() {
        assert_eq!(
            Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
            vec![2, -1, 3]
        );
    }

    #[test]
    fn all_unique() {
        assert_eq!(
            Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
            vec![-1, -1, -1, -1]
        );
    }

    #[test]
    fn all_same() {
        assert_eq!(
            Solution::solve_queries(vec![5, 5, 5, 5], vec![0, 1, 2, 3]),
            vec![1, 1, 1, 1]
        );
    }

    #[test]
    fn two_elements_same() {
        assert_eq!(Solution::solve_queries(vec![7, 7], vec![0, 1]), vec![1, 1]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::solve_queries(vec![42], vec![0]), vec![-1]);
    }

    #[test]
    fn circular_wrap_shorter() {
        assert_eq!(
            Solution::solve_queries(vec![1, 2, 3, 1], vec![0, 3]),
            vec![1, 1]
        );
    }
}
