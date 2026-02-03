impl Solution {
    /// Tree DP with usage counts per node.
    ///
    /// # Intuition
    /// Each trip contributes the prices along its unique path. Counting how many times each node
    /// appears across all trips reduces the problem to choosing a set of non-adjacent nodes to
    /// halve with minimum total weighted cost.
    ///
    /// # Approach
    /// 1. Build the tree adjacency list.
    /// 2. For every trip, DFS to collect the unique path and increment a usage counter for each
    ///    node on that path.
    /// 3. Run a tree DP with two states per node: not halved vs halved. If a node is halved, its
    ///    children cannot be halved. Combine children optimally to minimize cost.
    ///
    /// # Complexity
    /// - Time: O(n * trips)
    /// - Space: O(n)
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let node_count = n as usize;
        let mut adjacency = vec![Vec::<usize>::new(); node_count];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            adjacency[a].push(b);
            adjacency[b].push(a);
        }

        let mut usage_counts = vec![0i64; node_count];
        for trip in trips {
            let start = trip[0] as usize;
            let end = trip[1] as usize;
            let mut path = Vec::new();
            Self::collect_path(start, None, end, &adjacency, &mut path);
            for node in path {
                usage_counts[node] += 1;
            }
        }

        let (cost_full, cost_half) = Self::min_costs(0, None, &adjacency, &usage_counts, &price);
        cost_full.min(cost_half) as i32
    }

    fn collect_path(
        node: usize,
        parent: Option<usize>,
        target: usize,
        adjacency: &[Vec<usize>],
        path: &mut Vec<usize>,
    ) -> bool {
        if node == target {
            path.push(node);
            return true;
        }

        for &next in &adjacency[node] {
            if Some(next) == parent {
                continue;
            }
            if Self::collect_path(next, Some(node), target, adjacency, path) {
                path.push(node);
                return true;
            }
        }
        false
    }

    fn min_costs(
        node: usize,
        parent: Option<usize>,
        adjacency: &[Vec<usize>],
        usage_counts: &[i64],
        price: &[i32],
    ) -> (i64, i64) {
        let full_cost = usage_counts[node] * price[node] as i64;
        let half_cost = usage_counts[node] * (price[node] as i64 / 2);

        let mut cost_not_halved = full_cost;
        let mut cost_halved = half_cost;

        for &next in &adjacency[node] {
            if Some(next) == parent {
                continue;
            }
            let (child_not_halved, child_halved) =
                Self::min_costs(next, Some(node), adjacency, usage_counts, price);
            cost_not_halved += child_not_halved.min(child_halved);
            cost_halved += child_not_halved;
        }

        (cost_not_halved, cost_halved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3]];
        let price = vec![2, 2, 10, 6];
        let trips = vec![vec![0, 3], vec![2, 1], vec![2, 3]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 23);
    }

    #[test]
    fn example_two() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let price = vec![2, 2];
        let trips = vec![vec![0, 0]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 1);
    }

    #[test]
    fn single_node_multiple_trips() {
        let n = 1;
        let edges = vec![];
        let price = vec![10];
        let trips = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 10);
    }

    #[test]
    fn two_nodes_one_trip() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let price = vec![6, 4];
        let trips = vec![vec![0, 1]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 7);
    }

    #[test]
    fn prefers_halving_middle_node() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let price = vec![2, 10, 2];
        let trips = vec![vec![0, 2]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 9);
    }
}
