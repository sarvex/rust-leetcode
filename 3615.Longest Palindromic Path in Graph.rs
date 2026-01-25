use std::collections::HashMap;

impl Solution {
    /// Optimized two-pointer expansion with bitmask operations and pruning.
    ///
    /// # Intuition
    /// Build palindromes from center outward. Use bitmasks for fast neighbor
    /// lookups and group nodes by label to reduce iterations. Prune branches
    /// that can't beat current best.
    ///
    /// # Approach
    /// 1. Store adjacency as bitmasks for O(1) neighbor checking
    /// 2. Group nodes by label for efficient matching
    /// 3. Normalize states (left ≤ right) for symmetry
    /// 4. Prune when remaining nodes can't improve result
    ///
    /// # Complexity
    /// - Time: O(n² × 2^n) with aggressive pruning
    /// - Space: O(n² × 2^n) for memoization
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }

        let label: Vec<u8> = label.bytes().collect();

        let mut adj = vec![0u16; n];
        edges.iter().for_each(|edge| {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[u] |= 1 << v;
            adj[v] |= 1 << u;
        });

        let mut by_label = [0u16; 26];
        (0..n).for_each(|i| {
            by_label[(label[i] - b'a') as usize] |= 1 << i;
        });

        let mut result = 1i32;
        let mut memo: HashMap<u32, i8> = HashMap::with_capacity(1 << 18);

        // Odd length palindromes
        for center in 0..n {
            let val = Self::dfs(&adj, &by_label, center, center, 1u16 << center, n, &mut memo, &mut result);
            result = result.max(val);
        }

        // Even length palindromes
        for u in 0..n {
            for v in (u + 1)..n {
                if adj[u] & (1 << v) != 0 && label[u] == label[v] {
                    let mask = (1u16 << u) | (1u16 << v);
                    let val = Self::dfs(&adj, &by_label, u, v, mask, n, &mut memo, &mut result);
                    result = result.max(val);
                }
            }
        }

        result
    }

    fn dfs(
        adj: &[u16],
        by_label: &[u16; 26],
        left: usize,
        right: usize,
        mask: u16,
        n: usize,
        memo: &mut HashMap<u32, i8>,
        best: &mut i32,
    ) -> i32 {
        let current = mask.count_ones() as i32;
        let remaining = n as i32 - current;

        // Prune: can't beat current best even using all remaining nodes
        if current + remaining - (remaining & 1) <= *best {
            return current;
        }

        let (l, r) = if left <= right { (left, right) } else { (right, left) };
        let key = ((l as u32) << 18) | ((r as u32) << 14) | (mask as u32);

        if let Some(&v) = memo.get(&key) {
            return v as i32;
        }

        let mut max_len = current as i8;
        let left_avail = adj[left] & !mask;
        let right_avail = adj[right] & !mask;

        for c in 0..26 {
            let candidates = by_label[c];
            let left_cands = left_avail & candidates;
            let right_cands = right_avail & candidates;

            if left_cands != 0 && right_cands != 0 {
                let mut lc = left_cands;
                while lc != 0 {
                    let nl = lc.trailing_zeros() as usize;
                    lc &= lc - 1;

                    let mut rc = right_cands;
                    while rc != 0 {
                        let nr = rc.trailing_zeros() as usize;
                        rc &= rc - 1;

                        if nl != nr {
                            let new_mask = mask | (1 << nl) | (1 << nr);
                            let extended = Self::dfs(adj, by_label, nl, nr, new_mask, n, memo, best) as i8;
                            if extended > max_len {
                                max_len = extended;
                                *best = (*best).max(extended as i32);
                            }
                        }
                    }
                }
            }
        }

        memo.insert(key, max_len);
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::max_len(3, edges, "aba".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        assert_eq!(Solution::max_len(3, edges, "abc".to_string()), 1);
    }

    #[test]
    fn test_example_3() {
        let edges = vec![vec![0, 2], vec![0, 3], vec![3, 1]];
        assert_eq!(Solution::max_len(4, edges, "bbac".to_string()), 3);
    }

    #[test]
    fn test_single_node() {
        let edges: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::max_len(1, edges, "a".to_string()), 1);
    }

    #[test]
    fn test_two_nodes_same_label() {
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::max_len(2, edges, "aa".to_string()), 2);
    }

    #[test]
    fn test_two_nodes_different_label() {
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::max_len(2, edges, "ab".to_string()), 1);
    }

    #[test]
    fn test_longer_palindrome() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::max_len(5, edges, "abcba".to_string()), 5);
    }

    #[test]
    fn test_branching() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        assert_eq!(Solution::max_len(4, edges, "abbc".to_string()), 3);
    }

    #[test]
    fn test_complete_graph_same_labels() {
        // K14 with all 'a' except one 'z'
        let mut edges = vec![];
        for i in 0..14 {
            for j in (i + 1)..14 {
                edges.push(vec![i, j]);
            }
        }
        assert_eq!(Solution::max_len(14, edges, "aaaaaaaaaaaaaz".to_string()), 13);
    }
}
