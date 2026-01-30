
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // People become friends over time, all connected at timestamp 20190301
        let logs = vec![
            vec![20190101, 0, 1],
            vec![20190104, 3, 4],
            vec![20190107, 2, 3],
            vec![20190211, 1, 5],
            vec![20190224, 2, 4],
            vec![20190301, 0, 3],
            vec![20190312, 1, 2],
            vec![20190322, 4, 5],
        ];
        assert_eq!(Solution::earliest_acq(logs, 6), 20190301);
    }

    #[test]
    fn test_two_people() {
        let logs = vec![vec![100, 0, 1]];
        assert_eq!(Solution::earliest_acq(logs, 2), 100);
    }

    #[test]
    fn test_never_connected() {
        // People 0-1 and 2-3 form separate groups, never fully connected
        let logs = vec![vec![10, 0, 1], vec![20, 2, 3]];
        assert_eq!(Solution::earliest_acq(logs, 4), -1);
    }

    #[test]
    fn test_immediate_connection() {
        // All connections happen at the same timestamp
        let logs = vec![vec![1, 0, 1], vec![1, 1, 2]];
        assert_eq!(Solution::earliest_acq(logs, 3), 1);
    }

    #[test]
    fn test_single_person() {
        // Single person is trivially connected
        let logs: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::earliest_acq(logs, 1), -1);
    }
}
