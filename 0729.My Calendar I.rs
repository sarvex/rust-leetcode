use std::collections::BTreeMap;


/// Calendar that prevents double-booking using a balanced BST.
///
/// # Intuition
/// Store intervals keyed by end time. For a new booking, check if any
/// existing interval overlaps by querying the tree for the nearest
/// entry after the start time.
///
/// # Approach
/// Use a `BTreeMap<end, start>`. For a new `[start, end)`, look at the
/// first entry with key > start. If that entry's value (start of existing
/// interval) is less than the new end, there is an overlap.
///
/// # Complexity
/// - Time: O(log n) per booking
/// - Space: O(n) for stored intervals
struct MyCalendar {
    intervals: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some((&_end, &existing_start)) = self.intervals.range(start_time + 1..).next() {
            if existing_start < end_time {
                return false;
            }
        }
        self.intervals.insert(end_time, start_time);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_booking() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(!calendar.book(15, 25)); // Overlaps with [10, 20)
        assert!(calendar.book(20, 30)); // No overlap, starts at end of first
    }

    #[test]
    fn test_no_overlap() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(calendar.book(20, 30));
        assert!(calendar.book(30, 40));
    }

    #[test]
    fn test_complete_overlap() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(!calendar.book(5, 25)); // Completely overlaps
    }

    #[test]
    fn test_contained_interval() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 30));
        assert!(!calendar.book(15, 20)); // Contained within [10, 30)
    }

    #[test]
    fn test_adjacent_bookings() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(0, 10));
        assert!(calendar.book(10, 20));
        assert!(calendar.book(20, 30));
        assert!(!calendar.book(5, 15)); // Overlaps first and second
    }

    #[test]
    fn test_single_time_unit() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(1, 2));
        assert!(calendar.book(2, 3));
        assert!(!calendar.book(1, 2)); // Same interval again
    }

    #[test]
    fn test_booking_before_existing() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(20, 30));
        assert!(calendar.book(10, 20));
        assert!(calendar.book(0, 10));
    }
}
