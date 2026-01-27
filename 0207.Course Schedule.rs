use std::collections::VecDeque;

impl Solution {
    /// Determines if all courses can be finished using topological sort (Kahn's algorithm).
    ///
    /// # Intuition
    /// Course prerequisites form a directed graph. If the graph has a cycle,
    /// not all courses can be completed. Topological sort detects cycles.
    ///
    /// # Approach
    /// 1. Build an adjacency list and in-degree array.
    /// 2. Enqueue all nodes with zero in-degree.
    /// 3. Process the queue: for each node, decrement neighbors' in-degrees.
    /// 4. If all nodes are processed, no cycle exists.
    ///
    /// # Complexity
    /// - Time: O(V + E) where V is courses and E is prerequisites
    /// - Space: O(V + E) for adjacency list and in-degree array
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph = vec![Vec::new(); n];
        let mut in_degree = vec![0i32; n];

        for edge in &prerequisites {
            let (course, prereq) = (edge[0] as usize, edge[1] as usize);
            graph[prereq].push(course);
            in_degree[course] += 1;
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut processed = 0;

        while let Some(node) = queue.pop_front() {
            processed += 1;
            for &neighbor in &graph[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        processed == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_finish_linear() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn cannot_finish_cycle() {
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn no_prerequisites() {
        assert!(Solution::can_finish(3, vec![]));
    }
}
