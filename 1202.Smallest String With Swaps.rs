impl Solution {
    /// Union-Find grouping with sorted character assignment.
    ///
    /// # Intuition
    /// Characters connected by swap pairs (directly or transitively) can be
    /// rearranged freely within their group. Union-Find discovers connected
    /// components, then sorting each group's characters and assigning them
    /// greedily yields the lexicographically smallest result.
    ///
    /// # Approach
    /// 1. Build a Union-Find structure over all indices
    /// 2. Union indices from each swap pair
    /// 3. Group characters by their root representative
    /// 4. Sort each group's characters in descending order (pop from back)
    /// 5. Reconstruct the string by popping the smallest available character
    ///    from each index's group
    ///
    /// # Complexity
    /// - Time: O(n log n + m α(n)) where m is pairs count, α is inverse Ackermann
    /// - Space: O(n) for Union-Find and character buckets
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut parent: Vec<usize> = (0..n).collect();

        for pair in &pairs {
            Self::union(pair[0] as usize, pair[1] as usize, &mut parent);
        }

        let mut groups: Vec<Vec<u8>> = vec![Vec::new(); n];
        for (i, &b) in bytes.iter().enumerate() {
            let root = Self::find(i, &mut parent);
            groups[root].push(b);
        }

        for group in &mut groups {
            group.sort_unstable_by(|a, b| b.cmp(a));
        }

        let mut result = String::with_capacity(n);
        for i in 0..n {
            let root = Self::find(i, &mut parent);
            result.push(groups[root].pop().unwrap() as char);
        }

        result
    }

    fn find(x: usize, parent: &mut Vec<usize>) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent[x], parent);
        }
        parent[x]
    }

    fn union(x: usize, y: usize, parent: &mut Vec<usize>) {
        let px = Self::find(x, parent);
        let py = Self::find(y, parent);
        parent[px] = py;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_connected_components() {
        assert_eq!(
            Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]],),
            "bacd"
        );
    }

    #[test]
    fn transitive_swaps() {
        assert_eq!(
            Solution::smallest_string_with_swaps(
                "dcab".to_string(),
                vec![vec![0, 3], vec![1, 2], vec![0, 2]],
            ),
            "abcd"
        );
    }

    #[test]
    fn single_character() {
        assert_eq!(
            Solution::smallest_string_with_swaps("a".to_string(), vec![]),
            "a"
        );
    }

    #[test]
    fn no_pairs() {
        assert_eq!(
            Solution::smallest_string_with_swaps("cba".to_string(), vec![]),
            "cba"
        );
    }
}
