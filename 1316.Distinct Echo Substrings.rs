use std::collections::HashSet;

const BASE: u64 = 131;

impl Solution {
    /// Rolling hash to count distinct echo substrings.
    ///
    /// # Intuition
    /// An echo substring has the form `a + a` where both halves are identical.
    /// By precomputing polynomial hash values, we can compare any two substrings
    /// in O(1) and collect distinct first-halves via a HashSet.
    ///
    /// # Approach
    /// 1. Precompute power and prefix hash arrays
    /// 2. For each even-length substring, compare hash of first and second half
    /// 3. Insert matching first-half hashes into a HashSet for deduplication
    ///
    /// # Complexity
    /// - Time: O(n²) iterating all valid substring pairs
    /// - Space: O(n²) worst case for the hash set
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let n = text.len();
        let bytes = text.as_bytes();
        let mut power = vec![1u64; n + 1];
        let mut hash = vec![0u64; n + 1];

        for i in 0..n {
            let c = (bytes[i] - b'a' + 1) as u64;
            power[i + 1] = power[i].wrapping_mul(BASE);
            hash[i + 1] = hash[i].wrapping_mul(BASE).wrapping_add(c);
        }

        let get_hash = |l: usize, r: usize| -> u64 {
            hash[r].wrapping_sub(hash[l - 1].wrapping_mul(power[r - l + 1]))
        };

        let mut seen = HashSet::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                if (j - i) % 2 != 0 {
                    continue;
                }
                let mid = i + (j - i) / 2;
                let left = get_hash(i + 1, mid + 1);
                let right = get_hash(mid + 2, j + 1);
                if left == right {
                    seen.insert(left);
                }
            }
        }

        seen.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_echo() {
        assert_eq!(
            Solution::distinct_echo_substrings("abcabcabc".to_string()),
            3
        );
    }

    #[test]
    fn single_repeat() {
        assert_eq!(Solution::distinct_echo_substrings("abab".to_string()), 1);
    }

    #[test]
    fn no_echo() {
        assert_eq!(Solution::distinct_echo_substrings("abc".to_string()), 0);
    }
}
