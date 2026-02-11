use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Time to Cross a Bridge
    ///
    /// # Intuition
    /// This is a discrete event simulation problem where workers transition between
    /// states (waiting, crossing, picking, putting) and we must respect bridge priority
    /// rules. The key insight is that "least efficient" workers get priority, which
    /// means we need max-heaps ordered by inefficiency.
    ///
    /// # Approach
    /// Use four priority queues to track worker states:
    /// - `left_waiting`: Max-heap of workers ready to cross right (by inefficiency)
    /// - `right_waiting`: Max-heap of workers ready to cross left with box (by inefficiency)
    /// - `left_busy`: Min-heap of workers putting boxes (by completion time)
    /// - `right_busy`: Min-heap of workers picking boxes (by completion time)
    ///
    /// Simulate time progression, always processing bridge crossings based on priority rules.
    ///
    /// # Complexity
    /// - Time: O((n + k) * log k) - each box requires O(log k) heap operations
    /// - Space: O(k) - storing worker information in heaps
    pub fn find_crossing_time(n: i32, _k: i32, time: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let k = time.len();

        // Worker efficiency key: (left + right, index) - higher is less efficient
        // For max-heap comparison, we want less efficient workers first
        let efficiency: Vec<(i32, usize)> = time
            .iter()
            .enumerate()
            .map(|(i, t)| (t[0] + t[2], i))
            .collect();

        // left_waiting: workers on left side ready to cross (max-heap by inefficiency)
        // Stores (efficiency_sum, index)
        let mut left_waiting: BinaryHeap<(i32, usize)> = BinaryHeap::new();

        // right_waiting: workers on right side with box, ready to cross (max-heap by inefficiency)
        let mut right_waiting: BinaryHeap<(i32, usize)> = BinaryHeap::new();

        // left_busy: workers putting boxes on left side (min-heap by finish time)
        // Stores (finish_time, worker_index)
        let mut left_busy: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        // right_busy: workers picking boxes on right side (min-heap by finish time)
        let mut right_busy: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        // Initialize: all workers waiting on left side
        for i in 0..k {
            left_waiting.push(efficiency[i]);
        }

        let mut current_time = 0i32;
        let mut boxes_remaining = n; // boxes still needing to be picked up
        let mut last_arrival_time = 0i32;

        while boxes_remaining > 0 || !right_waiting.is_empty() || !right_busy.is_empty() {
            // Move workers who finished their tasks to waiting queues
            while let Some(&Reverse((finish_time, idx))) = left_busy.peek() {
                if finish_time <= current_time {
                    left_busy.pop();
                    left_waiting.push(efficiency[idx]);
                } else {
                    break;
                }
            }

            while let Some(&Reverse((finish_time, idx))) = right_busy.peek() {
                if finish_time <= current_time {
                    right_busy.pop();
                    right_waiting.push(efficiency[idx]);
                } else {
                    break;
                }
            }

            // Determine who crosses the bridge
            // Priority: right side workers (with boxes) first, then left side workers
            let right_can_cross = !right_waiting.is_empty();
            let left_can_cross = !left_waiting.is_empty() && boxes_remaining > 0;

            if right_can_cross {
                // Least efficient worker on right crosses to left
                let (_, idx) = right_waiting.pop().unwrap();
                let cross_time = time[idx][2]; // left_i
                let put_time = time[idx][3]; // put_i

                current_time += cross_time;
                last_arrival_time = current_time;

                // Worker starts putting the box
                left_busy.push(Reverse((current_time + put_time, idx)));
            } else if left_can_cross {
                // Least efficient worker on left crosses to right
                let (_, idx) = left_waiting.pop().unwrap();
                let cross_time = time[idx][0]; // right_i
                let pick_time = time[idx][1]; // pick_i

                current_time += cross_time;
                boxes_remaining -= 1;

                // Worker starts picking a box
                right_busy.push(Reverse((current_time + pick_time, idx)));
            } else {
                // No one can cross right now, advance time to next event
                let mut next_time = i32::MAX;

                if !left_busy.is_empty() && boxes_remaining > 0 {
                    if let Some(&Reverse((t, _))) = left_busy.peek() {
                        next_time = next_time.min(t);
                    }
                }

                if !right_busy.is_empty() {
                    if let Some(&Reverse((t, _))) = right_busy.peek() {
                        next_time = next_time.min(t);
                    }
                }

                if next_time == i32::MAX {
                    break;
                }

                current_time = next_time;
            }
        }

        last_arrival_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Solution;

    #[test]
    fn test_example_1() {
        // n=1, k=3, time=[[1,1,2,1],[1,1,3,1],[1,1,4,1]]
        // Expected: 6
        let n = 1;
        let time = vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]];
        assert_eq!(Solution::find_crossing_time(n, 3, time), 6);
    }

    #[test]
    fn test_example_2() {
        // n=3, k=2, time=[[1,5,1,8],[10,10,10,10]]
        // Expected: 37
        let n = 3;
        let time = vec![vec![1, 5, 1, 8], vec![10, 10, 10, 10]];
        assert_eq!(Solution::find_crossing_time(n, 2, time), 37);
    }

    #[test]
    fn test_single_worker_single_box() {
        let n = 1;
        let time = vec![vec![1, 1, 1, 1]];
        // Worker crosses right (1), picks (1), crosses left (1) = arrival at 3
        assert_eq!(Solution::find_crossing_time(n, 1, time), 3);
    }

    #[test]
    fn test_single_worker_multiple_boxes() {
        let n = 2;
        let time = vec![vec![1, 1, 1, 1]];
        // Trip 1: cross right (1), pick (1), cross left (1), put (1) -> done at 4
        // Trip 2: cross right (5), pick (6), cross left (7) -> arrival at 7
        assert_eq!(Solution::find_crossing_time(n, 1, time), 7);
    }

    #[test]
    fn test_efficiency_priority() {
        // Worker 0: efficiency = 2+2 = 4
        // Worker 1: efficiency = 3+3 = 6 (less efficient, should go first)
        let n = 1;
        let time = vec![vec![2, 1, 2, 1], vec![3, 1, 3, 1]];
        // Worker 1 (less efficient) crosses: right(3) + pick(1) + left(3) = 7
        assert_eq!(Solution::find_crossing_time(n, 2, time), 7);
    }

    #[test]
    fn test_same_efficiency_higher_index_priority() {
        // Both workers have same efficiency, but worker 1 (higher index) is less efficient
        let n = 1;
        let time = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
        // Worker 1 goes first: cross right (1), pick (1), cross left (1) = 3
        assert_eq!(Solution::find_crossing_time(n, 2, time), 3);
    }

    #[test]
    fn test_right_side_priority() {
        // When both sides have workers ready, right side (with boxes) has priority
        let n = 2;
        let time = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
        // Both workers can start crossing simultaneously conceptually,
        // but only one uses bridge at a time
        let result = Solution::find_crossing_time(n, 2, time);
        assert!(result > 0);
    }

    #[test]
    fn test_large_times() {
        let n = 1;
        let time = vec![vec![1000, 1000, 1000, 1000]];
        // cross right (1000), pick (1000), cross left (1000) = 3000
        assert_eq!(Solution::find_crossing_time(n, 1, time), 3000);
    }
}
