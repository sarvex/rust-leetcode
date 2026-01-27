struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
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
