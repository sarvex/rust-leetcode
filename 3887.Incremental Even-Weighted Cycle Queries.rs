impl Solution {
    /// Counts edges that maintain even-weight cycles using weighted Union-Find.
    ///
    /// # Intuition
    /// A graph has all even-weight cycles iff each node can be assigned a parity
    /// label so that `label[u] XOR label[v] = w` for every edge `(u, v, w)`.
    ///
    /// # Approach
    /// Weighted Union-Find with iterative path-halving. Each entry packs the
    /// parent index and XOR-distance bit into a single `u32` (`parent << 1 | dist`).
    /// For each edge, merge different components or accept same-component edges
    /// whose accumulated parity is consistent.
    ///
    /// # Complexity
    /// - Time: O(n + m · α(n))
    /// - Space: O(n)
    pub fn number_of_edges_added(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut data: Vec<u32> = (0..n as u32).map(|i| i << 1).collect();
        let mut rank = vec![0u8; n];
        let mut count = 0;

        for edge in &edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2] as u8);
            let (ru, du) = Self::find(&mut data, u);
            let (rv, dv) = Self::find(&mut data, v);

            if ru != rv {
                if rank[ru] < rank[rv] {
                    data[ru] = (rv as u32) << 1 | (du ^ w ^ dv) as u32;
                } else {
                    data[rv] = (ru as u32) << 1 | (dv ^ w ^ du) as u32;
                    if rank[ru] == rank[rv] {
                        rank[ru] += 1;
                    }
                }
                count += 1;
            } else if du ^ dv == w {
                count += 1;
            }
        }
        count
    }

    fn find(data: &mut [u32], mut x: usize) -> (usize, u8) {
        let mut parity = 0u8;
        while (data[x] >> 1) as usize != x {
            let px = (data[x] >> 1) as usize;
            let gpx = (data[px] >> 1) as usize;
            let new_dist = (data[x] ^ data[px]) & 1;
            data[x] = (gpx as u32) << 1 | new_dist;
            parity ^= new_dist as u8;
            x = gpx;
        }
        (x, parity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_cycle_rejected() {
        assert_eq!(
            Solution::number_of_edges_added(3, vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 1]]),
            2
        );
    }

    #[test]
    fn test_even_cycle_accepted() {
        assert_eq!(
            Solution::number_of_edges_added(3, vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 0]]),
            3
        );
    }

    #[test]
    fn test_tree_all_accepted() {
        assert_eq!(
            Solution::number_of_edges_added(4, vec![vec![0, 1, 1], vec![1, 2, 0], vec![2, 3, 1]]),
            3
        );
    }

    #[test]
    fn test_all_zero_weights() {
        assert_eq!(
            Solution::number_of_edges_added(3, vec![vec![0, 1, 0], vec![1, 2, 0], vec![0, 2, 0]]),
            3
        );
    }

    #[test]
    fn test_single_edge() {
        assert_eq!(Solution::number_of_edges_added(3, vec![vec![0, 1, 1]]), 1);
    }

    #[test]
    fn test_four_cycle_even() {
        assert_eq!(
            Solution::number_of_edges_added(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]]
            ),
            4
        );
    }

    #[test]
    fn test_four_cycle_odd() {
        assert_eq!(
            Solution::number_of_edges_added(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 0]]
            ),
            3
        );
    }

    #[test]
    fn test_long_chain_even_closing() {
        assert_eq!(
            Solution::number_of_edges_added(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 1],
                    vec![3, 4, 1],
                    vec![0, 4, 0]
                ]
            ),
            5
        );
    }

    #[test]
    fn test_long_chain_odd_closing() {
        assert_eq!(
            Solution::number_of_edges_added(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 1],
                    vec![3, 4, 1],
                    vec![0, 4, 1]
                ]
            ),
            4
        );
    }

    #[test]
    fn test_multiple_components() {
        assert_eq!(
            Solution::number_of_edges_added(
                6,
                vec![
                    vec![0, 1, 0],
                    vec![1, 2, 0],
                    vec![0, 2, 0],
                    vec![3, 4, 1],
                    vec![4, 5, 1],
                    vec![3, 5, 0]
                ]
            ),
            6
        );
    }

    #[test]
    fn test_dense_all_weight_one_rejections() {
        assert_eq!(
            Solution::number_of_edges_added(
                4,
                vec![
                    vec![0, 1, 1],
                    vec![0, 2, 1],
                    vec![0, 3, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 1],
                    vec![2, 3, 1]
                ]
            ),
            3
        );
    }

    #[test]
    fn test_chain_with_consistent_cross_edges() {
        assert_eq!(
            Solution::number_of_edges_added(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 1],
                    vec![3, 4, 1],
                    vec![0, 2, 0],
                    vec![1, 3, 0],
                    vec![0, 3, 1]
                ]
            ),
            7
        );
    }
}
