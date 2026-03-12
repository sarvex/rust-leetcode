impl Solution {
    /// Binary search on stability with optimized spanning tree construction.
    ///
    /// # Intuition
    /// The answer equals some edge strength (original or doubled). Binary
    /// search on target stability and check feasibility via reusable
    /// union-find with pre-computed mandatory state and sorted optionals.
    ///
    /// # Approach
    /// 1. Classify edges, pre-build mandatory union-find state, detect cycles
    /// 2. Sort optional edges descending; use partition-point lookups
    /// 3. Binary search with buffer-reusing union-find and component count
    ///    tracking for O(1) connectivity detection and early termination
    ///
    /// # Complexity
    /// - Time: O(E log E × α(N)) for binary search with union-find checks
    /// - Space: O(N + E) for union-find buffers and edge storage
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let m = edges.len();
        let mut mandatory = Vec::with_capacity(m);
        let mut optional = Vec::with_capacity(m);
        let mut mandatory_min = i32::MAX;

        for e in &edges {
            let (u, v, s) = (e[0] as usize, e[1] as usize, e[2]);
            if e[3] == 1 {
                mandatory.push((u, v, s));
                mandatory_min = mandatory_min.min(s);
            } else {
                optional.push((u, v, s));
            }
        }

        // Pre-build mandatory UF state and detect cycles
        let mut parent_base: Vec<usize> = (0..n).collect();
        let mut rank_base = vec![0u32; n];
        let mut base_components = n;

        for &(u, v, _) in &mandatory {
            let pu = find(&mut parent_base, u);
            let pv = find(&mut parent_base, v);
            if pu == pv {
                return -1;
            }
            union_nodes(&mut parent_base, &mut rank_base, pu, pv);
            base_components -= 1;
        }

        // Sort optional edges descending for partition-based processing
        optional.sort_unstable_by(|a, b| b.2.cmp(&a.2));

        // Generate candidates, skip doubled mandatory (can't upgrade mandatory)
        let mut candidates = Vec::with_capacity(2 * m);
        for e in &edges {
            candidates.push(e[2]);
            if e[3] == 0 {
                candidates.push(e[2] * 2);
            }
        }
        candidates.sort_unstable();
        candidates.dedup();

        // Prune candidates above mandatory_min (always infeasible)
        let upper = candidates.partition_point(|&c| c <= mandatory_min);
        candidates.truncate(upper);

        if candidates.is_empty() {
            return -1;
        }

        // Reusable UF buffers — copy_from_slice instead of allocating each step
        let mut parent = vec![0usize; n];
        let mut rank = vec![0u32; n];
        let mut result = -1i32;
        let (mut lo, mut hi) = (0usize, candidates.len() - 1);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let target = candidates[mid];

            parent.copy_from_slice(&parent_base);
            rank.copy_from_slice(&rank_base);
            let mut comp = base_components;

            // Strong optional edges (s >= target, free)
            let strong_end = optional.partition_point(|&(_, _, s)| s >= target);
            for &(u, v, _) in &optional[..strong_end] {
                let (pu, pv) = (find(&mut parent, u), find(&mut parent, v));
                if pu != pv {
                    union_nodes(&mut parent, &mut rank, pu, pv);
                    comp -= 1;
                    if comp == 1 {
                        break;
                    }
                }
            }

            // Weak optional edges (s < target, 2s >= target, cost 1 each)
            let mut upgrades = 0i32;
            if comp > 1 {
                let half = (target + 1) / 2;
                let weak_end = optional.partition_point(|&(_, _, s)| s >= half);
                for &(u, v, _) in &optional[strong_end..weak_end] {
                    let (pu, pv) = (find(&mut parent, u), find(&mut parent, v));
                    if pu != pv {
                        union_nodes(&mut parent, &mut rank, pu, pv);
                        comp -= 1;
                        upgrades += 1;
                        if upgrades > k || comp == 1 {
                            break;
                        }
                    }
                }
            }

            if comp == 1 && upgrades <= k {
                result = target;
                lo = mid + 1;
            } else if mid == 0 {
                break;
            } else {
                hi = mid - 1;
            }
        }

        result
    }
}

#[inline]
fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

#[inline]
fn union_nodes(parent: &mut [usize], rank: &mut [u32], x: usize, y: usize) {
    if rank[x] < rank[y] {
        parent[x] = y;
    } else {
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
        parent[y] = x;
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
