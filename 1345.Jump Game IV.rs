use std::collections::VecDeque;

impl Solution {
    /// BFS with binary-search teleportation — no HashMap, no hashing overhead.
    ///
    /// # Intuition
    /// The HashMap used in the standard approach pays a hashing cost on every
    /// teleport-group lookup. Since we only need to find and drain each group
    /// once, we can replace the map with a value-sorted index array and locate
    /// groups in O(log n) via binary search. The total cost of all group
    /// iterations is still O(n) because each index is consumed exactly once.
    ///
    /// # Approach
    /// 1. Build `sorted_by_val: Vec<usize>` — indices 0..n sorted by `arr[i]`.
    /// 2. BFS from index 0 with a `VecDeque`. For each dequeued `curr`:
    ///    - Return `steps` immediately if `curr == n - 1`.
    ///    - Enqueue unvisited `curr±1`.
    ///    - Binary-search `sorted_by_val` for the contiguous run sharing
    ///      `arr[curr]`. Enqueue every unvisited member, then zero out the run
    ///      (set each slot to `usize::MAX`) so it is never re-visited.
    /// 3. Increment `steps` once per BFS level after the full frontier.
    ///
    /// Zeroing the run is the drain-once guarantee: subsequent nodes with the
    /// same value find an empty (sentinel-filled) run and skip it in O(1).
    ///
    /// # Complexity
    /// - Time: O(n log n) — sort dominates; BFS is O(n) with O(log n) per
    ///   group lookup, but each index is processed at most once so total BFS
    ///   work is O(n log n).
    /// - Space: O(n) — sorted index array, visited array, queue.
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 {
            return 0;
        }

        // Sort indices by their value so same-value indices form contiguous runs.
        let mut sorted_by_val: Vec<usize> = (0..n).collect();
        sorted_by_val.sort_unstable_by_key(|&i| arr[i]);

        // For each index, record where it sits in sorted_by_val so we can find
        // its group's start in O(log n).
        // We binary-search by value, so we only need the sorted array itself.

        let mut visited = vec![false; n];
        visited[0] = true;

        let mut queue = VecDeque::with_capacity(n);
        queue.push_back(0usize);
        let mut steps = 0i32;

        while !queue.is_empty() {
            let frontier = queue.len();
            for _ in 0..frontier {
                let curr = queue.pop_front().unwrap();

                if curr == n - 1 {
                    return steps;
                }

                // ±1 neighbours.
                if curr + 1 < n && !visited[curr + 1] {
                    visited[curr + 1] = true;
                    queue.push_back(curr + 1);
                }
                if curr >= 1 && !visited[curr - 1] {
                    visited[curr - 1] = true;
                    queue.push_back(curr - 1);
                }

                // Teleport group: find the contiguous run for arr[curr].
                let val = arr[curr];
                // Left boundary: first position where arr[sorted[pos]] >= val.
                let lo = sorted_by_val.partition_point(|&i| {
                    if i == usize::MAX {
                        true
                    } else {
                        arr[i] < val
                    }
                });
                // Iterate the run while values match and slots are not consumed.
                let mut pos = lo;
                while pos < n {
                    let idx = sorted_by_val[pos];
                    if idx == usize::MAX {
                        pos += 1;
                        continue;
                    }
                    if arr[idx] != val {
                        break;
                    }
                    // Consume this slot.
                    sorted_by_val[pos] = usize::MAX;
                    if !visited[idx] {
                        visited[idx] = true;
                        queue.push_back(idx);
                    }
                    pos += 1;
                }
            }
            steps += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_jumps() {
        // 0 -> 4 -> 3 -> 9
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::min_jumps(vec![7]), 0);
    }

    #[test]
    fn one_teleport_jump() {
        // 0 -> 7 via same value
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }

    #[test]
    fn two_adjacent_elements() {
        assert_eq!(Solution::min_jumps(vec![1, 2]), 1);
    }

    #[test]
    fn all_same_value() {
        // From index 0 teleport directly to last index in one step.
        assert_eq!(Solution::min_jumps(vec![5, 5, 5, 5, 5]), 1);
    }

    #[test]
    fn sequential_no_teleport() {
        // All distinct: must walk step by step.
        assert_eq!(Solution::min_jumps(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn large_repeated_value_performance() {
        // 50 000 identical values — teleport group cleared in one BFS step.
        let mut arr = vec![7i32; 50_000];
        arr[0] = 11; // force at least one ±1 step before teleport
        assert!(Solution::min_jumps(arr) <= 2);
    }

    #[test]
    fn alternating_values() {
        // [0,1,0,1,...,0,1]: index 0 has value 0, last index 9 has value 1.
        // Step 1: teleport from 0 to index 8, or step to index 1.
        // Step 2: from index 8 step to 9, or from index 1 teleport to index 9.
        let arr: Vec<i32> = (0..10).map(|i| i % 2).collect();
        assert_eq!(Solution::min_jumps(arr), 2);
    }

    #[test]
    fn worst_case_all_distinct() {
        let arr: Vec<i32> = (0..50_000).collect();
        assert_eq!(Solution::min_jumps(arr), 49_999);
    }
}
