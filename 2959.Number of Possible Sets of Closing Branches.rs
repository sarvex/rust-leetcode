impl Solution {
    /// Counts closing sets by precomputing subset shortest paths.
    ///
    /// # Intuition
    /// With at most 10 branches, every subset of remaining branches can be tested. A remaining
    /// subset is valid when all pairwise shortest paths within that induced subgraph are at most
    /// `max_distance`.
    ///
    /// # Approach
    /// Build a direct-edge adjacency matrix with the minimum weight for parallel roads. Then
    /// precompute shortest paths for all masks by adding one intermediate node at a time, mirroring
    /// Floyd-Warshall but caching every subset state. For each remaining-branch mask, if every
    /// pairwise distance is at most `max_distance`, count it. The number of valid remaining subsets
    /// equals the number of valid closing sets.
    ///
    /// # Complexity
    /// - Time: O(2^n * n^2)
    /// - Space: O(2^n * n^2)
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let max_distance = max_distance as i64;
        let inf = i64::MAX / 4;

        let mut base = vec![vec![inf; n]; n];
        for i in 0..n {
            base[i][i] = 0;
        }
        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let w = road[2] as i64;
            if w < base[u][v] {
                base[u][v] = w;
                base[v][u] = w;
            }
        }

        let total_masks = 1_usize << n;
        let mut dist_masks = vec![vec![inf; n * n]; total_masks];
        for i in 0..n {
            for j in 0..n {
                dist_masks[0][i * n + j] = base[i][j];
            }
        }

        for mask in 1..total_masks {
            let k = mask.trailing_zeros() as usize;
            let prev = mask ^ (1_usize << k);
            let prev_dist = &dist_masks[prev];
            let mut current = prev_dist.clone();
            for i in 0..n {
                let ik = prev_dist[i * n + k];
                if ik == inf {
                    continue;
                }
                for j in 0..n {
                    let kj = prev_dist[k * n + j];
                    if kj == inf {
                        continue;
                    }
                    let candidate = ik + kj;
                    let idx = i * n + j;
                    if candidate < current[idx] {
                        current[idx] = candidate;
                    }
                }
            }
            dist_masks[mask] = current;
        }

        let mut count = 0_i32;
        for mask in 0..total_masks {
            if mask == 0 || mask & (mask - 1) == 0 {
                count += 1;
                continue;
            }

            let dist = &dist_masks[mask];
            let mut ok = true;
            'check: for i in 0..n {
                if (mask >> i) & 1 == 0 {
                    continue;
                }
                for j in (i + 1)..n {
                    if (mask >> j) & 1 == 0 {
                        continue;
                    }
                    if dist[i * n + j] > max_distance {
                        ok = false;
                        break 'check;
                    }
                }
            }

            if ok {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let n = 3;
        let max_distance = 5;
        let roads = vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]];
        assert_eq!(Solution::number_of_sets(n, max_distance, roads), 5);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let max_distance = 5;
        let roads = vec![vec![0, 1, 20], vec![0, 1, 10], vec![1, 2, 2], vec![0, 2, 2]];
        assert_eq!(Solution::number_of_sets(n, max_distance, roads), 7);
    }

    #[test]
    fn test_example_3() {
        let n = 1;
        let max_distance = 10;
        let roads = vec![];
        assert_eq!(Solution::number_of_sets(n, max_distance, roads), 2);
    }

    #[test]
    fn test_single_edge_too_long() {
        let n = 2;
        let max_distance = 4;
        let roads = vec![vec![0, 1, 5]];
        assert_eq!(Solution::number_of_sets(n, max_distance, roads), 3);
    }
}
