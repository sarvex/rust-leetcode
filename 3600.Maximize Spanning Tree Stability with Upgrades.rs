impl Solution {
    /// Binary search on stability with greedy spanning tree construction.
    ///
    /// # Intuition
    /// The answer must be an edge strength (original or doubled). Binary search
    /// on target stability, checking if a valid spanning tree exists using
    /// greedy Kruskal-style construction prioritizing non-upgraded edges.
    ///
    /// # Approach
    /// 1. Separate mandatory/optional edges, check mandatory for cycles
    /// 2. Binary search on candidate target values (all strengths and doubled)
    /// 3. For each target: use strong edges first (free), then weak edges (cost 1)
    /// 4. Check if all nodes connected within upgrade budget
    ///
    /// # Complexity
    /// - Time: O(E log E × α(N)) for binary search with union-find checks
    /// - Space: O(N + E) for union-find and edge storage
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;

        let mut mandatory = Vec::new();
        let mut optional = Vec::new();

        for e in &edges {
            let (u, v, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if must == 1 {
                mandatory.push((u, v, s));
            } else {
                optional.push((u, v, s));
            }
        }

        // Check mandatory edges for cycles
        let mut uf_check = UnionFind::new(n);
        let mut mandatory_min = i32::MAX;

        for &(u, v, s) in &mandatory {
            if uf_check.find(u) == uf_check.find(v) {
                return -1; // Cycle in mandatory edges
            }
            uf_check.union(u, v);
            mandatory_min = mandatory_min.min(s);
        }

        // Generate candidate target values
        let mut candidates: Vec<i32> = edges
            .iter()
            .flat_map(|e| [e[2], e[2] * 2])
            .collect();
        candidates.sort_unstable();
        candidates.dedup();

        if candidates.is_empty() {
            return -1;
        }

        let can_achieve = |target: i32| -> bool {
            // Mandatory edges must all meet target
            if !mandatory.is_empty() && mandatory_min < target {
                return false;
            }

            let mut uf = UnionFind::new(n);

            // Add mandatory edges
            for &(u, v, _) in &mandatory {
                uf.union(u, v);
            }

            // Add strong optional edges (no upgrade needed)
            for &(u, v, s) in &optional {
                if s >= target && uf.find(u) != uf.find(v) {
                    uf.union(u, v);
                }
            }

            // Add weak optional edges (need upgrade)
            let mut upgrades = 0;
            for &(u, v, s) in &optional {
                if s < target && 2 * s >= target && uf.find(u) != uf.find(v) {
                    uf.union(u, v);
                    upgrades += 1;
                }
            }

            // Check connectivity and budget
            let root = uf.find(0);
            (0..n).all(|i| uf.find(i) == root) && upgrades <= k
        };

        // Binary search for maximum achievable target
        let (mut lo, mut hi) = (0i32, candidates.len() as i32 - 1);
        let mut result = -1;

        while lo <= hi {
            let mid = (lo + hi) / 2;
            if can_achieve(candidates[mid as usize]) {
                result = candidates[mid as usize];
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        result
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else {
            self.parent[py] = px;
            self.rank[px] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1, 2, 1], vec![1, 2, 3, 0]];
        assert_eq!(Solution::max_stability(3, edges, 1), 2);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1, 4, 0], vec![1, 2, 3, 0], vec![0, 2, 1, 0]];
        assert_eq!(Solution::max_stability(3, edges, 2), 6);
    }

    #[test]
    fn test_example_3() {
        let edges = vec![vec![0, 1, 1, 1], vec![1, 2, 1, 1], vec![2, 0, 1, 1]];
        assert_eq!(Solution::max_stability(3, edges, 0), -1);
    }

    #[test]
    fn test_all_mandatory_valid() {
        let edges = vec![vec![0, 1, 5, 1], vec![1, 2, 3, 1]];
        assert_eq!(Solution::max_stability(3, edges, 0), 3);
    }

    #[test]
    fn test_disconnected() {
        let edges = vec![vec![0, 1, 5, 0]];
        assert_eq!(Solution::max_stability(3, edges, 1), -1);
    }

    #[test]
    fn test_no_upgrades_needed() {
        let edges = vec![vec![0, 1, 10, 0], vec![1, 2, 10, 0]];
        assert_eq!(Solution::max_stability(3, edges, 0), 10);
    }

    #[test]
    fn test_upgrade_helps() {
        let edges = vec![vec![0, 1, 5, 0], vec![1, 2, 3, 0]];
        // Without upgrade: min = 3
        // With upgrade on edge 1: strengths are 5 and 6, min = 5
        assert_eq!(Solution::max_stability(3, edges, 1), 5);
    }
}
