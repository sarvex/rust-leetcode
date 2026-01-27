impl Solution {
    /// Finds the node with the highest edge score, preferring smaller index on tie.
    ///
    /// # Intuition
    /// Each edge `i -> edges[i]` contributes `i` to the score of `edges[i]`.
    /// Accumulate scores in a single pass while tracking the current best.
    ///
    /// # Approach
    /// 1. Maintain a score array indexed by node
    /// 2. For each edge, add source index to the destination's score
    /// 3. Track the best node greedily during the pass
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut score = vec![0i64; n];
        let mut best = 0usize;

        for (i, &j) in edges.iter().enumerate() {
            let j = j as usize;
            score[j] += i as i64;
            if score[j] > score[best] || (score[j] == score[best] && j < best) {
                best = j;
            }
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::edge_score(vec![2, 0, 0, 2]), 0);
    }

    #[test]
    fn test_self_loop() {
        assert_eq!(Solution::edge_score(vec![0, 0]), 0);
    }

    #[test]
    fn test_two_nodes() {
        assert_eq!(Solution::edge_score(vec![1, 0]), 0);
    }
}
