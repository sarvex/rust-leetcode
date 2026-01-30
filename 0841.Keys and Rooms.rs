
impl Solution {
    /// Determines if all rooms can be visited starting from room 0.
    ///
    /// # Intuition
    /// This is a graph reachability problem from node 0. DFS/BFS from room 0
    /// using collected keys to unlock new rooms.
    ///
    /// # Approach
    /// Use a stack-based DFS. Mark rooms as visited. Push newly found keys
    /// onto the stack. Check if all rooms are visited at the end.
    ///
    /// # Complexity
    /// - Time: O(n + k) where k is total number of keys
    /// - Space: O(n)
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = vec![false; n];
        let mut stack = Vec::with_capacity(n);
        stack.push(0usize);

        while let Some(room) = stack.pop() {
            if visited[room] {
                continue;
            }
            visited[room] = true;
            stack.extend(rooms[room].iter().map(|&k| k as usize));
        }

        visited.iter().all(|&v| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_reachable() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![],
        ]));
    }

    #[test]
    fn test_not_all_reachable() {
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0],
        ]));
    }

    #[test]
    fn test_single_room() {
        assert!(Solution::can_visit_all_rooms(vec![vec![]]));
    }
}
