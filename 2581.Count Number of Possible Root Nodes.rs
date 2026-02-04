use std::collections::HashSet;

impl Solution {
    /// Reroot the tree while tracking correct guesses.
    ///
    /// # Intuition
    /// Fixing a root defines a parent direction for every edge. A guess is correct if it matches
    /// that direction. When rerooting across a single edge, only the two directed guesses on that
    /// edge can change correctness.
    ///
    /// # Approach
    /// 1. Build the adjacency list and store all guesses in a hash set.
    /// 2. Root the tree at 0, record parents, and count correct guesses for that root.
    /// 3. Reroot with a DFS: moving root from `u` to `v` decrements the count if `(u, v)` was a
    ///    guess and increments it if `(v, u)` was a guess.
    /// 4. Count roots with at least `k` correct guesses.
    ///
    /// # Complexity
    /// - Time: O(n + g)
    /// - Space: O(n + g)
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = edges.len() + 1;
        let mut graph = vec![Vec::new(); n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }

        let encode = |u: usize, v: usize| -> u64 { (u as u64) << 32 | v as u64 };
        let guess_set: HashSet<u64> = guesses
            .into_iter()
            .map(|guess| encode(guess[0] as usize, guess[1] as usize))
            .collect();

        let mut parent = vec![usize::MAX; n];
        parent[0] = 0;
        let mut stack = vec![0usize];

        while let Some(node) = stack.pop() {
            for &next in &graph[node] {
                if next == parent[node] {
                    continue;
                }
                parent[next] = node;
                stack.push(next);
            }
        }

        let mut initial_correct = 0i32;
        for node in 1..n {
            let p = parent[node];
            if guess_set.contains(&encode(p, node)) {
                initial_correct += 1;
            }
        }

        let mut result = 0i32;
        let mut correct_counts = vec![0i32; n];
        correct_counts[0] = initial_correct;

        let mut reroot_stack = vec![0usize];
        while let Some(node) = reroot_stack.pop() {
            let current = correct_counts[node];
            if current >= k {
                result += 1;
            }
            for &next in &graph[node] {
                if next == parent[node] {
                    continue;
                }
                let mut next_count = current;
                if guess_set.contains(&encode(node, next)) {
                    next_count -= 1;
                }
                if guess_set.contains(&encode(next, node)) {
                    next_count += 1;
                }
                correct_counts[next] = next_count;
                reroot_stack.push(next);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]];
        let guesses = vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]];
        assert_eq!(Solution::root_count(edges, guesses, 3), 3);
    }

    #[test]
    fn example_two() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let guesses = vec![vec![1, 0], vec![3, 4], vec![2, 1], vec![3, 2]];
        assert_eq!(Solution::root_count(edges, guesses, 1), 5);
    }

    #[test]
    fn k_zero_counts_all_roots() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3]];
        let guesses = vec![vec![0, 1], vec![2, 3]];
        assert_eq!(Solution::root_count(edges, guesses, 0), 4);
    }

    #[test]
    fn single_edge_threshold() {
        let edges = vec![vec![0, 1]];
        let guesses = vec![vec![0, 1]];
        assert_eq!(Solution::root_count(edges, guesses, 1), 1);
    }
}
