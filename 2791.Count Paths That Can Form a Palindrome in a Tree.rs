use std::collections::HashMap;

impl Solution {
    /// Prefix-parity masks with hash counting.
    ///
    /// # Intuition
    /// A path can be permuted into a palindrome iff at most one character has an odd count.
    /// Parity along any path equals XOR of prefix parities from the root.
    ///
    /// # Approach
    /// 1. Build children lists from `parent`, then DFS from root to compute a 26-bit parity mask
    ///    for every node (bit c is 1 if char c appears odd times on the root->node path).
    /// 2. For each node mask m, count previously seen nodes with mask m (XOR 0) and with
    ///    masks m ^ (1<<c) for all letters (XOR has exactly one bit). Add these counts to
    ///    the answer, then record m in the frequency map.
    ///
    /// # Complexity
    /// - Time: O(n * 26)
    /// - Space: O(n)
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        let node_count = parent.len();
        if node_count <= 1 {
            return 0;
        }

        let mut children = vec![Vec::new(); node_count];
        for child in 1..node_count {
            let parent_index = parent[child] as usize;
            children[parent_index].push(child);
        }

        let bytes = s.as_bytes();
        let mut masks = vec![0u32; node_count];
        let mut stack = Vec::with_capacity(node_count);
        stack.push((0usize, 0u32));

        while let Some((node, mask)) = stack.pop() {
            masks[node] = mask;
            for &child in &children[node] {
                let bit_index = (bytes[child] - b'a') as u32;
                let child_mask = mask ^ (1u32 << bit_index);
                stack.push((child, child_mask));
            }
        }

        let mut total = 0i64;
        let mut freq: HashMap<u32, i64> = HashMap::with_capacity(node_count * 2);

        for &mask in &masks {
            if let Some(count) = freq.get(&mask) {
                total += *count;
            }
            for bit in 0..26u32 {
                let alt = mask ^ (1u32 << bit);
                if let Some(count) = freq.get(&alt) {
                    total += *count;
                }
            }
            *freq.entry(mask).or_insert(0) += 1;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let parent = vec![-1, 0, 0, 1, 1, 2];
        let s = "acaabc".to_string();
        assert_eq!(Solution::count_palindrome_paths(parent, s), 8);
    }

    #[test]
    fn test_example_two() {
        let parent = vec![-1, 0, 0, 0, 0];
        let s = "aaaaa".to_string();
        assert_eq!(Solution::count_palindrome_paths(parent, s), 10);
    }

    #[test]
    fn test_single_node() {
        let parent = vec![-1];
        let s = "a".to_string();
        assert_eq!(Solution::count_palindrome_paths(parent, s), 0);
    }

    #[test]
    fn test_chain_mixed_chars() {
        let parent = vec![-1, 0, 1];
        let s = "aba".to_string();
        assert_eq!(Solution::count_palindrome_paths(parent, s), 2);
    }
}
