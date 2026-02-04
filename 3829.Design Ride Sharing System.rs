/// FIFO queue with O(1) remove-by-id using fixed arrays (rider_id in 1..=1000).
struct RiderQueue {
    head: usize, // 0 means none
    tail: usize, // 0 means none
    prev: Vec<usize>,
    next: Vec<usize>,
    in_queue: Vec<bool>,
}

impl RiderQueue {
    const MAX_RIDER_ID: usize = 1000;

    fn new() -> Self {
        let size = Self::MAX_RIDER_ID + 1;
        Self {
            head: 0,
            tail: 0,
            prev: vec![0; size],
            next: vec![0; size],
            in_queue: vec![false; size],
        }
    }

    fn push_back(&mut self, rider_id: i32) {
        let id = rider_id as usize;
        self.in_queue[id] = true;
        self.prev[id] = self.tail;
        self.next[id] = 0;
        if self.tail != 0 {
            self.next[self.tail] = id;
        } else {
            self.head = id;
        }
        self.tail = id;
    }

    fn pop_front(&mut self) -> Option<i32> {
        let id = self.head;
        if id == 0 {
            return None;
        }
        let next_id = self.next[id];
        self.head = next_id;
        if next_id != 0 {
            self.prev[next_id] = 0;
        } else {
            self.tail = 0;
        }
        self.in_queue[id] = false;
        self.prev[id] = 0;
        self.next[id] = 0;
        Some(id as i32)
    }

    fn remove(&mut self, rider_id: i32) -> bool {
        let id = rider_id as usize;
        if id == 0 || id > Self::MAX_RIDER_ID || !self.in_queue[id] {
            return false;
        }
        let prev_id = self.prev[id];
        let next_id = self.next[id];
        if prev_id != 0 {
            self.next[prev_id] = next_id;
        } else {
            self.head = next_id;
        }
        if next_id != 0 {
            self.prev[next_id] = prev_id;
        } else {
            self.tail = prev_id;
        }
        self.in_queue[id] = false;
        self.prev[id] = 0;
        self.next[id] = 0;
        true
    }

    fn is_empty(&self) -> bool {
        self.head == 0
    }
}

/// Fixed-size ring buffer for drivers (driver_id in 1..=1000).
struct DriverQueue {
    data: Vec<i32>,
    head: usize,
    size: usize,
    capacity: usize,
}

impl DriverQueue {
    const MAX_DRIVERS: usize = 1000;

    fn new() -> Self {
        let capacity = Self::MAX_DRIVERS + 1;
        Self {
            data: vec![0; capacity],
            head: 0,
            size: 0,
            capacity,
        }
    }

    fn push_back(&mut self, driver_id: i32) {
        debug_assert!(self.size < self.capacity);
        let idx = (self.head + self.size) % self.capacity;
        self.data[idx] = driver_id;
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let driver_id = self.data[self.head];
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        Some(driver_id)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// Ride-sharing system that matches riders and drivers in FIFO order.
///
/// # Intuition
/// Riders and drivers join queues; cancel removes the rider from the queue in
/// O(1). Match always sees the true front with no draining.
///
/// # Approach
/// - Rider queue uses fixed arrays keyed by rider_id to support O(1) remove.
/// - Drivers use a fixed-size ring buffer (no heap growth).
///
/// # Complexity
/// - Time: O(1) for add_rider, add_driver, match_driver_with_rider, cancel_rider.
/// - Space: O(n + m).
struct RideSharingSystem {
    riders: RiderQueue,
    drivers: DriverQueue,
}

impl RideSharingSystem {
    fn new() -> Self {
        Self {
            riders: RiderQueue::new(),
            drivers: DriverQueue::new(),
        }
    }

    fn add_rider(&mut self, rider_id: i32) {
        self.riders.push_back(rider_id);
    }

    fn add_driver(&mut self, driver_id: i32) {
        self.drivers.push_back(driver_id);
    }

    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        if self.riders.is_empty() || self.drivers.is_empty() {
            return vec![-1, -1];
        }
        let driver_id = self.drivers.pop_front().unwrap();
        let rider_id = self.riders.pop_front().unwrap();
        vec![driver_id, rider_id]
    }

    fn cancel_rider(&mut self, rider_id: i32) {
        self.riders.remove(rider_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut sys = RideSharingSystem::new();
        sys.add_rider(3);
        sys.add_driver(2);
        sys.add_rider(1);
        assert_eq!(sys.match_driver_with_rider(), vec![2, 3]);
        sys.add_driver(5);
        sys.cancel_rider(3); // already matched, no effect
        assert_eq!(sys.match_driver_with_rider(), vec![5, 1]);
        assert_eq!(sys.match_driver_with_rider(), vec![-1, -1]);
    }

    #[test]
    fn test_example_2() {
        let mut sys = RideSharingSystem::new();
        sys.add_rider(8);
        sys.add_driver(8);
        sys.add_driver(6);
        assert_eq!(sys.match_driver_with_rider(), vec![8, 8]);
        sys.add_rider(2);
        sys.cancel_rider(2);
        assert_eq!(sys.match_driver_with_rider(), vec![-1, -1]);
    }

    #[test]
    fn test_no_drivers_returns_minus_one() {
        let mut sys = RideSharingSystem::new();
        sys.add_rider(1);
        assert_eq!(sys.match_driver_with_rider(), vec![-1, -1]);
    }

    #[test]
    fn test_no_riders_returns_minus_one() {
        let mut sys = RideSharingSystem::new();
        sys.add_driver(1);
        assert_eq!(sys.match_driver_with_rider(), vec![-1, -1]);
    }
}
