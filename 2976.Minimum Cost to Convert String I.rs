impl Solution {
    /// Floyd-Warshall shortest path for character conversion costs.
    ///
    /// # Intuition
    /// Each character conversion can chain through intermediates (a→c→b).
    /// With only 26 letters, compute all-pairs shortest paths to find the
    /// minimum cost between any two characters.
    ///
    /// # Approach
    /// 1. Build a 26×26 distance matrix from direct conversion costs
    /// 2. Run Floyd-Warshall with early pruning when no path exists
    /// 3. Sum conversion costs for each differing position in source vs target
    ///
    /// # Complexity
    /// - Time: O(26³ + n) = O(n) where n is string length
    /// - Space: O(26²) = O(1)
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        const INF: i64 = i64::MAX / 2;
        const N: usize = 26;

        let mut dist = [[INF; N]; N];
        for i in 0..N {
            dist[i][i] = 0;
        }

        // Build graph from conversion rules
        let rules_len = original.len();
        for idx in 0..rules_len {
            let from = (original[idx] as u8 - b'a') as usize;
            let to = (changed[idx] as u8 - b'a') as usize;
            let co = cost[idx] as i64;
            if co < dist[from][to] {
                dist[from][to] = co;
            }
        }

        // Floyd-Warshall with early pruning
        for k in 0..N {
            let row_k = dist[k];
            for i in 0..N {
                let dik = dist[i][k];
                if dik >= INF {
                    continue;
                }
                for j in 0..N {
                    let dkj = row_k[j];
                    if dkj >= INF {
                        continue;
                    }
                    let cand = dik + dkj;
                    if cand < dist[i][j] {
                        dist[i][j] = cand;
                    }
                }
            }
        }

        // Convert to bytes once for faster access
        let src = source.as_bytes();
        let tgt = target.as_bytes();

        let mut total = 0i64;
        for (&s, &t) in src.iter().zip(tgt.iter()) {
            if s == t {
                continue;
            }
            let d = dist[(s - b'a') as usize][(t - b'a') as usize];
            if d >= INF {
                return -1;
            }
            total += d;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_conversion() {
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
    }

    #[test]
    fn multi_step_path() {
        assert_eq!(
            Solution::minimum_cost(
                "aaaa".to_string(),
                "bbbb".to_string(),
                vec!['a', 'c'],
                vec!['c', 'b'],
                vec![1, 2]
            ),
            12
        );
    }

    #[test]
    fn impossible_conversion() {
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "abce".to_string(),
                vec!['a'],
                vec!['e'],
                vec![10000]
            ),
            -1
        );
    }

    #[test]
    fn same_string() {
        assert_eq!(
            Solution::minimum_cost(
                "abc".to_string(),
                "abc".to_string(),
                vec!['a'],
                vec!['b'],
                vec![1]
            ),
            0
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(
            Solution::minimum_cost(
                "a".to_string(),
                "b".to_string(),
                vec!['a'],
                vec!['b'],
                vec![5]
            ),
            5
        );
    }

    #[test]
    fn shorter_path_wins() {
        assert_eq!(
            Solution::minimum_cost(
                "a".to_string(),
                "b".to_string(),
                vec!['a', 'a', 'c'],
                vec!['b', 'c', 'b'],
                vec![10, 1, 2]
            ),
            3
        );
    }
}
