struct Solution;

impl Solution {
    /// Determines if each node's DFS string is a palindrome using polynomial rolling hash.
    ///
    /// # Intuition
    /// The DFS string for a node is formed by concatenating all children's DFS strings
    /// followed by the node's character. Instead of building the actual string, we compute
    /// forward and reverse hashes incrementally. A palindrome has identical forward and
    /// reverse hashes.
    ///
    /// # Approach
    /// 1. Build a Compressed Sparse Row (CSR) representation for cache-friendly tree traversal
    /// 2. Linearize tree nodes using BFS for efficient bottom-up processing
    /// 3. Precompute powers of the hash base for O(1) hash updates
    /// 4. Process nodes in reverse topological order (leaves to root):
    ///    - Forward hash: concatenate children hashes then add node character
    ///    - Reverse hash: add node character then concatenate children in reverse
    /// 5. Compare forward and reverse hashes to detect palindromes
    ///
    /// # Complexity
    /// - Time: O(n) - single pass through all nodes with O(1) operations per node
    /// - Space: O(n) - for adjacency list, hash tables, and result vector
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        if n == 0 {
            return vec![];
        }

        let chars = s.as_bytes();

        // Build CSR adjacency structure for cache-efficient traversal
        let (adjacency_list, adjacency_starts) = Self::build_csr_tree(&parent);

        // Find root and linearize tree with BFS
        let root = parent.iter().position(|&p| p == -1).unwrap_or(0);
        let bfs_order = Self::linearize_tree(root, &adjacency_list, &adjacency_starts);

        // Precompute hash base powers
        const MOD: u64 = 4_294_967_291; // Large prime (2^32 - 5)
        const BASE: u64 = 29;
        let hash_powers = Self::compute_hash_powers(n, BASE, MOD);

        // Compute forward and reverse hashes bottom-up
        let mut forward_hash = vec![0u64; n];
        let mut reverse_hash = vec![0u64; n];
        let mut subtree_size = vec![0usize; n];

        // Process nodes in reverse BFS order (post-order traversal)
        for &node in bfs_order.iter().rev() {
            let children_range = adjacency_starts[node]..adjacency_starts[node + 1];
            let children = &adjacency_list[children_range];

            // Character value (1-indexed for 'a' = 1, 'b' = 2, etc.)
            let char_value = (chars[node] - b'a' + 1) as u64;

            // Forward hash: children concatenated, then node character
            let (fwd_hash, total_size) = Self::compute_forward_hash(
                children,
                &forward_hash,
                &subtree_size,
                &hash_powers,
                char_value,
                MOD,
            );
            forward_hash[node] = fwd_hash;
            subtree_size[node] = total_size;

            // Reverse hash: node character, then children in reverse order
            reverse_hash[node] = Self::compute_reverse_hash(
                children,
                &reverse_hash,
                &subtree_size,
                &hash_powers,
                char_value,
                MOD,
            );
        }

        // Node's DFS string is palindrome iff forward hash equals reverse hash
        (0..n).map(|i| forward_hash[i] == reverse_hash[i]).collect()
    }

    /// Build Compressed Sparse Row representation of tree
    #[inline]
    fn build_csr_tree(parent: &[i32]) -> (Vec<usize>, Vec<usize>) {
        let n = parent.len();

        // Count children per node
        let mut child_count = vec![0usize; n];
        for (_child, &parent_id) in parent.iter().enumerate() {
            if parent_id != -1 {
                child_count[parent_id as usize] += 1;
            }
        }

        // Compute start positions for each node's children
        let mut starts = Vec::with_capacity(n + 1);
        starts.push(0);
        for &count in &child_count {
            starts.push(starts.last().unwrap() + count);
        }

        // Fill adjacency list (n-1 edges in tree)
        let total_edges = if n > 0 { n - 1 } else { 0 };
        let mut adjacency = vec![0usize; total_edges];
        let mut current_pos = starts[..n].to_vec();

        for (child, &parent_id) in parent.iter().enumerate() {
            if parent_id != -1 {
                let parent_idx = parent_id as usize;
                adjacency[current_pos[parent_idx]] = child;
                current_pos[parent_idx] += 1;
            }
        }

        (adjacency, starts)
    }

    /// Linearize tree using BFS traversal
    #[inline]
    fn linearize_tree(root: usize, adjacency: &[usize], starts: &[usize]) -> Vec<usize> {
        let n = starts.len() - 1;
        let mut order = Vec::with_capacity(n);
        order.push(root);

        let mut read_idx = 0;
        while read_idx < order.len() {
            let node = order[read_idx];
            read_idx += 1;

            // Add all children to BFS queue
            for &child in &adjacency[starts[node]..starts[node + 1]] {
                order.push(child);
            }
        }

        order
    }

    /// Precompute powers of hash base modulo MOD
    #[inline]
    fn compute_hash_powers(max_power: usize, base: u64, modulus: u64) -> Vec<u64> {
        let mut powers = Vec::with_capacity(max_power + 1);
        powers.push(1);

        for _ in 1..=max_power {
            let next_power = (powers.last().unwrap() * base) % modulus;
            powers.push(next_power);
        }

        powers
    }

    /// Compute forward hash: children concatenated then node character
    #[inline]
    fn compute_forward_hash(
        children: &[usize],
        forward_hash: &[u64],
        subtree_size: &[usize],
        hash_powers: &[u64],
        char_value: u64,
        modulus: u64,
    ) -> (u64, usize) {
        let mut hash = 0u64;
        let mut total_length = 0usize;

        // Concatenate children hashes
        for &child in children {
            hash = (hash + forward_hash[child] * hash_powers[total_length]) % modulus;
            total_length += subtree_size[child];
        }

        // Append node character
        hash = (hash + char_value * hash_powers[total_length]) % modulus;
        total_length += 1;

        (hash, total_length)
    }

    /// Compute reverse hash: node character then children in reverse
    #[inline]
    fn compute_reverse_hash(
        children: &[usize],
        reverse_hash: &[u64],
        subtree_size: &[usize],
        hash_powers: &[u64],
        char_value: u64,
        modulus: u64,
    ) -> u64 {
        let mut hash = char_value;
        let mut position = 1usize;

        // Concatenate children hashes in reverse order
        for &child in children.iter().rev() {
            hash = (hash + reverse_hash[child] * hash_powers[position]) % modulus;
            position += subtree_size[child];
        }

        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let parent = vec![-1, 0, 0, 1, 1, 2];
        let s = "aababa".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![true, true, false, true, true, true]
        );
    }

    #[test]
    fn example_two() {
        let parent = vec![-1, 0, 0, 0, 0];
        let s = "aabcb".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![true, true, true, true, true]
        );
    }

    #[test]
    fn single_node() {
        let parent = vec![-1];
        let s = "a".to_string();
        assert_eq!(Solution::find_answer(parent, s), vec![true]);
    }

    #[test]
    fn linear_chain() {
        let parent = vec![-1, 0, 1, 2];
        let s = "abba".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![true, true, false, true]
        );
    }

    #[test]
    fn empty_tree() {
        let parent = vec![];
        let s = String::new();
        assert_eq!(Solution::find_answer(parent, s), vec![]);
    }

    #[test]
    fn two_nodes_palindrome() {
        let parent = vec![-1, 0];
        let s = "aa".to_string();
        assert_eq!(Solution::find_answer(parent, s), vec![true, true]);
    }

    #[test]
    fn two_nodes_not_palindrome() {
        let parent = vec![-1, 0];
        let s = "ab".to_string();
        assert_eq!(Solution::find_answer(parent, s), vec![false, true]);
    }

    #[test]
    fn star_tree_all_same() {
        let parent = vec![-1, 0, 0, 0];
        let s = "aaaa".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![true, true, true, true]
        );
    }

    #[test]
    fn star_tree_alternating() {
        let parent = vec![-1, 0, 0, 0];
        let s = "abab".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![false, true, false, true]
        );
    }

    #[test]
    fn deep_chain_palindrome() {
        let parent = vec![-1, 0, 1, 2, 3, 4];
        let s = "abccba".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![true, false, false, false, false, true]
        );
    }

    #[test]
    fn balanced_binary_tree() {
        let parent = vec![-1, 0, 0, 1, 1, 2, 2];
        let s = "abababa".to_string();
        let result = Solution::find_answer(parent, s);
        assert_eq!(result.len(), 7);
        // Root should be palindrome due to symmetry
        assert!(result[0]);
    }

    #[test]
    fn all_different_chars() {
        let parent = vec![-1, 0, 0, 1];
        let s = "abcd".to_string();
        assert_eq!(
            Solution::find_answer(parent, s),
            vec![false, false, true, true]
        );
    }

    #[test]
    fn large_tree_single_char() {
        // All nodes have same character - all should be palindromes
        let parent = vec![-1, 0, 0, 1, 1, 2, 2, 3, 3];
        let s = "aaaaaaaaa".to_string();
        let result = Solution::find_answer(parent, s);
        assert_eq!(result, vec![true; 9]);
    }

    #[test]
    fn complex_tree_structure() {
        // Complex tree with mixed palindrome properties
        let parent = vec![-1, 0, 0, 1, 1, 2, 2, 3, 4];
        let s = "abcbabcba".to_string();
        let result = Solution::find_answer(parent, s);
        // Verify result has correct length
        assert_eq!(result.len(), 9);
    }
}
