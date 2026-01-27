use std::collections::HashMap;

impl Solution {
    /// Determines if the frog can cross the river using memoized DFS.
    ///
    /// # Intuition
    /// From each stone, the frog can jump k-1, k, or k+1 units. A DFS with
    /// memoization on (stone_index, last_jump) avoids redundant exploration.
    ///
    /// # Approach
    /// 1. Map each stone value to its index for O(1) lookups.
    /// 2. DFS from stone 0 with jump size 0.
    /// 3. For each state, try jumps of k-1, k, k+1 and cache the result.
    ///
    /// # Complexity
    /// - Time: O(n²) — each (index, jump) pair visited at most once
    /// - Space: O(n²) for the memoization table
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let pos: HashMap<i32, usize> = stones.iter().enumerate().map(|(i, &s)| (s, i)).collect();
        let mut memo = vec![vec![-1i8; n]; n];

        fn dfs(
            idx: usize,
            k: usize,
            n: usize,
            stones: &[i32],
            pos: &HashMap<i32, usize>,
            memo: &mut Vec<Vec<i8>>,
        ) -> bool {
            if idx == n - 1 {
                return true;
            }
            if memo[idx][k] != -1 {
                return memo[idx][k] == 1;
            }
            let k_i32 = k as i32;
            let found = (k_i32 - 1..=k_i32 + 1).any(|j| {
                j > 0
                    && pos
                        .get(&(stones[idx] + j))
                        .map_or(false, |&next| dfs(next, j as usize, n, stones, pos, memo))
            });
            memo[idx][k] = found as i8;
            found
        }

        dfs(0, 0, n, &stones, &pos, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_cross() {
        assert!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
    }

    #[test]
    fn test_cannot_cross() {
        assert!(!Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]));
    }

    #[test]
    fn test_two_stones() {
        assert!(Solution::can_cross(vec![0, 1]));
    }
}
