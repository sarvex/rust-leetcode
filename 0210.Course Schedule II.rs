use std::collections::VecDeque;

impl Solution {
    /// Returns a valid course ordering using topological sort (Kahn's algorithm).
    ///
    /// # Intuition
    /// Process courses with no remaining prerequisites first. As each course
    /// is completed, decrement the in-degree of dependent courses.
    ///
    /// # Approach
    /// 1. Build adjacency list and in-degree array from prerequisites.
    /// 2. Enqueue all courses with zero in-degree.
    /// 3. BFS: dequeue, add to result, update neighbor in-degrees.
    /// 4. Return empty if not all courses are reachable (cycle exists).
    ///
    /// # Complexity
    /// - Time: O(V + E)
    /// - Space: O(V + E)
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut graph = vec![Vec::new(); n];
        let mut in_degree = vec![0i32; n];

        for edge in &prerequisites {
            let (course, prereq) = (edge[0] as usize, edge[1] as usize);
            graph[prereq].push(course);
            in_degree[course] += 1;
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut result = Vec::with_capacity(n);

        while let Some(node) = queue.pop_front() {
            result.push(node as i32);
            for &neighbor in &graph[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if result.len() == n {
            result
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_dependency() {
        let result = Solution::find_order(2, vec![vec![1, 0]]);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn cycle_returns_empty() {
        let result = Solution::find_order(2, vec![vec![1, 0], vec![0, 1]]);
        assert!(result.is_empty());
    }

    #[test]
    fn no_prerequisites() {
        let result = Solution::find_order(3, vec![]);
        assert_eq!(result.len(), 3);
    }
}
