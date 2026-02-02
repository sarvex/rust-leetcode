#[derive(Clone, Copy)]
struct Extremes {
    size: usize,
    top: [i32; 3],
    top_len: usize,
    bottom: [i32; 2],
    bottom_len: usize,
}

impl Extremes {
    fn empty() -> Self {
        Self {
            size: 0,
            top: [0; 3],
            top_len: 0,
            bottom: [0; 2],
            bottom_len: 0,
        }
    }

    fn from_value(value: i32) -> Self {
        Self {
            size: 1,
            top: [value, 0, 0],
            top_len: 1,
            bottom: [value, 0],
            bottom_len: 1,
        }
    }

    fn merge_from(&mut self, child: Self) {
        self.size += child.size;
        for &value in child.top[..child.top_len].iter() {
            self.insert_top(value);
        }
        for &value in child.bottom[..child.bottom_len].iter() {
            self.insert_bottom(value);
        }
    }

    fn insert_top(&mut self, value: i32) {
        let len = self.top_len;
        let mut idx = 0;
        while idx < len && self.top[idx] >= value {
            idx += 1;
        }
        if len < 3 {
            for j in (idx..len).rev() {
                self.top[j + 1] = self.top[j];
            }
            self.top[idx] = value;
            self.top_len += 1;
        } else if idx < 3 {
            for j in (idx..2).rev() {
                self.top[j + 1] = self.top[j];
            }
            self.top[idx] = value;
        }
    }

    fn insert_bottom(&mut self, value: i32) {
        let len = self.bottom_len;
        let mut idx = 0;
        while idx < len && self.bottom[idx] <= value {
            idx += 1;
        }
        if len < 2 {
            for j in (idx..len).rev() {
                self.bottom[j + 1] = self.bottom[j];
            }
            self.bottom[idx] = value;
            self.bottom_len += 1;
        } else if idx < 2 {
            for j in (idx..1).rev() {
                self.bottom[j + 1] = self.bottom[j];
            }
            self.bottom[idx] = value;
        }
    }
}

impl Solution {
    /// Post-order extremes to maximize triple products.
    ///
    /// # Intuition
    /// The maximum product of any three values in a set depends only on the
    /// three largest values and the two smallest (most negative) values.
    ///
    /// # Approach
    /// Root the tree at node `0`, build a parent array with an iterative DFS,
    /// and process nodes in reverse order (post-order). For each node keep:
    /// - subtree size
    /// - top three largest costs
    /// - bottom two smallest costs
    /// Merge children by inserting their extremes into fixed-size arrays.
    /// If the subtree has fewer than three nodes, place `1` coin. Otherwise,
    /// compute the best product from (largest * second * third) and
    /// (largest * two smallest). If the maximum product is negative, place `0`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
        let n = cost.len();
        let mut graph = vec![Vec::new(); n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }

        let mut parent = vec![usize::MAX; n];
        let mut order = Vec::with_capacity(n);
        let mut stack = Vec::with_capacity(n);
        parent[0] = 0;
        stack.push(0);

        while let Some(node) = stack.pop() {
            order.push(node);
            for &next in &graph[node] {
                if next == parent[node] {
                    continue;
                }
                parent[next] = node;
                stack.push(next);
            }
        }

        let mut extremes = vec![Extremes::empty(); n];
        let mut coins = vec![0i64; n];

        for &node in order.iter().rev() {
            extremes[node] = Extremes::from_value(cost[node]);
            for &next in &graph[node] {
                if next == parent[node] {
                    continue;
                }
                let child = extremes[next];
                extremes[node].merge_from(child);
            }

            let node_extremes = extremes[node];
            coins[node] = if node_extremes.size < 3 {
                1
            } else {
                let top_product = (node_extremes.top[0] as i64)
                    * (node_extremes.top[1] as i64)
                    * (node_extremes.top[2] as i64);
                let mixed_product = (node_extremes.top[0] as i64)
                    * (node_extremes.bottom[0] as i64)
                    * (node_extremes.bottom[1] as i64);
                let best = top_product.max(mixed_product);
                if best < 0 {
                    0
                } else {
                    best
                }
            };
        }

        coins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]];
        let cost = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            Solution::placed_coins(edges, cost),
            vec![120, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_example_2() {
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![2, 6],
            vec![2, 7],
            vec![2, 8],
        ];
        let cost = vec![1, 4, 2, 3, 5, 7, 8, -4, 2];
        assert_eq!(
            Solution::placed_coins(edges, cost),
            vec![280, 140, 32, 1, 1, 1, 1, 1, 1],
        );
    }

    #[test]
    fn test_example_3() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let cost = vec![1, 2, -2];
        assert_eq!(Solution::placed_coins(edges, cost), vec![0, 1, 1]);
    }

    #[test]
    fn test_negative_pair_with_positive() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let cost = vec![5, -10, -10];
        assert_eq!(Solution::placed_coins(edges, cost), vec![500, 1, 1]);
    }

    #[test]
    fn test_all_equal_values() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let cost = vec![2, 2, 2, 2];
        assert_eq!(Solution::placed_coins(edges, cost), vec![8, 1, 1, 1]);
    }
}
