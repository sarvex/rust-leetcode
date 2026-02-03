use std::collections::VecDeque;

impl Solution {
    /// Trim coinless leaves, then trim two more layers to account for distance-2 collection.
    ///
    /// # Intuition
    /// Leaves without coins are never worth visiting because they only add traversal cost. After
    /// removing them, coins within distance 2 of any visited node do not require traversing the
    /// last two edges on their paths.
    ///
    /// # Approach
    /// 1. Build an adjacency list and degrees for the tree.
    /// 2. Repeatedly remove leaves that have no coins.
    /// 3. Remove two additional layers of leaves regardless of coin presence.
    /// 4. The remaining subgraph is a tree; traversing every remaining edge twice gives the
    ///    minimum round trip length.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let node_count = coins.len();
        if node_count <= 1 {
            return 0;
        }

        let mut adjacency = vec![Vec::<usize>::new(); node_count];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency[u].push(v);
            adjacency[v].push(u);
        }

        let mut degree = adjacency
            .iter()
            .map(|neighbors| neighbors.len() as i32)
            .collect::<Vec<_>>();
        let mut active = vec![true; node_count];
        let mut queue = VecDeque::new();

        for node in 0..node_count {
            if degree[node] <= 1 && coins[node] == 0 {
                queue.push_back(node);
            }
        }

        while let Some(node) = queue.pop_front() {
            if !active[node] {
                continue;
            }
            active[node] = false;
            for &next in &adjacency[node] {
                if active[next] {
                    degree[next] -= 1;
                    if degree[next] == 1 && coins[next] == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }

        for node in 0..node_count {
            if active[node] && degree[node] <= 1 {
                queue.push_back(node);
            }
        }

        for _ in 0..2 {
            let layer_size = queue.len();
            if layer_size == 0 {
                break;
            }
            for _ in 0..layer_size {
                let node = queue.pop_front().unwrap();
                if !active[node] {
                    continue;
                }
                active[node] = false;
                for &next in &adjacency[node] {
                    if active[next] {
                        degree[next] -= 1;
                        if degree[next] == 1 {
                            queue.push_back(next);
                        }
                    }
                }
            }
        }

        let mut remaining_edges = 0i32;
        for node in 0..node_count {
            if active[node] {
                remaining_edges += degree[node];
            }
        }
        remaining_edges /= 2;

        if remaining_edges == 0 {
            0
        } else {
            remaining_edges * 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let coins = vec![1, 0, 0, 0, 0, 1];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::collect_the_coins(coins, edges), 2);
    }

    #[test]
    fn example_two() {
        let coins = vec![0, 0, 0, 1, 1, 0, 0, 1];
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![5, 6],
            vec![5, 7],
        ];
        assert_eq!(Solution::collect_the_coins(coins, edges), 2);
    }

    #[test]
    fn single_node_with_coin() {
        let coins = vec![1];
        let edges: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::collect_the_coins(coins, edges), 0);
    }

    #[test]
    fn all_zero_chain() {
        let coins = vec![0, 0, 0, 0];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::collect_the_coins(coins, edges), 0);
    }

    #[test]
    fn two_nodes_both_coins() {
        let coins = vec![1, 1];
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::collect_the_coins(coins, edges), 0);
    }

    #[test]
    fn long_chain_two_coins() {
        let coins = vec![1, 0, 0, 0, 0, 0, 1];
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
        ];
        assert_eq!(Solution::collect_the_coins(coins, edges), 4);
    }
}
