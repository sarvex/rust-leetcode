use std::collections::{hash_map::Entry, HashMap};

struct LengthGraph {
    len: usize,
    index: HashMap<Vec<u8>, usize>,
    dist: Vec<Vec<i64>>,
}

impl Solution {
    /// Computes the minimum cost to convert source to target with
    /// disjoint (or identical) substring operations.
    ///
    /// # Intuition
    /// Operations can only overlap if they are identical, so each index
    /// belongs to at most one substring interval. For a fixed length, repeated
    /// operations on the same interval form a shortest-path problem between
    /// strings of that length.
    ///
    /// # Approach
    /// 1. Group conversion rules by string length.
    /// 2. For each length, build a graph of unique strings and run
    ///    Floyd-Warshall to get all-pairs minimum costs.
    /// 3. Dynamic programming over positions: dp[i] is the minimum cost to
    ///    convert source[i..] to target[i..].
    ///
    /// # Complexity
    /// - Let m be number of rules (<= 100), n be string length (<= 1000).
    /// - Graph work: O(sum(len_graph_nodes^3)) <= O(m^3)
    /// - DP: O(n * number_of_lengths) <= O(n * m)
    /// - Space: O(sum(len_graph_nodes^2) + n)
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        const INF: i64 = i64::MAX / 4;

        let n = source.len();
        let src = source.as_bytes();
        let tgt = target.as_bytes();

        let mut rules_by_len: HashMap<usize, Vec<(Vec<u8>, Vec<u8>, i64)>> =
            HashMap::with_capacity(original.len());
        for ((orig, chg), co) in original.into_iter().zip(changed).zip(cost) {
            let len = orig.len();
            rules_by_len
                .entry(len)
                .or_default()
                .push((orig.into_bytes(), chg.into_bytes(), co as i64));
        }

        let mut graphs: Vec<LengthGraph> = Vec::with_capacity(rules_by_len.len());
        for (len, rules) in rules_by_len.into_iter() {
            let mut index: HashMap<Vec<u8>, usize> = HashMap::with_capacity(rules.len() * 2);
            let mut edges: Vec<(usize, usize, i64)> = Vec::with_capacity(rules.len());

            for (o, c, co) in rules {
                let from_id = index.len();
                let from = match index.entry(o) {
                    Entry::Occupied(entry) => *entry.get(),
                    Entry::Vacant(entry) => {
                        entry.insert(from_id);
                        from_id
                    }
                };

                let to_id = index.len();
                let to = match index.entry(c) {
                    Entry::Occupied(entry) => *entry.get(),
                    Entry::Vacant(entry) => {
                        entry.insert(to_id);
                        to_id
                    }
                };

                edges.push((from, to, co));
            }

            let m = index.len();
            let mut dist = vec![vec![INF; m]; m];
            for i in 0..m {
                dist[i][i] = 0;
            }

            for (from, to, co) in edges {
                if co < dist[from][to] {
                    dist[from][to] = co;
                }
            }

            for k in 0..m {
                for i in 0..m {
                    if dist[i][k] >= INF {
                        continue;
                    }
                    let dik = dist[i][k];
                    for j in 0..m {
                        if dist[k][j] >= INF {
                            continue;
                        }
                        let nd = dik + dist[k][j];
                        if nd < dist[i][j] {
                            dist[i][j] = nd;
                        }
                    }
                }
            }

            graphs.push(LengthGraph { len, index, dist });
        }

        graphs.sort_unstable_by_key(|graph| graph.len);

        let mut dp = vec![INF; n + 1];
        dp[n] = 0;

        for i in (0..n).rev() {
            if src[i] == tgt[i] {
                dp[i] = dp[i + 1];
            }

            for graph in graphs.iter() {
                let len = graph.len;
                if i + len > n {
                    break;
                }

                let src_slice = &src[i..i + len];
                let tgt_slice = &tgt[i..i + len];

                if let (Some(from), Some(to)) =
                    (graph.index.get(src_slice), graph.index.get(tgt_slice))
                {
                    let cost_seg = graph.dist[*from][*to];
                    if cost_seg < INF {
                        let candidate = cost_seg + dp[i + len];
                        if candidate < dp[i] {
                            dp[i] = candidate;
                        }
                    }
                }
            }
        }

        if dp[0] >= INF {
            -1
        } else {
            dp[0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "c".to_string(),
                    "e".to_string(),
                    "d".to_string()
                ],
                vec![
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "e".to_string(),
                    "b".to_string(),
                    "e".to_string()
                ],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::minimum_cost(
                "abcdefgh".to_string(),
                "acdeeghh".to_string(),
                vec!["bcd".to_string(), "fgh".to_string(), "thh".to_string()],
                vec!["cde".to_string(), "thh".to_string(), "ghh".to_string()],
                vec![1, 3, 5]
            ),
            9
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::minimum_cost(
                "abcdefgh".to_string(),
                "addddddd".to_string(),
                vec!["bcd".to_string(), "defgh".to_string()],
                vec!["ddd".to_string(), "ddddd".to_string()],
                vec![100, 1578]
            ),
            -1
        );
    }

    #[test]
    fn matching_prefix_needs_longer_rule() {
        assert_eq!(
            Solution::minimum_cost(
                "ab".to_string(),
                "ac".to_string(),
                vec!["ab".to_string()],
                vec!["ac".to_string()],
                vec![7]
            ),
            7
        );
    }

    #[test]
    fn multi_step_same_segment() {
        assert_eq!(
            Solution::minimum_cost(
                "ab".to_string(),
                "bc".to_string(),
                vec!["ab".to_string(), "ac".to_string()],
                vec!["ac".to_string(), "bc".to_string()],
                vec![5, 3]
            ),
            8
        );
    }

    #[test]
    fn no_rules_but_equal() {
        assert_eq!(
            Solution::minimum_cost(
                "abc".to_string(),
                "abc".to_string(),
                vec![],
                vec![],
                vec![]
            ),
            0
        );
    }
}
