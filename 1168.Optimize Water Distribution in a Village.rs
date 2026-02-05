struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        parent.extend(0..n);
        Self {
            parent,
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (pa, pb) = (self.find(a), self.find(b));
        if pa == pb {
            return false;
        }
        if self.size[pa] >= self.size[pb] {
            self.parent[pb] = pa;
            self.size[pa] += self.size[pb];
        } else {
            self.parent[pa] = pb;
            self.size[pb] += self.size[pa];
        }
        true
    }
}

impl Solution {
    /// Finds minimum cost to supply water using Kruskal's MST with virtual node.
    ///
    /// # Intuition
    /// Model well costs as edges from a virtual node 0 to each house. Then
    /// finding the MST gives the minimum cost to connect all houses to water.
    ///
    /// # Approach
    /// Add virtual edges for wells. Combine with pipe edges. Sort by cost.
    /// Use Union-Find to build MST greedily.
    ///
    /// # Complexity
    /// - Time: O(E log E) where E = pipes + wells
    /// - Space: O(n) for Union-Find
    pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, mut pipes: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        for (i, &cost) in wells.iter().enumerate() {
            pipes.push(vec![0, (i + 1) as i32, cost]);
        }
        pipes.sort_unstable_by_key(|p| p[2]);

        let mut uf = UnionFind::new(n + 1);
        let mut total = 0;
        for pipe in &pipes {
            if uf.union(pipe[0] as usize, pipe[1] as usize) {
                total += pipe[2];
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 3 houses, wells cost [1,2,2], pipes connect them
        let wells = vec![1, 2, 2];
        let pipes = vec![vec![1, 2, 1], vec![2, 3, 1]];
        assert_eq!(Solution::min_cost_to_supply_water(3, wells, pipes), 3);
    }

    #[test]
    fn test_wells_only() {
        // Cheaper to dig wells than lay pipes
        let wells = vec![1, 1];
        let pipes = vec![vec![1, 2, 100]];
        assert_eq!(Solution::min_cost_to_supply_water(2, wells, pipes), 2);
    }

    #[test]
    fn test_pipes_cheaper() {
        // One well + pipe is cheaper than two wells
        let wells = vec![5, 5];
        let pipes = vec![vec![1, 2, 1]];
        assert_eq!(Solution::min_cost_to_supply_water(2, wells, pipes), 6);
    }

    #[test]
    fn test_single_house() {
        let wells = vec![10];
        let pipes: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::min_cost_to_supply_water(1, wells, pipes), 10);
    }
}
