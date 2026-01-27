impl Solution {
    /// Shortest superstring containing three given strings.
    ///
    /// # Intuition
    /// Try all 6 permutations of merging order. For each permutation, greedily
    /// merge two strings by finding the longest overlap, then merge the result
    /// with the third. Return the lexicographically smallest shortest result.
    ///
    /// # Approach
    /// 1. Define a merge function that overlaps two strings maximally.
    /// 2. Enumerate all 6 permutations of (a, b, c).
    /// 3. For each, merge first two, then merge result with third.
    /// 4. Track the shortest (then lexicographically smallest) result.
    ///
    /// # Complexity
    /// - Time: O(n²) where n is total length of the three strings
    /// - Space: O(n)
    pub fn minimum_string(a: String, b: String, c: String) -> String {
        let strings = [&a, &b, &c];
        let perms: [[usize; 3]; 6] = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];

        perms
            .iter()
            .map(|&[i, j, k]| {
                let merged = Self::merge(strings[i], strings[j]);
                Self::merge(&merged, strings[k])
            })
            .min_by(|x, y| x.len().cmp(&y.len()).then_with(|| x.cmp(y)))
            .unwrap()
    }

    fn merge(s1: &str, s2: &str) -> String {
        if s1.contains(s2) {
            return s1.to_string();
        }
        if s2.contains(s1) {
            return s2.to_string();
        }
        let overlap = (1..s1.len())
            .find(|&i| s2.starts_with(&s1[i..]))
            .unwrap_or(s1.len());
        let mut result = String::with_capacity(overlap + s2.len());
        result.push_str(&s1[..overlap]);
        result.push_str(s2);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_strings() {
        assert_eq!(
            Solution::minimum_string("abc".to_string(), "bca".to_string(), "aaa".to_string()),
            "aaabca"
        );
    }

    #[test]
    fn one_contains_another() {
        assert_eq!(
            Solution::minimum_string("ab".to_string(), "ba".to_string(), "aba".to_string()),
            "aba"
        );
    }
}
