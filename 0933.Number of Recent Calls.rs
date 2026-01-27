use std::collections::VecDeque;

/// Queue-based recent call counter within a 3000ms window.
///
/// # Intuition
/// Keep a queue of timestamps. On each ping, remove timestamps older than
/// `t - 3000` and return the queue length.
///
/// # Approach
/// Push new timestamp, pop from front while front < t - 3000.
///
/// # Complexity
/// - Time: O(1) amortized per ping
/// - Space: O(W) where W is the window size (at most 3001 elements)
struct RecentCounter {
    queue: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while self.queue.front().is_some_and(|&v| v < t - 3000) {
            self.queue.pop_front();
        }
        self.queue.len() as i32
    }
}
