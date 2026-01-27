impl Solution {
    /// Iterator-based run-length decoding using chunks and flat_map.
    ///
    /// # Intuition
    /// The input pairs `[freq, val]` at even/odd indices describe a run-length
    /// encoding. Processing pairs via `chunks(2)` and expanding each with
    /// `flat_map` yields the decoded list idiomatically.
    ///
    /// # Approach
    /// 1. Chunk the input into pairs of `[freq, val]`
    /// 2. For each pair, repeat `val` exactly `freq` times
    /// 3. Flatten all repetitions into the result
    ///
    /// # Complexity
    /// - Time: O(sum of frequencies)
    /// - Space: O(sum of frequencies) for the output
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        nums.chunks(2)
            .flat_map(|pair| std::iter::repeat(pair[1]).take(pair[0] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_decompression() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            vec![2, 4, 4, 4]
        );
    }

    #[test]
    fn single_pair() {
        assert_eq!(Solution::decompress_rl_elist(vec![1, 1]), vec![1]);
    }

    #[test]
    fn multiple_repeats() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![2, 1, 1, 3]),
            vec![1, 1, 3]
        );
    }
}
