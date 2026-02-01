use std::collections::VecDeque;

impl Solution {
    /// Rebuild a grid by taking a boundary path and expanding inward.
    ///
    /// # Intuition
    /// Grid graphs have predictable degrees: corners have degree 2, boundary nodes degree 3,
    /// and interior nodes degree 4. A shortest path between adjacent corners lies on the
    /// boundary and yields one full row or column of the grid.
    ///
    /// # Approach
    /// - Build adjacency lists for the graph.
    /// - If any node has degree 1, the graph is a path; return the node order as a single row.
    /// - Otherwise, pick a corner, BFS to find the closest other corner, and reconstruct the
    ///   shortest path between them as the first row (length `cols`).
    /// - Set `rows = n / cols`. For each next row, for each column, take the neighbor of the
    ///   node above that is not its left/right (and not the node above that) to step inward.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m)
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let node_count = n as usize;
        let mut adjacency = vec![Vec::<i32>::new(); node_count];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency[u].push(v as i32);
            adjacency[v].push(u as i32);
        }

        let degree_one: Vec<usize> = (0..node_count).filter(|&i| adjacency[i].len() == 1).collect();
        if !degree_one.is_empty() {
            let start = degree_one[0];
            let mut path = Vec::with_capacity(node_count);
            let mut prev = usize::MAX;
            let mut current = start;
            while path.len() < node_count {
                path.push(current as i32);
                let next = adjacency[current]
                    .iter()
                    .map(|&x| x as usize)
                    .find(|&x| x != prev);
                match next {
                    Some(nxt) => {
                        prev = current;
                        current = nxt;
                    }
                    None => break,
                }
            }
            return vec![path];
        }

        let corners: Vec<usize> = (0..node_count).filter(|&i| adjacency[i].len() == 2).collect();
        let start = corners[0];

        let mut dist = vec![-1_i32; node_count];
        let mut parent = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        dist[start] = 0;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            let current_dist = dist[node];
            for &neighbor_i32 in &adjacency[node] {
                let neighbor = neighbor_i32 as usize;
                if dist[neighbor] == -1 {
                    dist[neighbor] = current_dist + 1;
                    parent[neighbor] = node;
                    queue.push_back(neighbor);
                }
            }
        }

        let mut closest_corner = start;
        let mut min_dist = i32::MAX;
        for &corner in &corners {
            if corner == start {
                continue;
            }
            let d = dist[corner];
            if d < min_dist {
                min_dist = d;
                closest_corner = corner;
            }
        }

        let cols = (min_dist + 1) as usize;
        let rows = node_count / cols;

        let mut path_rev = Vec::with_capacity(cols);
        let mut node = closest_corner;
        while node != start {
            path_rev.push(node);
            node = parent[node];
        }
        path_rev.push(start);
        path_rev.reverse();

        let first_row: Vec<i32> = path_rev.iter().map(|&x| x as i32).collect();
        let mut grid = Vec::with_capacity(rows);
        grid.push(first_row);

        for r in 1..rows {
            let mut row = Vec::with_capacity(cols);
            for c in 0..cols {
                let above = grid[r - 1][c] as usize;
                let mut banned = [usize::MAX; 3];
                let mut banned_count = 0;
                if r >= 2 {
                    banned[banned_count] = grid[r - 2][c] as usize;
                    banned_count += 1;
                }
                if c >= 1 {
                    banned[banned_count] = grid[r - 1][c - 1] as usize;
                    banned_count += 1;
                }
                if c + 1 < cols {
                    banned[banned_count] = grid[r - 1][c + 1] as usize;
                    banned_count += 1;
                }

                let next = adjacency[above]
                    .iter()
                    .map(|&x| x as usize)
                    .find(|&x| !banned[..banned_count].contains(&x))
                    .unwrap();
                row.push(next as i32);
            }
            grid.push(row);
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn is_valid_grid(n: i32, edges: &[Vec<i32>], grid: &[Vec<i32>]) -> bool {
        if grid.is_empty() {
            return false;
        }
        let cols = grid[0].len();
        if cols == 0 || !grid.iter().all(|row| row.len() == cols) {
            return false;
        }

        let node_count = n as usize;
        let mut seen = vec![false; node_count];
        for &value in grid.iter().flatten() {
            if value < 0 || value >= n {
                return false;
            }
            let idx = value as usize;
            if seen[idx] {
                return false;
            }
            seen[idx] = true;
        }
        if seen.iter().any(|&v| !v) {
            return false;
        }

        let mut grid_edges: HashSet<(i32, i32)> = HashSet::new();
        for r in 0..grid.len() {
            for c in 0..cols {
                let node = grid[r][c];
                if r + 1 < grid.len() {
                    let other = grid[r + 1][c];
                    let pair = if node < other { (node, other) } else { (other, node) };
                    grid_edges.insert(pair);
                }
                if c + 1 < cols {
                    let other = grid[r][c + 1];
                    let pair = if node < other { (node, other) } else { (other, node) };
                    grid_edges.insert(pair);
                }
            }
        }

        if grid_edges.len() != edges.len() {
            return false;
        }
        edges.iter().all(|edge| {
            let u = edge[0];
            let v = edge[1];
            let pair = if u < v { (u, v) } else { (v, u) };
            grid_edges.contains(&pair)
        })
    }

    #[test]
    fn example_one() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        let grid = Solution::construct_grid_layout(n, edges.clone());
        assert!(is_valid_grid(n, &edges, &grid));
    }

    #[test]
    fn example_two() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![2, 4]];
        let grid = Solution::construct_grid_layout(n, edges.clone());
        assert!(is_valid_grid(n, &edges, &grid));
    }

    #[test]
    fn example_three() {
        let n = 9;
        let edges = vec![
            vec![0, 1],
            vec![0, 4],
            vec![0, 5],
            vec![1, 7],
            vec![2, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![4, 6],
            vec![4, 7],
            vec![6, 8],
            vec![7, 8],
        ];
        let grid = Solution::construct_grid_layout(n, edges.clone());
        assert!(is_valid_grid(n, &edges, &grid));
    }

    #[test]
    fn two_by_two_grid() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        let grid = Solution::construct_grid_layout(n, edges.clone());
        assert!(is_valid_grid(n, &edges, &grid));
    }
}
