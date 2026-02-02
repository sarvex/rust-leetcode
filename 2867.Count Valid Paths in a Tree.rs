impl Solution {
    /// Counts paths that contain exactly one prime label.
    ///
    /// # Intuition
    /// Every valid path has a unique prime node on it, so we can count paths by attributing them
    /// to that prime.
    ///
    /// # Approach
    /// - Sieve primes up to `n`.
    /// - Build the adjacency list for the tree.
    /// - Compute connected components of non-prime nodes only and store the component size for
    ///   each non-prime node.
    /// - For each prime node `p`, collect sizes of adjacent non-prime components. Paths where `p`
    ///   is the only prime are:
    ///   - `sum sizes` for paths with endpoint `p`.
    ///   - `sum_{i < j} sizes[i] * sizes[j]` for paths between components through `p`.
    /// - Sum contributions across all prime nodes.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let is_prime = Self::sieve_primes(n);
        let mut graph = vec![Vec::new(); n + 1];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let component_size = Self::non_prime_component_sizes(n, &graph, &is_prime);
        let mut answer = 0_i64;

        for node in 1..=n {
            if !is_prime[node] {
                continue;
            }
            let mut total = 0_i64;
            let mut pair_count = 0_i64;
            let mut accumulated = 0_i64;

            for &neighbor in &graph[node] {
                if is_prime[neighbor] {
                    continue;
                }
                let size = component_size[neighbor] as i64;
                total += size;
                pair_count += accumulated * size;
                accumulated += size;
            }

            answer += total + pair_count;
        }

        answer
    }

    fn sieve_primes(limit: usize) -> Vec<bool> {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        if limit >= 1 {
            is_prime[1] = false;
        }

        let mut p = 2_usize;
        while p * p <= limit {
            if is_prime[p] {
                let mut multiple = p * p;
                while multiple <= limit {
                    is_prime[multiple] = false;
                    multiple += p;
                }
            }
            p += 1;
        }

        is_prime
    }

    fn non_prime_component_sizes(n: usize, graph: &[Vec<usize>], is_prime: &[bool]) -> Vec<usize> {
        let mut component_size = vec![0_usize; n + 1];
        let mut visited = vec![false; n + 1];

        for node in 1..=n {
            if is_prime[node] || visited[node] {
                continue;
            }
            let mut stack = vec![node];
            visited[node] = true;
            let mut members = Vec::new();

            while let Some(current) = stack.pop() {
                members.push(current);
                for &neighbor in &graph[current] {
                    if !is_prime[neighbor] && !visited[neighbor] {
                        visited[neighbor] = true;
                        stack.push(neighbor);
                    }
                }
            }

            let size = members.len();
            for node in members {
                component_size[node] = size;
            }
        }

        component_size
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let n = 5;
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]];
        assert_eq!(Solution::count_paths(n, edges), 4);
    }

    #[test]
    fn test_example_2() {
        let n = 6;
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]];
        assert_eq!(Solution::count_paths(n, edges), 6);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges = vec![];
        assert_eq!(Solution::count_paths(n, edges), 0);
    }

    #[test]
    fn test_two_nodes() {
        let n = 2;
        let edges = vec![vec![1, 2]];
        assert_eq!(Solution::count_paths(n, edges), 1);
    }
}
