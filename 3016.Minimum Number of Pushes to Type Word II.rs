impl Solution {
    /// Greedy frequency-based assignment minimizes total pushes across 8 keys.
    ///
    /// # Intuition
    /// Each key (2–9) can hold unlimited letters, but the k-th letter on a key costs k pushes.
    /// With 8 keys, the first 8 most-frequent letters each cost 1 push, the next 8 cost 2, and so on.
    /// Sorting by descending frequency and assigning greedily is optimal.
    ///
    /// # Approach
    /// 1. Count frequency of each letter (26-element array suffices).
    /// 2. Sort frequencies in descending order.
    /// 3. For the letter at sorted index `i`, it sits at depth `i / 8 + 1` and contributes
    ///    `frequency * (i / 8 + 1)` to the total cost.
    ///
    /// # Complexity
    /// - Time: O(n + 26 log 26) = O(n)
    /// - Space: O(1) — fixed-size frequency array
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq = [0i32; 26];
        for b in word.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        freq.sort_unstable_by(|a, b| b.cmp(a));
        freq.iter()
            .enumerate()
            .map(|(i, &f)| f * (i as i32 / 8 + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_letters() {
        // "abcde": 5 distinct letters, each appears once, all fit at depth 1
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
    }

    #[test]
    fn test_repeated_letters() {
        // "xyzxyzxyzxyz": x=4, y=4, z=4; each maps to its own key at depth 1
        // cost = 4*1 + 4*1 + 4*1 = 12
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_string()), 12);
    }

    #[test]
    fn test_mixed_frequencies() {
        // "aabbccddeeffgghhiiiiii": a=2,b=2,c=2,d=2,e=2,f=2,g=2,h=2,i=6
        // sorted desc: i=6, a=b=c=d=e=f=g=h=2
        // i at index 0: depth 1 => 6*1 = 6
        // a..h at index 1-8: depth 1 => 2*1 each, but index 8 is depth 2
        // indices 0-7: depth 1 => i(6) + a(2)+b(2)+c(2)+d(2)+e(2)+f(2)+g(2) = 6+14 = 20
        // index 8: h(2) at depth 2 => 2*2 = 4
        // total = 24
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string()),
            24
        );
    }

    #[test]
    fn test_single_letter() {
        assert_eq!(Solution::minimum_pushes("a".to_string()), 1);
    }

    #[test]
    fn test_single_repeated_letter() {
        // All same letter, one unique char at depth 1
        assert_eq!(Solution::minimum_pushes("aaaa".to_string()), 4);
    }

    #[test]
    fn test_exactly_eight_distinct() {
        // 8 distinct letters all at depth 1
        assert_eq!(Solution::minimum_pushes("abcdefgh".to_string()), 8);
    }

    #[test]
    fn test_nine_distinct() {
        // First 8 at depth 1 (8 pushes), 9th at depth 2 (2 pushes) = 10
        assert_eq!(Solution::minimum_pushes("abcdefghi".to_string()), 10);
    }

    #[test]
    fn test_high_frequency_dominates() {
        // 'a' appears 100 times, 'b' once — 'a' should be at depth 1
        // cost = 100*1 + 1*1 = 101
        let mut word = "a".repeat(100);
        word.push('b');
        assert_eq!(Solution::minimum_pushes(word), 101);
    }
}
