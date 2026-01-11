impl Solution {
    /// Minimum Weighted Subgraph With the Required Paths
    ///
    /// # Intuition
    /// The optimal subgraph has paths from src1 and src2 meeting at some node m,
    /// then sharing a path from m to dest, minimizing total weight.
    ///
    /// # Approach
    /// 1. Build CSR (Compressed Sparse Row) format graphs for cache efficiency
    /// 2. Run Dijkstra from src1, src2 on forward graph, and from dest on reverse graph
    /// 3. Find minimum of dist(src1,m) + dist(src2,m) + dist(m,dest) over all nodes m
    ///
    /// # Complexity
    /// - Time: O(E log V)
    /// - Space: O(V + E)
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = n as usize;
        let m = edges.len();

        // Count degrees for CSR offset computation
        let mut fwd_deg = vec![0u32; n + 1];
        let mut rev_deg = vec![0u32; n + 1];

        for e in &edges {
            fwd_deg[e[0] as usize + 1] += 1;
            rev_deg[e[1] as usize + 1] += 1;
        }

        // Compute prefix sums for offsets
        for i in 1..=n {
            fwd_deg[i] += fwd_deg[i - 1];
            rev_deg[i] += rev_deg[i - 1];
        }

        // Build CSR edge arrays
        let mut fwd_edges = vec![(0u32, 0u32); m];
        let mut rev_edges = vec![(0u32, 0u32); m];
        let mut fwd_idx = fwd_deg.clone();
        let mut rev_idx = rev_deg.clone();

        for e in &edges {
            let (u, v, w) = (e[0] as usize, e[1] as u32, e[2] as u32);
            let i = fwd_idx[u] as usize;
            fwd_edges[i] = (v, w);
            fwd_idx[u] += 1;

            let j = rev_idx[v as usize] as usize;
            rev_edges[j] = (u as u32, w);
            rev_idx[v as usize] += 1;
        }

        let dijkstra = |offsets: &[u32], edges: &[(u32, u32)], start: usize| -> Vec<u64> {
            let mut dist = vec![u64::MAX; n];
            let mut heap = BinaryHeap::new();

            dist[start] = 0;
            heap.push(Reverse((0u64, start as u32)));

            while let Some(Reverse((d, u))) = heap.pop() {
                let u = u as usize;
                if d > dist[u] {
                    continue;
                }

                let lo = offsets[u] as usize;
                let hi = offsets[u + 1] as usize;

                for &(v, w) in &edges[lo..hi] {
                    let nd = d + w as u64;
                    if nd < dist[v as usize] {
                        dist[v as usize] = nd;
                        heap.push(Reverse((nd, v)));
                    }
                }
            }
            dist
        };

        let d1 = dijkstra(&fwd_deg, &fwd_edges, src1 as usize);
        let d2 = dijkstra(&fwd_deg, &fwd_edges, src2 as usize);
        let d3 = dijkstra(&rev_deg, &rev_edges, dest as usize);

        let mut ans = u64::MAX;
        for i in 0..n {
            if d1[i] < u64::MAX && d2[i] < u64::MAX && d3[i] < u64::MAX {
                ans = ans.min(d1[i] + d2[i] + d3[i]);
            }
        }

        if ans == u64::MAX {
            -1
        } else {
            ans as i64
        }
    }
}
