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
            queue: VecDeque::with_capacity(3001), // Maximum window size
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_recent_counter() {
        let mut counter = RecentCounter::new();

        assert_eq!(counter.ping(1), 1); // [1]
        assert_eq!(counter.ping(100), 2); // [1, 100]
        assert_eq!(counter.ping(3001), 3); // [1, 100, 3001]
        assert_eq!(counter.ping(3002), 3); // [100, 3001, 3002] (1 is removed as 1 < 3002-3000=2)
    }

    #[test]
    fn test_consecutive_pings() {
        let mut counter = RecentCounter::new();

        // All within 3000ms window
        assert_eq!(counter.ping(1), 1);
        assert_eq!(counter.ping(500), 2);
        assert_eq!(counter.ping(1000), 3);
        assert_eq!(counter.ping(1500), 4);
        assert_eq!(counter.ping(2000), 5);
        assert_eq!(counter.ping(2500), 6);
        assert_eq!(counter.ping(3000), 7);

        // Now at t=3001, ping(1) at t=1 should be kept (1 >= 3001-3000=1)
        assert_eq!(counter.ping(3001), 8);

        // At t=3002, ping at t=1 should be removed (1 < 3002-3000=2)
        assert_eq!(counter.ping(3002), 8);

        // At t=3500, pings before t=500 should be removed
        // Remaining: [500, 1000, 1500, 2000, 2500, 3000, 3001, 3002, 3500]
        assert_eq!(counter.ping(3500), 9);
    }

    #[test]
    fn test_large_gap_between_pings() {
        let mut counter = RecentCounter::new();

        assert_eq!(counter.ping(1), 1);
        assert_eq!(counter.ping(2), 2);

        // Large gap - all previous pings should be removed
        assert_eq!(counter.ping(10000), 1);
        assert_eq!(counter.ping(10001), 2);

        // Another large gap
        assert_eq!(counter.ping(20000), 1);
    }

    #[test]
    fn test_exact_3000ms_window() {
        let mut counter = RecentCounter::new();

        assert_eq!(counter.ping(1000), 1);
        assert_eq!(counter.ping(2000), 2);
        assert_eq!(counter.ping(3000), 3);
        assert_eq!(counter.ping(4000), 4);

        // At t=4000, ping at 1000 should still be included (1000 >= 4000 - 3000 = 1000)
        // So we have pings at [1000, 2000, 3000, 4000]

        // At t=4001, ping at 1000 should be removed (1000 < 4001 - 3000 = 1001)
        assert_eq!(counter.ping(4001), 4); // [2000, 3000, 4000, 4001]
    }

    #[test]
    fn test_rapid_fire_pings() {
        let mut counter = RecentCounter::new();

        // Simulate 100 pings in quick succession
        for i in 1..=100 {
            let count = counter.ping(i);
            assert_eq!(count, i);
        }

        // At t=3001, pings before t=1 should be removed (none)
        // All 100 pings plus the new one = 101
        assert_eq!(counter.ping(3001), 101);

        // At t=3002, ping at t=1 should be removed
        assert_eq!(counter.ping(3002), 101);

        // Jump to 5000ms - only pings from 2000 onwards remain
        // All pings < 2000 are removed, so we lose pings 1-1999
        // Only ping at 5000 remains since all others are < 2000
        assert_eq!(counter.ping(5000), 3);
    }

    #[test]
    fn test_minimum_timestamp() {
        let mut counter = RecentCounter::new();

        // Test with minimum valid timestamp
        assert_eq!(counter.ping(1), 1);
        assert_eq!(counter.ping(2), 2);
        assert_eq!(counter.ping(3000), 3);
        // At t=3001, ping at t=1 is kept (1 >= 3001-3000=1)
        assert_eq!(counter.ping(3001), 4);
        // At t=3002, ping at t=1 is removed (1 < 3002-3000=2)
        assert_eq!(counter.ping(3002), 4);
    }

    #[test]
    fn test_maximum_window_size() {
        let mut counter = RecentCounter::new();

        // Fill the window with maximum possible pings (3001)
        for i in 0..=3000 {
            counter.ping(i);
        }

        // All 3001 pings should be in the window
        assert_eq!(counter.queue.len(), 3001);

        // Next ping at 3001: removes ping at 0 (0 < 3001-3000=1)
        // Keeps pings from 1 to 3000, plus new ping at 3001
        assert_eq!(counter.ping(3001), 3001);
    }

    #[test]
    fn test_window_boundary() {
        let mut counter = RecentCounter::new();

        // Test exact boundary conditions
        assert_eq!(counter.ping(1), 1);
        assert_eq!(counter.ping(3001), 2); // 1 is kept (1 >= 3001-3000=1)

        counter = RecentCounter::new();
        assert_eq!(counter.ping(0), 1);
        assert_eq!(counter.ping(3000), 2); // 0 is kept (0 >= 3000-3000=0)
        assert_eq!(counter.ping(3001), 2); // 0 is removed (0 < 3001-3000=1)
    }
}
