/// Segment tree node storing run-length information for a range.
#[derive(Clone)]
struct Node {
    /// Character at the leftmost position.
    left_char: u8,
    /// Character at the rightmost position.
    right_char: u8,
    /// Length of the longest prefix run of the same character.
    prefix: i32,
    /// Length of the longest suffix run of the same character.
    suffix: i32,
    /// Length of the longest run of any single character in this range.
    max_run: i32,
    /// Total length of this range.
    len: i32,
}

impl Node {
    fn new(c: u8) -> Self {
        Node {
            left_char: c,
            right_char: c,
            prefix: 1,
            suffix: 1,
            max_run: 1,
            len: 1,
        }
    }

    /// Merge two adjacent nodes (left covers [l, m], right covers [m+1, r]).
    fn merge(left: &Node, right: &Node) -> Node {
        let mut node = Node {
            left_char: left.left_char,
            right_char: right.right_char,
            prefix: left.prefix,
            suffix: right.suffix,
            max_run: left.max_run.max(right.max_run),
            len: left.len + right.len,
        };

        // Extend prefix into right child if the entire left child is one run.
        if left.left_char == right.left_char && left.prefix == left.len {
            node.prefix = left.len + right.prefix;
        }

        // Extend suffix into left child if the entire right child is one run.
        if right.right_char == left.right_char && right.suffix == right.len {
            node.suffix = right.len + left.suffix;
        }

        // Merge across the boundary if the joining characters match.
        if left.right_char == right.left_char {
            node.max_run = node.max_run.max(left.suffix + right.prefix);
        }

        node
    }
}

struct SegTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegTree {
    fn build(s: &[u8]) -> Self {
        let n = s.len();
        let mut tree = vec![
            Node {
                left_char: 0,
                right_char: 0,
                prefix: 0,
                suffix: 0,
                max_run: 0,
                len: 0,
            };
            4 * n
        ];
        Self::build_range(&mut tree, s, 1, 0, n - 1);
        SegTree { n, tree }
    }

    fn build_range(tree: &mut Vec<Node>, s: &[u8], node: usize, l: usize, r: usize) {
        if l == r {
            tree[node] = Node::new(s[l]);
            return;
        }
        let mid = (l + r) / 2;
        Self::build_range(tree, s, node * 2, l, mid);
        Self::build_range(tree, s, node * 2 + 1, mid + 1, r);
        let left = tree[node * 2].clone();
        let right = tree[node * 2 + 1].clone();
        tree[node] = Node::merge(&left, &right);
    }

    fn update(&mut self, pos: usize, c: u8) {
        let n = self.n;
        Self::update_range(&mut self.tree, 1, 0, n - 1, pos, c);
    }

    fn update_range(tree: &mut Vec<Node>, node: usize, l: usize, r: usize, pos: usize, c: u8) {
        if l == r {
            tree[node] = Node::new(c);
            return;
        }
        let mid = (l + r) / 2;
        if pos <= mid {
            Self::update_range(tree, node * 2, l, mid, pos, c);
        } else {
            Self::update_range(tree, node * 2 + 1, mid + 1, r, pos, c);
        }
        let left = tree[node * 2].clone();
        let right = tree[node * 2 + 1].clone();
        tree[node] = Node::merge(&left, &right);
    }

    fn query_max(&self) -> i32 {
        self.tree[1].max_run
    }
}

impl Solution {
    /// Segment tree with run-length metadata for O((n + k) log n) total complexity.
    ///
    /// # Intuition
    /// Each query modifies a single character and asks for the global maximum run.
    /// A segment tree where every node stores prefix/suffix run lengths, boundary
    /// characters, and the local maximum run enables O(log n) point updates and
    /// O(1) root queries.
    ///
    /// # Approach
    /// 1. Build a segment tree over the characters of `s`. Each leaf holds a single
    ///    character with prefix = suffix = max_run = 1.
    /// 2. Merge two children by:
    ///    - Extending the parent's prefix if the entire left child is one run.
    ///    - Extending the parent's suffix if the entire right child is one run.
    ///    - Checking whether the boundary run (left.suffix + right.prefix) beats
    ///      the current max (only valid when boundary characters match).
    /// 3. For each query, update the character at the given index and read the
    ///    max_run at the root.
    ///
    /// # Complexity
    /// - Time: O((n + k) log n)
    /// - Space: O(n)
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let mut bytes: Vec<u8> = s.into_bytes();
        let qc: Vec<u8> = query_characters.into_bytes();
        let k = qc.len();

        let mut seg = SegTree::build(&bytes);
        let mut result = Vec::with_capacity(k);

        for i in 0..k {
            let pos = query_indices[i] as usize;
            let c = qc[i];
            bytes[pos] = c;
            seg.update(pos, c);
            result.push(seg.query_max());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::longest_repeating("babacc".to_string(), "bcb".to_string(), vec![1, 3, 3]),
            vec![3, 3, 4]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::longest_repeating("abyzz".to_string(), "aa".to_string(), vec![2, 1]),
            vec![2, 3]
        );
    }

    #[test]
    fn test_single_char_string() {
        assert_eq!(
            Solution::longest_repeating("a".to_string(), "b".to_string(), vec![0]),
            vec![1]
        );
    }

    #[test]
    fn test_all_same_char() {
        // "aaaa" -> update index 2 to 'b' -> "aaba" -> max = 2
        assert_eq!(
            Solution::longest_repeating("aaaa".to_string(), "b".to_string(), vec![2]),
            vec![2]
        );
    }

    #[test]
    fn test_restore_run() {
        // "aabaa" -> update index 2 to 'a' -> "aaaaa" -> max = 5
        assert_eq!(
            Solution::longest_repeating("aabaa".to_string(), "a".to_string(), vec![2]),
            vec![5]
        );
    }

    #[test]
    fn test_multiple_updates_same_index() {
        // "ab" -> update 0 to 'a' -> "ab" max=1; update 0 to 'b' -> "bb" max=2
        assert_eq!(
            Solution::longest_repeating("ab".to_string(), "ab".to_string(), vec![0, 0]),
            vec![1, 2]
        );
    }

    #[test]
    fn test_boundary_merge() {
        // "aba" -> update 1 to 'a' -> "aaa" max=3
        assert_eq!(
            Solution::longest_repeating("aba".to_string(), "a".to_string(), vec![1]),
            vec![3]
        );
    }
}
