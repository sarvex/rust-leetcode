impl Solution {
    /// Greedy frequency reduction for unique character counts.
    ///
    /// # Intuition
    /// Sort frequencies in descending order. For each frequency, if it
    /// equals or exceeds the previous one, reduce it to one less (minimum 0).
    /// The total reduction count is the answer.
    ///
    /// # Approach
    /// 1. Count character frequencies
    /// 2. Sort frequencies descending
    /// 3. Greedily reduce each frequency to be strictly less than the previous
    ///
    /// # Complexity
    /// - Time: O(n + 26 log 26) ≈ O(n)
    /// - Space: O(1) — 26-element array
    pub fn min_deletions(s: String) -> i32 {
        let mut freq = [0i32; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        freq.sort_unstable_by(|a, b| b.cmp(a));

        let mut deletions = 0;
        for i in 1..26 {
            while freq[i] >= freq[i - 1] && freq[i] > 0 {
                freq[i] -= 1;
                deletions += 1;
            }
        }

        deletions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_deletion() {
        assert_eq!(Solution::min_deletions("aab".to_string()), 0);
    }

    #[test]
    fn multiple_deletions() {
        assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
    }

    #[test]
    fn cascading_deletions() {
        assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
    }
}
