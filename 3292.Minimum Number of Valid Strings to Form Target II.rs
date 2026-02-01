impl Solution {
    /// Minimize segments with longest prefix matches and greedy jumping.
    ///
    /// # Intuition
    /// Any valid piece is a prefix of some word. If a target suffix matches a prefix of
    /// length `L`, then every shorter length is also valid, so only the longest match per
    /// position matters.
    ///
    /// # Approach
    /// - Hash every word prefix with a double rolling hash, grouped by length.
    /// - For each start index, binary search the maximum length whose hash appears in the
    ///   prefix set (validity is monotone).
    /// - Interpret these maxima as jump lengths and apply the greedy Jump Game II strategy
    ///   to minimize the number of pieces.
    ///
    /// # Complexity
    /// - Time: O((W + T) + T log L), where `W` is total word length.
    ///   `T` is target length, and `L` is the maximum word length.
    /// - Space: O(W + T)
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        const MOD1: u64 = 1_000_000_007;
        const MOD2: u64 = 1_000_000_009;
        const BASE1: u64 = 911_382_323;
        const BASE2: u64 = 972_663_749;

        #[inline]
        fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
            ((a as u128 * b as u128) % modulus as u128) as u64
        }

        #[inline]
        fn combine(h1: u64, h2: u64) -> u64 {
            (h1 << 32) | h2
        }

        #[inline]
        fn slice_hash(
            prefix: &[u64],
            powers: &[u64],
            left: usize,
            right: usize,
            modulus: u64,
        ) -> u64 {
            let removed = mod_mul(prefix[left], powers[right - left], modulus);
            if prefix[right] >= removed {
                prefix[right] - removed
            } else {
                prefix[right] + modulus - removed
            }
        }

        let target_bytes = target.as_bytes();
        let target_len = target_bytes.len();
        if target_len == 0 {
            return 0;
        }

        let max_word_len = words.iter().map(|word| word.len()).max().unwrap_or(0);
        let mut prefix_hashes = vec![Vec::<u64>::new(); max_word_len + 1];

        for word in words {
            let mut hash1 = 0_u64;
            let mut hash2 = 0_u64;
            for (index, byte) in word.bytes().enumerate() {
                let value = (byte - b'a' + 1) as u64;
                hash1 = (mod_mul(hash1, BASE1, MOD1) + value) % MOD1;
                hash2 = (mod_mul(hash2, BASE2, MOD2) + value) % MOD2;
                prefix_hashes[index + 1].push(combine(hash1, hash2));
            }
        }

        for hashes in &mut prefix_hashes {
            if hashes.len() > 1 {
                hashes.sort_unstable();
                hashes.dedup();
            }
        }

        let mut pow1 = vec![1_u64; target_len + 1];
        let mut pow2 = vec![1_u64; target_len + 1];
        for i in 0..target_len {
            pow1[i + 1] = mod_mul(pow1[i], BASE1, MOD1);
            pow2[i + 1] = mod_mul(pow2[i], BASE2, MOD2);
        }

        let mut prefix1 = vec![0_u64; target_len + 1];
        let mut prefix2 = vec![0_u64; target_len + 1];
        for i in 0..target_len {
            let value = (target_bytes[i] - b'a' + 1) as u64;
            prefix1[i + 1] = (mod_mul(prefix1[i], BASE1, MOD1) + value) % MOD1;
            prefix2[i + 1] = (mod_mul(prefix2[i], BASE2, MOD2) + value) % MOD2;
        }

        let mut max_match = vec![0_usize; target_len];
        for start in 0..target_len {
            let max_possible = max_word_len.min(target_len - start);
            let mut low = 0_usize;
            let mut high = max_possible;
            while low < high {
                let mid = (low + high + 1) / 2;
                let hash1 = slice_hash(&prefix1, &pow1, start, start + mid, MOD1);
                let hash2 = slice_hash(&prefix2, &pow2, start, start + mid, MOD2);
                let key = combine(hash1, hash2);
                if prefix_hashes[mid].binary_search(&key).is_ok() {
                    low = mid;
                } else {
                    high = mid - 1;
                }
            }
            max_match[start] = low;
        }

        let mut steps = 0_usize;
        let mut current_end = 0_usize;
        let mut farthest = 0_usize;
        for i in 0..target_len {
            if i > farthest {
                return -1;
            }
            farthest = farthest.max(i + max_match[i]);
            if i == current_end {
                steps += 1;
                current_end = farthest;
                if current_end >= target_len {
                    return steps as i32;
                }
                if current_end == i {
                    return -1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::min_valid_strings(
                vec!["abc".to_string(), "aaaaa".to_string(), "bcdef".to_string()],
                "aabcdabc".to_string(),
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_valid_strings(
                vec!["abababab".to_string(), "ab".to_string()],
                "ababaababa".to_string(),
            ),
            2
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::min_valid_strings(vec!["abcdef".to_string()], "xyz".to_string()),
            -1
        );
    }

    #[test]
    fn single_character_target() {
        assert_eq!(
            Solution::min_valid_strings(vec!["a".to_string()], "a".to_string()),
            1
        );
    }

    #[test]
    fn unreachable_in_middle() {
        assert_eq!(
            Solution::min_valid_strings(
                vec!["ab".to_string(), "abc".to_string()],
                "abx".to_string(),
            ),
            -1
        );
    }
}
