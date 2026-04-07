impl Solution {
    /// Answers palindromic path queries using Euler tour LCA and XOR bitmask Fenwick tree.
    ///
    /// # Intuition
    /// A string can be rearranged into a palindrome iff at most one character has odd frequency.
    /// Tracking character parities as a 26-bit XOR bitmask, the path is palindromic iff the
    /// resulting mask has at most one bit set (i.e., it is zero or a power of two).
    ///
    /// # Approach
    /// 1. Build the tree and run an iterative DFS that simultaneously computes the Euler tour
    ///    for LCA (recording `(depth, node)` pairs) and records `in_time`/`out_time` plus
    ///    initial `root_xor` (XOR of character bitmasks from root to each node).
    /// 2. Build a flat sparse table over the Euler tour for O(1) LCA via range-min query
    ///    on depth. A precomputed log table avoids `ilog2()` calls in the hot path.
    /// 3. Use a Fenwick tree (BIT) with XOR to handle character updates efficiently.
    ///    Changing node u's character XORs a delta into the contiguous Euler-tour range
    ///    `[in_time[u], out_time[u]]`.
    /// 4. For each query `(u, v)`: compute `path_xor = eff(u) ^ eff(v) ^ (1 << char[lca])`,
    ///    where `eff(x) = root_xor[x] ^ bit_query(in_time[x])`.
    ///    The path is palindrome-rearrangeable iff `mask & (mask - 1) == 0`.
    ///
    /// # Complexity
    /// - Time: O(n log n + q log n)
    /// - Space: O(n log n)
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let n = n as usize;
        if n == 0 {
            return Vec::new();
        }

        let s_bytes = s.as_bytes();
        let mut cur_chars = Vec::with_capacity(n);
        for &b in s_bytes {
            cur_chars.push(b - b'a');
        }

        let mut deg = vec![0u32; n];
        for e in &edges {
            deg[e[0] as usize] += 1;
            deg[e[1] as usize] += 1;
        }
        let mut adj: Vec<Vec<u32>> = (0..n)
            .map(|i| Vec::with_capacity(deg[i] as usize))
            .collect();
        for e in &edges {
            let (u, v) = (e[0] as u32, e[1] as u32);
            adj[u as usize].push(v);
            adj[v as usize].push(u);
        }
        drop(deg);

        // Euler entries packed as (depth << 32 | node) in u64 for single-compare RMQ
        let euler_cap = 2 * n - 1;
        let mut euler = Vec::with_capacity(euler_cap);
        let mut first = vec![0u32; n];
        let mut in_time = vec![0u32; n];
        let mut out_time = vec![0u32; n];
        let mut root_xor = vec![0u32; n];
        let mut timer = 0u32;

        let mut stack: Vec<(u32, u32, u32)> = Vec::with_capacity(n);
        let mut depth = vec![0u32; n];

        in_time[0] = timer;
        timer += 1;
        root_xor[0] = 1 << cur_chars[0];
        first[0] = 0;
        euler.push(((0u64) << 32) | 0u64);
        depth[0] = 0;
        stack.push((0, u32::MAX, 0));

        while let Some(frame) = stack.last_mut() {
            let node = frame.0 as usize;
            let parent = frame.1;
            let child_idx = frame.2 as usize;

            let adj_len = adj[node].len();
            let mut found_child = false;
            let mut idx = child_idx;
            while idx < adj_len {
                let neighbor = adj[node][idx];
                idx += 1;
                if neighbor != parent {
                    frame.2 = idx as u32;
                    let nb = neighbor as usize;
                    let d = depth[node] + 1;
                    depth[nb] = d;
                    in_time[nb] = timer;
                    timer += 1;
                    root_xor[nb] = root_xor[node] ^ (1u32 << cur_chars[nb]);
                    first[nb] = euler.len() as u32;
                    euler.push((d as u64) << 32 | neighbor as u64);
                    stack.push((neighbor, node as u32, 0));
                    found_child = true;
                    break;
                }
            }

            if !found_child {
                out_time[node] = timer - 1;
                stack.pop();
                if let Some(pf) = stack.last() {
                    let pnode = pf.0 as usize;
                    euler.push((depth[pnode] as u64) << 32 | pnode as u64);
                }
            }
        }
        drop(depth);

        // Flat sparse table: rmq[k * euler_len + i] = min of euler[i..i + 2^k]
        let euler_len = euler.len();
        let max_log = if euler_len <= 1 {
            0
        } else {
            euler_len.ilog2() as usize
        };

        let mut log_table = vec![0u32; euler_len + 1];
        for i in 2..=euler_len {
            log_table[i] = log_table[i >> 1] + 1;
        }

        let table_size = (max_log + 1) * euler_len;
        let mut rmq = Vec::with_capacity(table_size);
        rmq.extend_from_slice(&euler);

        for k in 1..=max_log {
            let step = 1usize << (k - 1);
            let prev_offset = (k - 1) * euler_len;
            let size = euler_len + 1 - (1 << k);
            for i in 0..size {
                let a = rmq[prev_offset + i];
                let b = rmq[prev_offset + i + step];
                rmq.push(if a <= b { a } else { b });
            }
            for _ in size..euler_len {
                rmq.push(u64::MAX);
            }
        }

        #[inline(always)]
        fn rmq_query(rmq: &[u64], log_table: &[u32], euler_len: usize, l: usize, r: usize) -> u32 {
            let span = r - l + 1;
            let k = log_table[span] as usize;
            let offset = k * euler_len;
            let a = rmq[offset + l];
            let b = rmq[offset + r + 1 - (1 << k)];
            (if a <= b { a } else { b }) as u32
        }

        let bit_len = n + 2;
        let mut bit = vec![0u32; bit_len];

        #[inline(always)]
        fn bit_update(bit: &mut [u32], mut i: usize, delta: u32) {
            i += 1;
            while i < bit.len() {
                bit[i] ^= delta;
                i += i & i.wrapping_neg();
            }
        }

        #[inline(always)]
        fn bit_query(bit: &[u32], mut i: usize) -> u32 {
            i += 1;
            let mut r = 0u32;
            while i > 0 {
                r ^= bit[i];
                i &= i - 1;
            }
            r
        }

        #[inline(always)]
        fn parse_uint(bytes: &[u8], start: usize) -> (usize, usize) {
            let mut val = 0usize;
            let mut i = start;
            while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
                val = val * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            (val, i)
        }

        let lca = |u: usize, v: usize| -> usize {
            let fu = first[u] as usize;
            let fv = first[v] as usize;
            let (l, r) = if fu <= fv { (fu, fv) } else { (fv, fu) };
            rmq_query(&rmq, &log_table, euler_len, l, r) as usize
        };

        let mut results = Vec::with_capacity(queries.len());

        for q in &queries {
            let qb = q.as_bytes();
            if qb[0] == b'u' {
                let (node, pos) = parse_uint(qb, 7);
                let new_char = qb[pos + 1] - b'a';
                let old_char = cur_chars[node];
                if old_char != new_char {
                    let delta = (1u32 << old_char) ^ (1u32 << new_char);
                    bit_update(&mut bit, in_time[node] as usize, delta);
                    bit_update(&mut bit, out_time[node] as usize + 1, delta);
                    cur_chars[node] = new_char;
                }
            } else {
                let (u, pos) = parse_uint(qb, 6);
                let (v, _) = parse_uint(qb, pos + 1);
                if u == v {
                    results.push(true);
                    continue;
                }
                let l = lca(u, v);
                let eff_u = root_xor[u] ^ bit_query(&bit, in_time[u] as usize);
                let eff_v = root_xor[v] ^ bit_query(&bit, in_time[v] as usize);
                let path_xor = eff_u ^ eff_v ^ (1u32 << cur_chars[l]);
                results.push(path_xor & path_xor.wrapping_sub(1) == 0);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_basic_tree() {
        // Tree: 0-1-2, chars "abc"
        // Path 0->2: "abc" -> not palindromic (3 distinct chars)
        // Path 0->1: "ab" -> not palindromic
        // Path 1->1: "b" -> palindromic
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let s = "abc".to_string();
        let queries = vec![
            "query 0 2".to_string(),
            "query 0 1".to_string(),
            "query 1 1".to_string(),
        ];
        assert_eq!(
            Solution::palindrome_path(n, edges, s, queries),
            vec![false, false, true]
        );
    }

    #[test]
    fn test_palindromic_path() {
        // Tree: 0-1-2, chars "aba"
        // Path 0->2: "aba" -> palindromic (already a palindrome)
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let s = "aba".to_string();
        let queries = vec!["query 0 2".to_string()];
        assert_eq!(Solution::palindrome_path(n, edges, s, queries), vec![true]);
    }

    #[test]
    fn test_update_then_query() {
        // Tree: 0-1-2, chars "abc"
        // Update node 2 to 'a' -> chars "aba"
        // Path 0->2: "aba" -> palindromic
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let s = "abc".to_string();
        let queries = vec![
            "query 0 2".to_string(),
            "update 2 a".to_string(),
            "query 0 2".to_string(),
        ];
        assert_eq!(
            Solution::palindrome_path(n, edges, s, queries),
            vec![false, true]
        );
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges: Vec<Vec<i32>> = Vec::new();
        let s = "a".to_string();
        let queries = vec!["query 0 0".to_string()];
        assert_eq!(Solution::palindrome_path(n, edges, s, queries), vec![true]);
    }

    #[test]
    fn test_star_tree() {
        // Star tree: 0 is center, connected to 1,2,3,4
        // chars: "aabba"
        //          0  1  2  3  4
        //          a  a  b  b  a
        // Path 1->2 goes through 0: chars "a,a,b" = "aab" -> can rearrange to "aba" -> true
        // Path 1->3: "a,a,b" -> same as above -> true
        // Path 1->4: "a,a,a" -> palindromic -> true
        // Path 2->3: "b,a,b" -> "bab" -> true
        // Path 2->4: "b,a,a" -> "aba" -> true
        // Path 3->4: "b,a,a" -> "aba" -> true
        let n = 5;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let s = "aabba".to_string();
        let queries = vec![
            "query 1 2".to_string(),
            "query 1 3".to_string(),
            "query 1 4".to_string(),
            "query 2 3".to_string(),
            "query 2 4".to_string(),
            "query 3 4".to_string(),
        ];
        assert_eq!(
            Solution::palindrome_path(n, edges, s, queries),
            vec![true, true, true, true, true, true]
        );
    }

    #[test]
    fn test_even_length_palindromic() {
        // Path with even length: "aabb" -> can rearrange to "abba" -> true
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let s = "aabb".to_string();
        let queries = vec!["query 0 3".to_string()];
        assert_eq!(Solution::palindrome_path(n, edges, s, queries), vec![true]);
    }

    #[test]
    fn test_multiple_updates() {
        // Tree: 0-1-2
        // Initial chars: "abc"
        // Path 0->2: "abc" -> false
        // Update 0 to 'c': "cbc"
        // Path 0->2: "cbc" -> palindromic -> true
        // Update 1 to 'a': "cac"
        // Path 0->2: "cac" -> palindromic -> true
        // Update 0 to 'b': "bac"
        // Path 0->2: "bac" -> not palindromic (all different) -> false
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let s = "abc".to_string();
        let queries = vec![
            "query 0 2".to_string(),
            "update 0 c".to_string(),
            "query 0 2".to_string(),
            "update 1 a".to_string(),
            "query 0 2".to_string(),
            "update 0 b".to_string(),
            "query 0 2".to_string(),
        ];
        assert_eq!(
            Solution::palindrome_path(n, edges, s, queries),
            vec![false, true, true, false]
        );
    }
}
