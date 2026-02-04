impl Solution {
    /// Build the lexicographically smallest string that matches the LCP matrix.
    ///
    /// # Intuition
    /// `lcp[i][j] > 0` means the first characters of the suffixes are equal, so indices belong to
    /// the same equivalence class. The LCP matrix depends only on where characters are equal, not
    /// on which letters are chosen, so assigning distinct letters to each class suffices.
    ///
    /// # Approach
    /// 1. Validate symmetry, diagonal values, and bounds.
    /// 2. Union indices with `lcp[i][j] > 0`, then reject any `lcp[i][j] == 0` within a class.
    /// 3. Order classes by first occurrence and assign letters `a..z`; fail if more than 26.
    /// 4. Recompute the LCP matrix from the constructed string to confirm an exact match.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n^2)
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        if n == 0 {
            return String::new();
        }

        for row in &lcp {
            if row.len() != n {
                return String::new();
            }
        }

        for i in 0..n {
            if lcp[i][i] != (n - i) as i32 {
                return String::new();
            }
            for j in (i + 1)..n {
                let value = lcp[i][j];
                if value != lcp[j][i] {
                    return String::new();
                }
                if value < 0 || value > (n - i.max(j)) as i32 {
                    return String::new();
                }
            }
        }

        let mut dsu = DisjointSet::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                if lcp[i][j] > 0 {
                    dsu.union(i, j);
                }
            }
        }

        let mut root_of = vec![0usize; n];
        for i in 0..n {
            root_of[i] = dsu.find(i);
        }

        for i in 0..n {
            for j in (i + 1)..n {
                if lcp[i][j] == 0 && root_of[i] == root_of[j] {
                    return String::new();
                }
            }
        }

        let mut first_index = vec![usize::MAX; n];
        for (index, &root) in root_of.iter().enumerate() {
            if index < first_index[root] {
                first_index[root] = index;
            }
        }

        let mut roots: Vec<(usize, usize)> = first_index
            .iter()
            .enumerate()
            .filter_map(|(root, &first)| {
                if first == usize::MAX {
                    None
                } else {
                    Some((first, root))
                }
            })
            .collect();
        roots.sort_by_key(|(first, _)| *first);

        if roots.len() > 26 {
            return String::new();
        }

        let mut char_for_root = vec!['?'; n];
        for (offset, &(_, root)) in roots.iter().enumerate() {
            char_for_root[root] = (b'a' + offset as u8) as char;
        }

        let word: String = root_of.iter().map(|&root| char_for_root[root]).collect();

        let word_bytes = word.as_bytes();
        let mut computed = vec![vec![0i32; n + 1]; n + 1];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if word_bytes[i] == word_bytes[j] {
                    computed[i][j] = computed[i + 1][j + 1] + 1;
                }
                if computed[i][j] != lcp[i][j] {
                    return String::new();
                }
            }
        }

        word
    }
}

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, node: usize) -> usize {
        let parent = self.parent[node];
        if parent == node {
            return node;
        }
        let root = self.find(parent);
        self.parent[node] = root;
        root
    }

    fn union(&mut self, left: usize, right: usize) {
        let mut left_root = self.find(left);
        let mut right_root = self.find(right);
        if left_root == right_root {
            return;
        }
        if self.rank[left_root] < self.rank[right_root] {
            std::mem::swap(&mut left_root, &mut right_root);
        }
        self.parent[right_root] = left_root;
        if self.rank[left_root] == self.rank[right_root] {
            self.rank[left_root] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let lcp = vec![
            vec![4, 0, 2, 0],
            vec![0, 3, 0, 1],
            vec![2, 0, 2, 0],
            vec![0, 1, 0, 1],
        ];
        assert_eq!(Solution::find_the_string(lcp), "abab");
    }

    #[test]
    fn example_two() {
        let lcp = vec![
            vec![4, 3, 2, 1],
            vec![3, 3, 2, 1],
            vec![2, 2, 2, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(Solution::find_the_string(lcp), "aaaa");
    }

    #[test]
    fn example_three() {
        let lcp = vec![
            vec![4, 3, 2, 1],
            vec![3, 3, 2, 1],
            vec![2, 2, 2, 1],
            vec![1, 1, 1, 3],
        ];
        assert_eq!(Solution::find_the_string(lcp), "");
    }

    #[test]
    fn single_character() {
        let lcp = vec![vec![1]];
        assert_eq!(Solution::find_the_string(lcp), "a");
    }

    #[test]
    fn too_many_classes() {
        let n = 27;
        let mut lcp = vec![vec![0i32; n]; n];
        for i in 0..n {
            lcp[i][i] = (n - i) as i32;
        }
        assert_eq!(Solution::find_the_string(lcp), "");
    }
}
