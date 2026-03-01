impl Solution {
    /// Count distinct non-empty sequences via backtracking on letter frequencies.
    ///
    /// # Intuition
    /// Each sequence is built by choosing a letter at each step. Duplicate letters
    /// yield the same sequence when chosen in the same order, so we count by
    /// iterating over unique letters and recursing on remaining counts.
    ///
    /// # Approach
    /// 1. Build a frequency array for 'A'..='Z'.
    /// 2. DFS: for each letter with count > 0, use one occurrence (count this
    ///    sequence), recurse, then restore. Sum over all choices.
    ///
    /// # Complexity
    /// - Time: O(26^L) in the worst case where L is total tiles; in practice
    ///   bounded by distinct permutations of the multiset.
    /// - Space: O(1) for the frequency array plus recursion depth O(n).
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut freq = [0i32; 26];
        for b in tiles.bytes() {
            freq[(b - b'A') as usize] += 1;
        }

        fn dfs(freq: &mut [i32; 26]) -> i32 {
            let mut count = 0;
            for i in 0..26 {
                if freq[i] > 0 {
                    freq[i] -= 1;
                    count += 1 + dfs(freq);
                    freq[i] += 1;
                }
            }
            count
        }

        dfs(&mut freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_aab() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    }

    #[test]
    fn example_aaabbc() {
        assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
    }

    #[test]
    fn two_same() {
        assert_eq!(Solution::num_tile_possibilities("AA".to_string()), 2);
    }

    #[test]
    fn two_diff() {
        assert_eq!(Solution::num_tile_possibilities("AB".to_string()), 4);
    }
}
