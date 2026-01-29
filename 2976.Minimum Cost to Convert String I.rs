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
        for ((&o, &c), &co) in original.iter().zip(&changed).zip(&cost) {
            let from = (o as u8 - b'a') as usize;
            let to = (c as u8 - b'a') as usize;
            dist[from][to] = dist[from][to].min(co as i64);
        }

        // Floyd-Warshall with early pruning
        for k in 0..N {
            for i in 0..N {
                if dist[i][k] < INF {
                    for j in 0..N {
                        if dist[k][j] < INF {
                            dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                        }
                    }
                }
            }
        }

        // Convert to bytes once for faster access
        let src = source.as_bytes();
        let tgt = target.as_bytes();

        let mut total = 0i64;
        for i in 0..src.len() {
            if src[i] != tgt[i] {
                let d = dist[(src[i] - b'a') as usize][(tgt[i] - b'a') as usize];
                if d >= INF {
                    return -1;
                }
                total += d;
            }
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
