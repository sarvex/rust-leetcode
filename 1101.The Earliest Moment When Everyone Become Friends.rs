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
    /// Finds the earliest time when all people become friends using Union-Find.
    ///
    /// # Intuition
    /// Process friendships in chronological order. When all people are in one
    /// connected component, return the timestamp.
    ///
    /// # Approach
    /// Sort logs by timestamp. Union-Find merges friends. After each union,
    /// decrement the component count. Return the timestamp when count reaches 1.
    ///
    /// # Complexity
    /// - Time: O(m log m + m * alpha(n)) where m is log count
    /// - Space: O(n)
    pub fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
        logs.sort_unstable_by_key(|log| log[0]);
        let mut uf = UnionFind::new(n as usize);
        let mut components = n;
        for log in &logs {
            if uf.union(log[1] as usize, log[2] as usize) {
                components -= 1;
                if components == 1 {
                    return log[0];
                }
            }
        }
        -1
    }
}
