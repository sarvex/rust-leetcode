struct HitCounter {
    timestamps: Vec<i32>,
}

impl HitCounter {
    /// Hit counter that counts hits in the last 300 seconds using binary search.
    ///
    /// # Intuition
    /// Store all timestamps sorted. Use binary search to find the first timestamp
    /// within the 300-second window.
    ///
    /// # Approach
    /// 1. Append timestamps to a vector (already sorted by arrival order).
    /// 2. On getHits, binary search for the first timestamp >= `timestamp - 299`.
    /// 3. The count is `total - index`.
    ///
    /// # Complexity
    /// - Time: O(1) hit, O(log n) getHits
    /// - Space: O(n)
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
        }
    }

    fn hit(&mut self, timestamp: i32) {
        self.timestamps.push(timestamp);
    }

    fn get_hits(&self, timestamp: i32) -> i32 {
        let target = timestamp - 299;
        let idx = self.timestamps.partition_point(|&t| t < target);
        (self.timestamps.len() - idx) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(2);
        counter.hit(3);
        assert_eq!(counter.get_hits(4), 3);
        counter.hit(300);
        assert_eq!(counter.get_hits(300), 4);
        assert_eq!(counter.get_hits(301), 3);
    }

    #[test]
    fn no_hits() {
        let counter = HitCounter::new();
        assert_eq!(counter.get_hits(1), 0);
    }

    #[test]
    fn hits_expire() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        assert_eq!(counter.get_hits(300), 1);
        assert_eq!(counter.get_hits(301), 0);
    }

    #[test]
    fn multiple_hits_same_timestamp() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(1);
        counter.hit(1);
        assert_eq!(counter.get_hits(1), 3);
        assert_eq!(counter.get_hits(300), 3);
        assert_eq!(counter.get_hits(301), 0);
    }

    #[test]
    fn hits_at_boundary() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(299);
        counter.hit(300);
        assert_eq!(counter.get_hits(300), 3);
        assert_eq!(counter.get_hits(301), 2);
        assert_eq!(counter.get_hits(599), 1);
        assert_eq!(counter.get_hits(600), 0);
    }

    #[test]
    fn large_gap_between_hits() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(1000);
        assert_eq!(counter.get_hits(1000), 1);
        assert_eq!(counter.get_hits(1299), 1);
        assert_eq!(counter.get_hits(1300), 0);
    }
}
