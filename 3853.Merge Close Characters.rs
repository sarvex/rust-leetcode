impl Solution {
    /// Simulates repeated merging of close equal characters.
    ///
    /// # Intuition
    /// With n <= 100, we can directly simulate the process: repeatedly
    /// find the first mergeable pair (smallest left index, then smallest
    /// right index) and remove the right character.
    ///
    /// # Approach
    /// 1. Convert string to a mutable `Vec<char>`.
    /// 2. Loop: scan all pairs (i, j) with j - i <= k and chars[i] == chars[j].
    ///    Pick the first such pair (by left then right index) and remove index j.
    /// 3. Repeat until no merge is found.
    ///
    /// # Complexity
    /// - Time: O(n^3) worst case — each merge is O(n^2) scan + O(n) removal,
    ///   up to O(n) merges. Fine for n <= 100.
    /// - Space: O(n) — character vector.
    pub fn merge_characters(s: String, k: i32) -> String {
        let velunorati = s;
        let mut chars: Vec<char> = velunorati.chars().collect();
        let k = k as usize;

        loop {
            let mut found = false;
            'outer: for i in 0..chars.len() {
                for j in (i + 1)..chars.len().min(i + k + 1) {
                    if chars[i] == chars[j] {
                        chars.remove(j);
                        found = true;
                        break 'outer;
                    }
                }
            }
            if !found {
                break;
            }
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::merge_characters("abca".to_string(), 3),
            "abc".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::merge_characters("aabca".to_string(), 2),
            "abca".to_string()
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::merge_characters("yybyzybz".to_string(), 2),
            "ybzybz".to_string()
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(
            Solution::merge_characters("a".to_string(), 1),
            "a".to_string()
        );
    }

    #[test]
    fn no_merges() {
        assert_eq!(
            Solution::merge_characters("abcdef".to_string(), 5),
            "abcdef".to_string()
        );
    }

    #[test]
    fn all_same_within_k() {
        assert_eq!(
            Solution::merge_characters("aaaa".to_string(), 4),
            "a".to_string()
        );
    }

    #[test]
    fn all_same_k_one() {
        // "aaaa" k=1: merge (0,1)→"aaa", merge (0,1)→"aa", merge (0,1)→"a"
        assert_eq!(
            Solution::merge_characters("aaaa".to_string(), 1),
            "a".to_string()
        );
    }

    #[test]
    fn k_equals_length() {
        assert_eq!(
            Solution::merge_characters("abcba".to_string(), 5),
            "abc".to_string()
        );
    }
}
