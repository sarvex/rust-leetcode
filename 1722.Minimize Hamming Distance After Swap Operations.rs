use std::collections::HashMap;

impl Solution {
    /// Union-Find over indices with per-component multiset matching.
    ///
    /// # Intuition
    /// Allowed swaps form an equivalence relation on indices: any two indices
    /// in the same connected component can be reordered arbitrarily (any
    /// permutation is reachable via repeated transpositions on a connected
    /// graph). So within one component, only the *multiset* of source values
    /// matters - not the order. The best we can do is match as many source
    /// values as possible to target values in that same component. Remaining
    /// unmatched target positions contribute to the Hamming distance.
    ///
    /// # Approach
    /// 1. Build Union-Find over `n` indices and union each allowed swap pair.
    /// 2. For every index `i`, find its root `r` and record `source[i]` in a
    ///    frequency map keyed by `(root, value)`.
    /// 3. Walk target: for each `i`, look up `(root(i), target[i])`. If the
    ///    frequency is positive, decrement it (matched - contributes 0);
    ///    otherwise this position contributes 1 to the Hamming distance.
    ///
    /// Using a single `HashMap<i64, i32>` with a packed `(root << 20) | value`
    /// key avoids tuple-hash overhead. `value <= 10^5 < 2^17`, so 20 bits is
    /// safe; `root < n <= 10^5 < 2^17`, so the composite fits in `i64`.
    ///
    /// # Complexity
    /// - Time: O((n + m) * α(n)) where `m = allowed_swaps.len()`
    /// - Space: O(n)
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<u32> = vec![0; n];

        for swap in &allowed_swaps {
            Self::union(swap[0] as usize, swap[1] as usize, &mut parent, &mut rank);
        }

        let mut freq: HashMap<i64, i32> = HashMap::with_capacity(n);
        for (i, &v) in source.iter().enumerate() {
            let root = Self::find(i, &mut parent);
            let key = ((root as i64) << 20) | (v as i64);
            *freq.entry(key).or_insert(0) += 1;
        }

        let mut distance = 0;
        for (i, &v) in target.iter().enumerate() {
            let root = Self::find(i, &mut parent);
            let key = ((root as i64) << 20) | (v as i64);
            match freq.get_mut(&key) {
                Some(count) if *count > 0 => *count -= 1,
                _ => distance += 1,
            }
        }

        distance
    }

    fn find(mut x: usize, parent: &mut [usize]) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }

    fn union(x: usize, y: usize, parent: &mut [usize], rank: &mut [u32]) {
        let (mut px, mut py) = (Self::find(x, parent), Self::find(y, parent));
        if px == py {
            return;
        }
        if rank[px] < rank[py] {
            std::mem::swap(&mut px, &mut py);
        }
        parent[py] = px;
        if rank[px] == rank[py] {
            rank[px] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_two_components() {
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![1, 2, 3, 4],
                vec![2, 1, 4, 5],
                vec![vec![0, 1], vec![2, 3]],
            ),
            1
        );
    }

    #[test]
    fn example_two_no_swaps() {
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![1, 2, 3, 4],
                vec![1, 3, 2, 4],
                vec![],
            ),
            2
        );
    }

    #[test]
    fn example_three_transitive_union() {
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![5, 1, 2, 4, 3],
                vec![1, 5, 4, 2, 3],
                vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]],
            ),
            0
        );
    }

    #[test]
    fn single_element_match() {
        assert_eq!(
            Solution::minimum_hamming_distance(vec![7], vec![7], vec![]),
            0
        );
    }

    #[test]
    fn single_element_mismatch_no_swaps() {
        assert_eq!(
            Solution::minimum_hamming_distance(vec![7], vec![9], vec![]),
            1
        );
    }

    #[test]
    fn duplicates_in_component_consume_only_once() {
        // Component {0,1,2}: source multiset {1,1,2}, target multiset {1,2,2}.
        // Intersection as multisets has size 2 -> 1 mismatch.
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![1, 1, 2],
                vec![1, 2, 2],
                vec![vec![0, 1], vec![1, 2]],
            ),
            1
        );
    }

    #[test]
    fn redundant_swap_pairs_do_not_break_union() {
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![1, 2, 3, 4],
                vec![4, 3, 2, 1],
                vec![
                    vec![0, 1],
                    vec![1, 0],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                ],
            ),
            0
        );
    }

    #[test]
    fn isolated_indices_are_exact_match_only() {
        // Index 2 is isolated: source[2]=9 != target[2]=8 -> contributes 1.
        // Indices {0,1} form a component with matching multisets -> 0.
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![3, 4, 9],
                vec![4, 3, 8],
                vec![vec![0, 1]],
            ),
            1
        );
    }

    #[test]
    fn all_same_values_zero_distance() {
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![5, 5, 5, 5],
                vec![5, 5, 5, 5],
                vec![],
            ),
            0
        );
    }

    #[test]
    fn large_value_boundary() {
        // value = 100000 sits near its stated upper bound; ensure packed key is fine.
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![100_000, 1],
                vec![1, 100_000],
                vec![vec![0, 1]],
            ),
            0
        );
    }
}
